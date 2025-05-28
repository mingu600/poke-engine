use crate::choices::Choices;
use crate::engine::evaluate::{
    evaluate_pokemon, get_boost_multiplier, CONFUSION, LEECH_SEED, POKEMON_ATTACK_BOOST,
    POKEMON_DEFENSE_BOOST, POKEMON_SPECIAL_ATTACK_BOOST, POKEMON_SPECIAL_DEFENSE_BOOST,
    POKEMON_SPEED_BOOST, SUBSTITUTE, USED_TERA,
};
use crate::engine::state::{MoveChoice, PokemonVolatileStatus};
use crate::mcts::{sigmoid, MctsResult, MctsSideResult, Node};
use crate::state::{LastUsedMove, Side, State};

const RECOVERY: f32 = 10.0;

fn evaluate_recovery(side: &Side) -> f32 {
    match side.last_used_move {
        LastUsedMove::Move(move_idx) => {
            match side.get_active_immutable().moves[&move_idx].id {
                // 50% recovery moves
                Choices::RECOVER
                | Choices::ROOST
                | Choices::MOONLIGHT
                | Choices::MORNINGSUN
                | Choices::SYNTHESIS
                | Choices::SLACKOFF
                | Choices::MILKDRINK
                | Choices::SOFTBOILED
                | Choices::WISH
                | Choices::HEALORDER
                | Choices::REST
                | Choices::STRENGTHSAP => {
                    return RECOVERY;
                }
                _ => return 0.0,
            }
        }
        _ => return 0.0,
    }
}

/// A custom evaluation function for 1v1 matchups
pub fn evaluate_matchup(state: &State) -> f32 {
    let mut score = 0.0;

    // Get active Pokémon
    let s1_active = state.side_one.get_active_immutable();
    let s2_active = state.side_two.get_active_immutable();

    score += evaluate_pokemon(s1_active);
    score -= evaluate_recovery(&state.side_one);
    for vs in state.side_one.volatile_statuses.iter() {
        match vs {
            PokemonVolatileStatus::LEECHSEED => score += LEECH_SEED,
            PokemonVolatileStatus::SUBSTITUTE => score += SUBSTITUTE,
            PokemonVolatileStatus::CONFUSION => score += CONFUSION,
            _ => {}
        }
    }

    score += get_boost_multiplier(state.side_one.attack_boost) * POKEMON_ATTACK_BOOST;
    score += get_boost_multiplier(state.side_one.defense_boost) * POKEMON_DEFENSE_BOOST;
    score +=
        get_boost_multiplier(state.side_one.special_attack_boost) * POKEMON_SPECIAL_ATTACK_BOOST;
    score +=
        get_boost_multiplier(state.side_one.special_defense_boost) * POKEMON_SPECIAL_DEFENSE_BOOST;
    score += get_boost_multiplier(state.side_one.speed_boost) * POKEMON_SPEED_BOOST;

    // if s1_active.terastallized {
    //     score += USED_TERA;
    // }
    score -= evaluate_pokemon(s2_active);
    score += evaluate_recovery(&state.side_two);
    for vs in state.side_two.volatile_statuses.iter() {
        match vs {
            PokemonVolatileStatus::LEECHSEED => score -= LEECH_SEED,
            PokemonVolatileStatus::SUBSTITUTE => score -= SUBSTITUTE,
            PokemonVolatileStatus::CONFUSION => score -= CONFUSION,
            _ => {}
        }
    }

    score -= get_boost_multiplier(state.side_two.attack_boost) * POKEMON_ATTACK_BOOST;
    score -= get_boost_multiplier(state.side_two.defense_boost) * POKEMON_DEFENSE_BOOST;
    score -=
        get_boost_multiplier(state.side_two.special_attack_boost) * POKEMON_SPECIAL_ATTACK_BOOST;
    score -=
        get_boost_multiplier(state.side_two.special_defense_boost) * POKEMON_SPECIAL_DEFENSE_BOOST;
    score -= get_boost_multiplier(state.side_two.speed_boost) * POKEMON_SPEED_BOOST;

    // if s2_active.terastallized {
    //     score -= USED_TERA;
    // }
    score
}

/// Modified MCTS rollout that uses our custom matchup evaluation
fn do_mcts_for_matchup(root_node: &mut Node, state: &mut State, root_eval: &f32) {
    let (mut new_node, s1_move, s2_move) = unsafe { root_node.selection(state) };
    new_node = unsafe { (*new_node).expand(state, s1_move, s2_move) };

    // Use the matchup-specific evaluation
    let battle_is_over = state.battle_is_over();
    let rollout_result = if battle_is_over == 0.0 {
        let current_eval = evaluate_matchup(state);
        sigmoid(current_eval - *root_eval)
    } else {
        if battle_is_over == -1.0 {
            0.0
        } else {
            battle_is_over
        }
    };

    unsafe { (*new_node).backpropagate(rollout_result, state) }
}

/// Perform MCTS specifically for 1v1 matchup analysis
/// Skips Terastallized moves and focuses on the core 1v1 dynamics
pub fn perform_mcts_for_matchup(
    state: &mut State,
    side_one_options: Vec<MoveChoice>,
    side_two_options: Vec<MoveChoice>,
    iterations: u32,
) -> MctsResult {
    // Filter out terastallized moves
    let filtered_s1_options: Vec<MoveChoice> = side_one_options
        .into_iter()
        .filter(|choice| {
            match choice {
                MoveChoice::MoveTera(_) => false, // Skip Terastallized moves
                _ => true,
            }
        })
        .collect();

    let filtered_s2_options: Vec<MoveChoice> = side_two_options
        .into_iter()
        .filter(|choice| {
            match choice {
                MoveChoice::MoveTera(_) => false, // Skip Terastallized moves
                _ => true,
            }
        })
        .collect();

    let mut root_node = Node::new(filtered_s1_options, filtered_s2_options);
    root_node.root = true;

    // Use the matchup-specific evaluation for the root
    let root_eval = evaluate_matchup(state);

    const EARLY_STOP_THRESHOLD: f32 = 0.8;
    const CHECK_INTERVAL: u32 = 100;

    for k in 0..iterations {
        do_mcts_for_matchup(&mut root_node, state, &root_eval);
        if k >= 100 && k % CHECK_INTERVAL == 0 {
            // Check if any move has an average score above threshold
            if root_node.s1_options.iter().any(|option| {
                option.visits > 0
                    && (option.total_score / option.visits as f32) > EARLY_STOP_THRESHOLD
            }) || root_node.s2_options.iter().any(|option| {
                option.visits > 0
                    && (option.total_score / option.visits as f32) > EARLY_STOP_THRESHOLD
            }) {
                break;
            }
        }
    }

    let max_depth = root_node.get_max_depth();
    let result = MctsResult {
        s1: root_node
            .s1_options
            .iter()
            .map(|v| MctsSideResult {
                move_choice: v.move_choice.clone(),
                total_score: v.total_score,
                visits: v.visits,
            })
            .collect(),
        s2: root_node
            .s2_options
            .iter()
            .map(|v| MctsSideResult {
                move_choice: v.move_choice.clone(),
                total_score: v.total_score,
                visits: v.visits,
            })
            .collect(),
        iteration_count: root_node.times_visited,
        max_depth,
    };

    result
}

/// Function to analyze a matchup using MCTS and classify it
pub fn analyze_matchup_with_mcts(state: &mut State, iterations: u32) -> f32 {
    // Get available moves for both Pokémon
    let (s1_options, s2_options) = state.get_all_options();

    // If no valid moves, return inconclusive result
    if s1_options.is_empty() || s2_options.is_empty() {
        return 0.5;
    }

    // Run MCTS simulation
    let result = perform_mcts_for_matchup(state, s1_options, s2_options, iterations);

    // Calculate winning percentage based on MCTS results
    calculate_winning_percentage(&result)
}

/// Calculate winning percentage based on MCTS results
fn calculate_winning_percentage(result: &MctsResult) -> f32 {
    // If no visits, return inconclusive result
    if result.iteration_count == 0 {
        return 0.5;
    }

    // Sum up s1's average score across all moves (weighted by visits)
    let mut max_avg_score = 0.0;

    for move_result in &result.s1 {
        max_avg_score += move_result.total_score / move_result.visits as f32;
    }

    max_avg_score
}

/// Classify matchup based on win percentage from MCTS
pub fn classify_matchup_from_mcts(win_percentage: f32) -> i8 {
    if win_percentage > 0.9 {
        // Strongly favorable (counter)
        2
    } else if win_percentage > 0.7 {
        // Favorable (check)
        1
    } else if win_percentage < 0.1 {
        // Strongly unfavorable (hard countered)
        -2
    } else if win_percentage < 0.3 {
        // Unfavorable (checked)
        -1
    } else {
        // Neutral matchup
        0
    }
}

/// Main function to compute and classify a matchup using MCTS
pub fn compute_matchup_mcts(state: &mut State) -> i8 {
    let win_percentage = analyze_matchup_with_mcts(state, 100);

    // Classify based on win percentage
    classify_matchup_from_mcts(win_percentage)
}
