use crate::choices::{Choice, Choices, MoveCategory};
use crate::engine::generate_instructions::{
    calculate_damage_rolls, cannot_use_move, move_has_no_effect, moves_first,
};
use crate::engine::items::Items;
use crate::instruction::StateInstructions;
use crate::matchup_mcts::{create_simulation_state, BattleConditions};
use crate::state::{Move, PokemonIndex, PokemonStatus, SideMovesFirst, SideReference, State};

// Import the modified MCTS matchup functions
use crate::matchup_calc_mcts::compute_matchup_mcts;

// Matchup Classification Thresholds
const STRONG_FAVORABLE_THRESHOLD: f32 = 0.9; // Win percentage for "strongly favorable" matchup (counter)
const FAVORABLE_THRESHOLD: f32 = 0.7; // Win percentage for "favorable" matchup (check)
const UNFAVORABLE_THRESHOLD: f32 = 0.3; // Win percentage for "unfavorable" matchup (checked)
const STRONG_UNFAVORABLE_THRESHOLD: f32 = 0.1; // Win percentage for "strongly unfavorable" matchup (hard countered)

/// Find the best move for a given Pokémon
pub fn find_best_move(state: &State, side_ref: &SideReference) -> (Choice, Move) {
    let side = match side_ref {
        SideReference::SideOne => &state.side_one,
        SideReference::SideTwo => &state.side_two,
    };

    let pokemon = side.get_active_immutable();
    let mut best_choice = Choice::default();
    let mut best_damage = 0;
    let mut best_move = Move::default();

    // Analyze each move
    let mut iter = pokemon.moves.into_iter();
    while let Some(m) = iter.next() {
        if m.id == Choices::NONE || m.pp <= 0 || m.disabled {
            continue;
        }

        let choice = m.choice.clone();

        // Skip moves that would have no effect or can't be used
        if move_has_no_effect(state, &choice, side_ref) || cannot_use_move(state, &choice, side_ref)
        {
            continue;
        }

        // Only consider damaging moves
        if choice.category != MoveCategory::Status {
            let damage_rolls =
                calculate_damage_rolls(state.clone(), side_ref, choice.clone(), &choice);

            if let Some(damage_range) = damage_rolls {
                if !damage_range.is_empty() {
                    // Calculate average damage
                    let avg_damage =
                        damage_range.iter().sum::<i16>() as f32 / damage_range.len() as f32;

                    // Check if this move is better than current best
                    if avg_damage as i16 > best_damage {
                        best_damage = avg_damage as i16;
                        best_choice = choice;
                        best_move = m.clone();
                    }
                }
            }
        }
    }

    (best_choice, best_move)
}

/// Get damage range for a specific move
pub fn get_damage_range(state: &State, choice: &Choice, side_ref: &SideReference) -> Vec<i16> {
    if choice.category == MoveCategory::Status {
        return Vec::new();
    }

    // Use existing damage calculation function
    match calculate_damage_rolls(state.clone(), side_ref, choice.clone(), choice) {
        Some(damage_range) => damage_range,
        None => Vec::new(),
    }
}

/// Get the best priority move damage for a Pokémon
pub fn get_best_priority_move_damage(state: &State, side_ref: &SideReference) -> i16 {
    let side = match side_ref {
        SideReference::SideOne => &state.side_one,
        SideReference::SideTwo => &state.side_two,
    };

    let pokemon = side.get_active_immutable();
    let mut best_priority_damage = 0;

    // Check for priority moves
    for m in pokemon.moves.into_iter() {
        if m.id == Choices::NONE || m.pp <= 0 || m.disabled {
            continue;
        }

        if m.choice.priority > 0 && m.choice.category != MoveCategory::Status {
            // Calculate damage for this priority move
            if let Some(damage_range) =
                calculate_damage_rolls(state.clone(), side_ref, m.choice.clone(), &m.choice)
            {
                if !damage_range.is_empty() {
                    // Use average damage as the metric
                    let avg_damage = (damage_range.iter().sum::<i16>() as f32
                        / damage_range.len() as f32) as i16;
                    if avg_damage > best_priority_damage {
                        best_priority_damage = avg_damage;
                    }
                }
            }
        }
    }

    best_priority_damage
}

/// Classify matchup based on win percentage
pub fn classify_matchup_result(win_percentage: f32) -> i8 {
    if win_percentage > STRONG_FAVORABLE_THRESHOLD {
        // Strongly favorable (counter)
        2
    } else if win_percentage > FAVORABLE_THRESHOLD {
        // Favorable (check)
        1
    } else if win_percentage < STRONG_UNFAVORABLE_THRESHOLD {
        // Strongly unfavorable (hard countered)
        -2
    } else if win_percentage < UNFAVORABLE_THRESHOLD {
        // Unfavorable (checked)
        -1
    } else {
        // Neutral matchup
        0
    }
}

/// Analyzes the matchup between the two Pokémon using mathematical win conditions
pub fn compute_matchup_mathematical(
    state: &mut State,
    s1_idx: PokemonIndex,
    s2_idx: PokemonIndex,
    conditions: &BattleConditions,
) -> i8 {
    // Create a simulation state with the specified conditions
    let sim_state = create_simulation_state(state, s1_idx, s2_idx, conditions);

    // Get Pokémon stats and info
    let s1_pokemon = sim_state.side_one.get_active_immutable();
    let s2_pokemon = sim_state.side_two.get_active_immutable();

    // Find the best move choices for each Pokémon
    let (s1_best_choice, s1_best_move) = find_best_move(&sim_state, &SideReference::SideOne);
    let (s2_best_choice, s2_best_move) = find_best_move(&sim_state, &SideReference::SideTwo);

    // Analyze speed to determine who goes first
    let mut s1_move_choice = s1_best_choice.clone();
    let mut s2_move_choice = s2_best_choice.clone();
    s1_move_choice.first_move = true;
    s2_move_choice.first_move = true;

    let mut instructions = StateInstructions::default();
    let first_mover = moves_first(
        &sim_state,
        &s1_move_choice,
        &s2_move_choice,
        &mut instructions,
    );

    // Determine who moves first (accounting for speed ties)
    let moves_first_side_one = match first_mover {
        SideMovesFirst::SideOne => true,
        SideMovesFirst::SideTwo => false,
        SideMovesFirst::SpeedTie => true, // In case of tie, we favor side one for analysis
    };

    // Calculate damage ranges
    let s1_damage_range = get_damage_range(&sim_state, &s1_best_choice, &SideReference::SideOne);
    let s2_damage_range = get_damage_range(&sim_state, &s2_best_choice, &SideReference::SideTwo);

    // Get average and max damage
    let (s1_avg_damage, _) = if s1_damage_range.is_empty() {
        (0, 0)
    } else {
        let avg =
            (s1_damage_range.iter().sum::<i16>() as f32 / s1_damage_range.len() as f32) as i16;
        let max = *s1_damage_range.iter().max().unwrap_or(&0);
        (avg, max)
    };

    let (s2_avg_damage, _) = if s2_damage_range.is_empty() {
        (0, 0)
    } else {
        let avg =
            (s2_damage_range.iter().sum::<i16>() as f32 / s2_damage_range.len() as f32) as i16;
        let max = *s2_damage_range.iter().max().unwrap_or(&0);
        (avg, max)
    };

    // Calculate priority move damage
    let s1_priority_damage = get_best_priority_move_damage(&sim_state, &SideReference::SideOne);
    let s2_priority_damage = get_best_priority_move_damage(&sim_state, &SideReference::SideTwo);

    // Analyze passive effects for both Pokémon
    let passive_effects_s1 =
        analyze_passive_effects(&sim_state, &SideReference::SideOne, &SideReference::SideTwo);

    let passive_effects_s2 =
        analyze_passive_effects(&sim_state, &SideReference::SideTwo, &SideReference::SideOne);

    // Get setup capabilities with improved analysis
    let setup_info_s1 = analyze_setup_capabilities(
        &sim_state,
        &SideReference::SideOne,
        &SideReference::SideTwo,
        s2_avg_damage,
        s1_avg_damage,
        moves_first_side_one,
        &passive_effects_s1,
        &passive_effects_s2,
    );

    let setup_info_s2 = analyze_setup_capabilities(
        &sim_state,
        &SideReference::SideTwo,
        &SideReference::SideOne,
        s1_avg_damage,
        s2_avg_damage,
        !moves_first_side_one,
        &passive_effects_s2,
        &passive_effects_s1,
    );

    // Check if moves lower stats
    let s1_has_stat_lowering = is_stat_lowering_move(&s1_best_move);
    let s2_has_stat_lowering = is_stat_lowering_move(&s2_best_move);

    // Get second best moves
    let s1_second_best_damage = if s1_has_stat_lowering {
        get_second_best_move_damage(&sim_state, &SideReference::SideOne)
    } else {
        0
    };

    let s2_second_best_damage = if s2_has_stat_lowering {
        get_second_best_move_damage(&sim_state, &SideReference::SideTwo)
    } else {
        0
    };

    // Get stat drop percentages
    let s1_stat_drop_percentage = if s1_has_stat_lowering {
        get_stat_drop_percentage(&s1_best_move)
    } else {
        1.0 // No drop
    };

    let s2_stat_drop_percentage = if s2_has_stat_lowering {
        get_stat_drop_percentage(&s2_best_move)
    } else {
        1.0 // No drop
    };

    // Calculate effective net damage considering passive effects
    let s1_net_passive =
        passive_effects_s1.passive_recovery_amount - passive_effects_s1.passive_damage_incoming;
    let s2_net_passive =
        passive_effects_s2.passive_recovery_amount - passive_effects_s2.passive_damage_incoming;

    let effective_damage_s1_to_s2 =
        s1_avg_damage + passive_effects_s1.passive_damage_outgoing - s2_net_passive;

    let effective_damage_s2_to_s1 =
        s2_avg_damage + passive_effects_s2.passive_damage_outgoing - s1_net_passive;

    // Ensure damage values are at least 0
    let effective_damage_s1_to_s2 = effective_damage_s1_to_s2.max(0);
    let effective_damage_s2_to_s1 = effective_damage_s2_to_s1.max(0);

    // Use our category-based analysis approach with improved setup analysis and MCTS
    classify_matchup_by_category(
        s1_pokemon.hp,
        s2_pokemon.hp,
        effective_damage_s1_to_s2,
        effective_damage_s2_to_s1,
        &passive_effects_s1,
        &passive_effects_s2,
        &setup_info_s1,
        &setup_info_s2,
        moves_first_side_one,
        s1_priority_damage,
        s2_priority_damage,
        s1_has_stat_lowering,
        s1_second_best_damage,
        s1_stat_drop_percentage,
        s2_has_stat_lowering,
        s2_second_best_damage,
        s2_stat_drop_percentage,
        state, // Pass the original state
    )
}

/// Calculate damage scaling after stat boosts
pub fn calculate_boosted_damage(
    base_damage: i16,
    attack_stages: i32,
    special_attack_stages: i32,
) -> i16 {
    // Use the higher of the two boost stages
    let effective_stages = attack_stages.max(special_attack_stages);

    if effective_stages <= 0 {
        return base_damage;
    }

    // Apply the standard damage multiplier formula from Pokémon games
    // +1 stage = 1.5x, +2 stages = 2x, +3 stages = 2.5x, etc.
    let multiplier = match effective_stages {
        1 => 1.5,
        2 => 2.0,
        3 => 2.5,
        4 => 3.0,
        5 => 3.5,
        6 => 4.0,
        _ => 4.0, // Cap at +6 stages
    };

    (base_damage as f32 * multiplier) as i16
}

/// Determine if setup is viable based on matchup conditions
pub fn is_setup_viable(
    current_hp: i16,
    avg_damage: i16,
    max_damage: i16,
    setup_turns: i32,
    has_recovery: bool,
    recovery_amount: i16,
    moves_first: bool,
) -> bool {
    if setup_turns == 0 {
        return false; // No setup moves
    }

    // If we move first, we take damage after setting up
    let damage_during_setup = if moves_first {
        avg_damage * setup_turns as i16
    } else {
        // If we move second, we take damage before and after setting up
        avg_damage * (setup_turns as i16 + 1)
    };

    // If we have recovery, we can potentially recover during setup
    if has_recovery && recovery_amount > avg_damage {
        // Recovery outpaces damage; setup is viable regardless of current HP
        return true;
    } else if has_recovery {
        // With recovery that doesn't outpace damage, we need to factor it in
        let effective_damage = damage_during_setup - recovery_amount;
        return current_hp > effective_damage;
    } else {
        // Without recovery, we simply need enough HP to survive the setup phase
        // We use a slightly higher threshold with max damage to be conservative
        let safety_threshold = (max_damage * setup_turns as i16) + (max_damage / 4);
        return current_hp > safety_threshold;
    }
}

/// Structure to track different types of recovery and passive damage
pub struct PassiveEffects {
    // Recovery
    pub has_active_recovery: bool, // Moves like Recover, Synthesis, etc.
    pub active_recovery_amount: i16, // HP recovered per active recovery use
    pub active_recovery_pp: i8,    // Total PP of active recovery moves

    pub passive_recovery_amount: i16, // HP recovered per turn passively (Leftovers, etc.)

    // Passive damage
    pub passive_damage_outgoing: i16, // Damage dealt to opponent each turn (Leech Seed, etc.)
    pub passive_damage_incoming: i16, // Damage taken each turn (effects like burn, poison)
}

/// Analyze recovery and passive damage capabilities of a Pokémon
pub fn analyze_passive_effects(
    state: &State,
    side_ref: &SideReference,
    opponent_ref: &SideReference,
) -> PassiveEffects {
    let side = match side_ref {
        SideReference::SideOne => &state.side_one,
        SideReference::SideTwo => &state.side_two,
    };

    let opponent_side = match opponent_ref {
        SideReference::SideOne => &state.side_one,
        SideReference::SideTwo => &state.side_two,
    };

    let pokemon = side.get_active_immutable();
    let opponent = opponent_side.get_active_immutable();

    let mut has_active_recovery = false;
    let mut active_recovery_amount = 0i16;
    let mut active_recovery_pp = 0i8;
    let mut passive_recovery_amount = 0i16;
    let mut passive_damage_outgoing = 0i16;
    let mut passive_damage_incoming = 0i16;

    // Check for active recovery moves
    for m in pokemon.moves.into_iter() {
        if m.id == Choices::NONE || m.pp <= 0 || m.disabled {
            continue;
        }

        match m.id {
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
            | Choices::HEALORDER => {
                has_active_recovery = true;
                let move_recovery = pokemon.maxhp / 2;
                if move_recovery > active_recovery_amount {
                    active_recovery_amount = move_recovery;
                    active_recovery_pp = m.pp;
                }
            }

            // 25% recovery moves that also provide passive effects
            Choices::AQUARING | Choices::INGRAIN => {
                // One-time setup cost, passive recovery afterwards
                passive_recovery_amount = passive_recovery_amount.max(pokemon.maxhp / 16);
            }

            Choices::JUNGLEHEALING => {
                has_active_recovery = true;
                active_recovery_amount = pokemon.maxhp / 4;
                active_recovery_pp = m.pp;
            }

            // Drain moves - considered active since they require attacking
            Choices::DRAINPUNCH
            | Choices::GIGADRAIN
            | Choices::HORNLEECH
            | Choices::DRAININGKISS
            | Choices::OBLIVIONWING
            | Choices::PARABOLICCHARGE => {
                has_active_recovery = true;
                // Recovery amount depends on damage, handled separately
            }

            // Rest is 100% recovery but causes sleep
            Choices::REST => {
                has_active_recovery = true;
                let move_recovery = pokemon.maxhp;
                if move_recovery > active_recovery_amount {
                    active_recovery_amount = move_recovery;
                    active_recovery_pp = m.pp;
                }
            }

            // Passive damage to opponent
            Choices::LEECHSEED => {
                passive_recovery_amount += opponent.maxhp / 8;
                passive_damage_outgoing += opponent.maxhp / 8;
            }

            // Add other moves that cause residual damage like Infestation, etc.
            Choices::INFESTATION
            | Choices::MAGMASTORM
            | Choices::FIRESPIN
            | Choices::WHIRLPOOL
            | Choices::SANDTOMB => {
                passive_damage_outgoing += opponent.maxhp / 8;
            }

            _ => {}
        }
    }

    // Check for passive recovery items
    if pokemon.item == Items::LEFTOVERS {
        passive_recovery_amount += pokemon.maxhp / 16;
    } else if pokemon.item == Items::BLACKSLUDGE {
        passive_recovery_amount += pokemon.maxhp / 16;
    }

    // Check for statuses causing passive damage
    match pokemon.status {
        PokemonStatus::BURN => {
            passive_damage_incoming += pokemon.maxhp / 16;
        }
        PokemonStatus::POISON => {
            passive_damage_incoming += pokemon.maxhp / 8;
        }
        PokemonStatus::TOXIC => {
            // Toxic damage increases each turn, use 2/16 as an average
            passive_damage_incoming += pokemon.maxhp / 8;
        }
        _ => {}
    }

    // Check for weather effects
    // match state.weather.weather_type {
    //     Weather::HAIL => {
    //         if !pokemon.types.contains(&Types::ICE) {
    //             passive_damage_incoming += pokemon.maxhp / 16;
    //         }
    //     }
    //     Weather::SAND => {
    //         if !pokemon.types.contains(&Types::ROCK)
    //             && !pokemon.types.contains(&Types::GROUND)
    //             && !pokemon.types.contains(&Types::STEEL)
    //         {
    //             passive_damage_incoming += pokemon.maxhp / 16;
    //         }
    //     }
    //     _ => {}
    // }

    PassiveEffects {
        has_active_recovery,
        active_recovery_amount,
        active_recovery_pp,
        passive_recovery_amount,
        passive_damage_outgoing,
        passive_damage_incoming,
    }
}

/// Setup capabilities for a Pokémon
pub struct SetupInfo {
    pub has_setup: bool,                // Does the Pokémon have setup moves?
    pub optimal_setup_stages: i32,      // Optimal number of setup turns for this matchup
    pub max_possible_stages: i32,       // Maximum possible stages (typically 6)
    pub attack_per_stage: i32,          // Attack boost per stage
    pub defense_per_stage: i32,         // Defense boost per stage
    pub special_attack_per_stage: i32,  // SpA boost per stage
    pub special_defense_per_stage: i32, // SpD boost per stage
    pub speed_per_stage: i32,           // Speed boost per stage
    pub is_physical_attacker: bool,     // Is this Pokémon primarily physical?
    pub is_special_attacker: bool,      // Is this Pokémon primarily special?
    pub boosts_speed: bool,             // Does the setup boost speed?
}

/// Calculate optimal setup stages for a given matchup
pub fn calculate_optimal_setup(
    hp: i16,
    opponent_damage: i16,
    my_damage: i16,
    setup_info: &SetupInfo,
    moves_first: bool,
    passive_effects: &PassiveEffects,
    opponent_passive_effects: &PassiveEffects,
) -> i32 {
    // If no setup or opponent deals overwhelming damage, don't set up
    if !setup_info.has_setup || opponent_damage >= hp / 2 {
        return 0;
    }

    // Account for recovery and passive effects
    let net_passive_effect =
        passive_effects.passive_recovery_amount - passive_effects.passive_damage_incoming;

    let effective_opponent_damage =
        opponent_damage + opponent_passive_effects.passive_damage_outgoing - net_passive_effect;

    // If we can't survive setting up even once, don't set up
    let damage_taken_first_turn = if moves_first {
        effective_opponent_damage
    } else {
        effective_opponent_damage * 2
    };

    if damage_taken_first_turn >= hp {
        return 0;
    }

    // Determine turn advantage needed to win
    let turns_to_ko_no_setup = if my_damage > 0 {
        if passive_effects.has_active_recovery {
            3
        } else {
            ((hp / my_damage) as f64).ceil() as i32
        }
    } else {
        99
    };

    let opponent_turns_to_ko = if effective_opponent_damage > 0 {
        ((hp / effective_opponent_damage) as f64).ceil() as i32
    } else {
        99
    };

    // Calculate damage boost per setup stage
    let damage_multiplier_per_stage = match setup_info.is_physical_attacker {
        true => 1.0 + (setup_info.attack_per_stage as f32 * 0.5),
        false => 1.0 + (setup_info.special_attack_per_stage as f32 * 0.5),
    };

    // Account for active recovery
    let recovery_factor = if passive_effects.has_active_recovery {
        1.5 // Recovery gives more setup flexibility
    } else {
        1.0
    };

    // Calculate optimal stages based on damage needed and turns available
    let mut optimal_stages = 0;
    let mut current_damage = my_damage as f32;

    // Keep setting up as long as it's beneficial
    for stage in 1..=setup_info.max_possible_stages {
        // Damage after this stage of setup
        let next_damage = current_damage * damage_multiplier_per_stage;

        // Turns to KO with this setup stage
        let ko_turns_with_setup = if next_damage > 0.0 {
            (hp as f32 / next_damage).ceil()
        } else {
            99.0
        };

        // Total turns including setup
        let total_turns = stage as f32 + ko_turns_with_setup;

        // Will we get KOed while setting up?
        let setup_safety_factor =
            (hp as f32 - (stage as f32 * effective_opponent_damage as f32)) / hp as f32;

        // If we can't survive this much setup, break
        if setup_safety_factor <= 0.0 {
            break;
        }

        // If setting up more improves our position, do it
        if (opponent_turns_to_ko as f32 * recovery_factor > total_turns)
            && (total_turns < turns_to_ko_no_setup as f32)
        {
            optimal_stages = stage;
            current_damage = next_damage;
        } else {
            // Setting up more isn't worth it
            break;
        }
    }

    // Special case: If setup boosts speed and opponent is faster, set up at least once
    if setup_info.boosts_speed && !moves_first && optimal_stages == 0 && opponent_turns_to_ko >= 3 {
        optimal_stages = 1;
    }

    optimal_stages
}

/// Analyze setup capabilities, taking matchup into account
pub fn analyze_setup_capabilities(
    state: &State,
    side_ref: &SideReference,
    opponent_ref: &SideReference,
    opponent_damage: i16,
    my_damage: i16,
    moves_first: bool,
    passive_effects: &PassiveEffects,
    opponent_passive_effects: &PassiveEffects,
) -> SetupInfo {
    let side = match side_ref {
        SideReference::SideOne => &state.side_one,
        SideReference::SideTwo => &state.side_two,
    };

    let pokemon = side.get_active_immutable();
    let mut has_setup = false;
    let mut attack_per_stage = 0;
    let mut defense_per_stage = 0;
    let mut special_attack_per_stage = 0;
    let mut special_defense_per_stage = 0;
    let mut speed_per_stage = 0;
    let mut boosts_speed = false;

    // Determine attacking type based on moves and stats
    let is_physical_attacker = is_primarily_physical(state, side_ref);
    let is_special_attacker = !is_physical_attacker;

    // Check for setup moves
    for m in pokemon.moves.into_iter() {
        if m.id == Choices::NONE || m.pp <= 0 || m.disabled {
            continue;
        }

        match m.id {
            // Attack boosts
            Choices::SWORDSDANCE => {
                has_setup = true;
                attack_per_stage = 2;
            }

            // Special Attack boosts
            Choices::NASTYPLOT | Choices::TAILGLOW => {
                has_setup = true;
                special_attack_per_stage = 2;
            }

            // Mixed setup moves
            Choices::DRAGONDANCE => {
                has_setup = true;
                attack_per_stage = 1;
                speed_per_stage = 1;
                boosts_speed = true;
            }
            Choices::CALMMIND => {
                has_setup = true;
                special_attack_per_stage = 1;
                special_defense_per_stage = 1;
            }
            Choices::BULKUP => {
                has_setup = true;
                attack_per_stage = 1;
                defense_per_stage = 1;
            }
            Choices::SHELLSMASH => {
                has_setup = true;
                attack_per_stage = 2;
                special_attack_per_stage = 2;
                speed_per_stage = 2;
                defense_per_stage = -1;
                special_defense_per_stage = -1;
                boosts_speed = true;
            }
            // Other setup moves...
            _ => {}
        }
    }

    let max_possible_stages = 6; // Maximum stat boost in Pokémon

    // Calculate optimal setup stages for this matchup
    let optimal_setup_stages = if has_setup {
        calculate_optimal_setup(
            pokemon.hp,
            opponent_damage,
            my_damage,
            &SetupInfo {
                has_setup,
                optimal_setup_stages: 0, // Will be calculated
                max_possible_stages,
                attack_per_stage,
                defense_per_stage,
                special_attack_per_stage,
                special_defense_per_stage,
                speed_per_stage,
                is_physical_attacker,
                is_special_attacker,
                boosts_speed,
            },
            moves_first,
            passive_effects,
            opponent_passive_effects,
        )
    } else {
        0
    };

    SetupInfo {
        has_setup,
        optimal_setup_stages,
        max_possible_stages,
        attack_per_stage,
        defense_per_stage,
        special_attack_per_stage,
        special_defense_per_stage,
        speed_per_stage,
        is_physical_attacker,
        is_special_attacker,
        boosts_speed,
    }
}

/// Helper to determine if a Pokémon is primarily physical
fn is_primarily_physical(state: &State, side_ref: &SideReference) -> bool {
    let side = match side_ref {
        SideReference::SideOne => &state.side_one,
        SideReference::SideTwo => &state.side_two,
    };

    let pokemon = side.get_active_immutable();

    // First check moves - if all damaging moves are of one category, that's our answer
    let mut physical_moves = 0;
    let mut special_moves = 0;

    for m in pokemon.moves.into_iter() {
        if m.id == Choices::NONE || m.pp <= 0 || m.disabled {
            continue;
        }

        if m.choice.category == MoveCategory::Physical {
            physical_moves += 1;
        } else if m.choice.category == MoveCategory::Special {
            special_moves += 1;
        }
    }

    if physical_moves > 0 && special_moves == 0 {
        return true;
    }

    if special_moves > 0 && physical_moves == 0 {
        return false;
    }

    // If mixed, use stats to decide
    pokemon.attack > pokemon.special_attack
}

/// Determine if a move lowers the user's stats
pub fn is_stat_lowering_move(m: &Move) -> bool {
    matches!(
        m.id,
        Choices::DRACOMETEOR
            | Choices::LEAFSTORM
            | Choices::OVERHEAT
            | Choices::PSYCHOBOOST
            | Choices::SUPERPOWER
            | Choices::FLEURCANNON
            | Choices::HAMMERARM
            | Choices::MAKEITRAIN
    )
}

/// Get the stat drop percentage for a move
pub fn get_stat_drop_percentage(m: &Move) -> f32 {
    match m.id {
        Choices::DRACOMETEOR
        | Choices::LEAFSTORM
        | Choices::OVERHEAT
        | Choices::PSYCHOBOOST
        | Choices::FLEURCANNON => 0.5, // -2 SpA drops to approximately 44% damage

        Choices::SUPERPOWER | Choices::MAKEITRAIN | Choices::HAMMERARM => 0.67, // -1 Atk drops to approximately 67% damage

        _ => 1.0, // No drop
    }
}

/// Get damage from second-best move
pub fn get_second_best_move_damage(state: &State, side_ref: &SideReference) -> i16 {
    let side = match side_ref {
        SideReference::SideOne => &state.side_one,
        SideReference::SideTwo => &state.side_two,
    };

    let pokemon = side.get_active_immutable();
    let (best_choice, _) = find_best_move(state, side_ref);
    let best_move_id = best_choice.move_id;

    let mut second_best_damage = 0;

    // Find second best move
    for mv in pokemon.moves.into_iter() {
        if mv.id == Choices::NONE || mv.pp <= 0 || mv.disabled || mv.id == best_move_id {
            continue;
        }

        if mv.choice.category != MoveCategory::Status {
            let damage_range = get_damage_range(state, &mv.choice, side_ref);
            if !damage_range.is_empty() {
                let avg_damage =
                    (damage_range.iter().sum::<i16>() as f32 / damage_range.len() as f32) as i16;
                if avg_damage > second_best_damage {
                    second_best_damage = avg_damage;
                }
            }
        }
    }

    second_best_damage
}

/// Calculate turns to KO considering stat-lowering moves
fn calculate_optimal_ko_path(
    target_hp: i16,
    best_move_damage: i16,
    priority_damage: i16,
    has_stat_lowering_move: bool,
    second_best_damage: i16,
    stat_drop_percentage: f32,
) -> (i32, bool) {
    // Returns (turns, uses_priority_for_last_hit)
    if best_move_damage <= 0 && priority_damage <= 0 {
        return (99, false); // Can't KO
    }

    // First check if priority move is a one-hit KO
    if priority_damage >= target_hp {
        return (1, true);
    }

    // Check if regular move is a one-hit KO
    if best_move_damage >= target_hp {
        return (1, false);
    }

    // For non-stat-lowering moves with priority
    if !has_stat_lowering_move && priority_damage > 0 {
        // Standard damage calculation
        let regular_turns_float = target_hp as f32 / best_move_damage as f32;
        let regular_turns = regular_turns_float.ceil() as i32;

        // Check if using priority for last hit is better
        // This happens when regular damage gets target close to KO range
        let regular_damage_dealt = best_move_damage * (regular_turns - 1) as i16;
        let remaining_hp = target_hp - regular_damage_dealt;

        if remaining_hp <= priority_damage {
            // Can finish with priority
            return (regular_turns, true);
        }

        return (regular_turns, false);
    }

    // For stat-lowering moves
    if has_stat_lowering_move {
        // Calculate damage after stat drop
        let post_drop_damage = (best_move_damage as f32 * stat_drop_percentage) as i16;

        // Option 1: Use stat-lowering move first, then reduced damage moves
        let remaining_hp = target_hp - best_move_damage;

        // Check if priority can finish after first hit
        if priority_damage >= remaining_hp {
            return (2, true); // Best move + priority finishes
        }

        // Calculate remaining turns with post-drop damage
        let remaining_turns_option1 = if post_drop_damage > 0 {
            let turns_float = remaining_hp as f32 / post_drop_damage as f32;
            turns_float.ceil() as i32
        } else {
            99
        };

        // Check if priority can finish at the end
        let damage_before_last = if remaining_turns_option1 > 1 {
            best_move_damage + (post_drop_damage * (remaining_turns_option1 - 1) as i16)
        } else {
            best_move_damage
        };

        let hp_before_last = target_hp - damage_before_last;
        let uses_priority1 = priority_damage > 0 && hp_before_last <= priority_damage;

        let ko_turns_option1 = if uses_priority1 {
            1 + (remaining_turns_option1 - 1).max(0)
        } else {
            1 + remaining_turns_option1
        };

        // Option 2: If there's a second best move, consider using it
        let (ko_turns_option2, uses_priority2) = if second_best_damage > 0 {
            if second_best_damage + best_move_damage >= target_hp {
                // 2HKO with second move then stat-lowering move
                (2, false)
            } else if priority_damage > 0 && second_best_damage + priority_damage >= target_hp {
                // 2HKO with second move then priority
                (2, true)
            } else if second_best_damage >= best_move_damage / 2 {
                // More efficient to use second best move repeatedly
                let turns_float = target_hp as f32 / second_best_damage as f32;
                let turns = turns_float.ceil() as i32;

                // Check if priority can finish at the end
                let damage_before_last = second_best_damage * (turns - 1) as i16;
                let hp_before_last = target_hp - damage_before_last;
                let uses_priority = priority_damage > 0 && hp_before_last <= priority_damage;

                if uses_priority {
                    (turns - 1 + 1, true)
                } else {
                    (turns, false)
                }
            } else {
                // Not better than option 1
                (ko_turns_option1, uses_priority1)
            }
        } else {
            // No viable second move
            (ko_turns_option1, uses_priority1)
        };

        // Return the better option
        if ko_turns_option1 <= ko_turns_option2 {
            return (ko_turns_option1, uses_priority1);
        } else {
            return (ko_turns_option2, uses_priority2);
        }
    }

    // Default case: standard calculation for regular move
    let turns = (target_hp as f32 / best_move_damage as f32).ceil() as i32;
    return (turns, false);
}

/// Matchup categories based on setup and recovery capabilities
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatchupCategory {
    PureDamageRace,     // [0,0,0,0] - Neither has setup or recovery
    ASetupOnly,         // [1,0,0,0] - A has setup only
    ARecoveryOnly,      // [0,1,0,0] - A has recovery only
    BSetupOnly,         // [0,0,1,0] - B has setup only
    BRecoveryOnly,      // [0,0,0,1] - B has recovery only
    ASetupBSetup,       // [1,0,1,0] - Both have setup
    ARecoveryBRecovery, // [0,1,0,1] - Both have recovery
    ASetupARecovery,    // [1,1,0,0] - A has setup and recovery
    BSetupBRecovery,    // [0,0,1,1] - B has setup and recovery
    ASetupBRecovery,    // [1,0,0,1] - A has setup, B has recovery
    ARecoveryBSetup,    // [0,1,1,0] - A has recovery, B has setup
    AFullBSetup,        // [1,1,1,0] - A has both, B has setup
    AFullBRecovery,     // [1,1,0,1] - A has both, B has recovery
    ASetupBFull,        // [1,0,1,1] - A has setup, B has both
    ARecoveryBFull,     // [0,1,1,1] - A has recovery, B has both
    BothFull,           // [1,1,1,1] - Both have setup and recovery
}

/// Get the matchup category based on capabilities
pub fn get_matchup_category(
    a_has_setup: bool,
    a_has_active_recovery: bool,
    b_has_setup: bool,
    b_has_active_recovery: bool,
) -> MatchupCategory {
    match (
        a_has_setup,
        a_has_active_recovery,
        b_has_setup,
        b_has_active_recovery,
    ) {
        (false, false, false, false) => MatchupCategory::PureDamageRace,
        (true, false, false, false) => MatchupCategory::ASetupOnly,
        (false, true, false, false) => MatchupCategory::ARecoveryOnly,
        (false, false, true, false) => MatchupCategory::BSetupOnly,
        (false, false, false, true) => MatchupCategory::BRecoveryOnly,
        (true, false, true, false) => MatchupCategory::ASetupBSetup,
        (false, true, false, true) => MatchupCategory::ARecoveryBRecovery,
        (true, true, false, false) => MatchupCategory::ASetupARecovery,
        (false, false, true, true) => MatchupCategory::BSetupBRecovery,
        (true, false, false, true) => MatchupCategory::ASetupBRecovery,
        (false, true, true, false) => MatchupCategory::ARecoveryBSetup,
        (true, true, true, false) => MatchupCategory::AFullBSetup,
        (true, true, false, true) => MatchupCategory::AFullBRecovery,
        (true, false, true, true) => MatchupCategory::ASetupBFull,
        (false, true, true, true) => MatchupCategory::ARecoveryBFull,
        (true, true, true, true) => MatchupCategory::BothFull,
    }
}

/// Main classification function using category-based approach
pub fn classify_matchup_by_category(
    hp_a: i16,
    hp_b: i16,
    damage_a_to_b: i16,
    damage_b_to_a: i16,
    passive_effects_a: &PassiveEffects,
    passive_effects_b: &PassiveEffects,
    setup_info_a: &SetupInfo,
    setup_info_b: &SetupInfo,
    a_is_faster: bool,
    priority_damage_a: i16,
    priority_damage_b: i16,
    a_has_stat_lowering_move: bool,
    a_second_best_damage: i16,
    a_stat_drop_percentage: f32,
    b_has_stat_lowering_move: bool,
    b_second_best_damage: i16,
    b_stat_drop_percentage: f32,
    state: &mut State, // Added state parameter
) -> i8 {
    // Determine category based on move capabilities, not viability
    let category = get_matchup_category(
        setup_info_a.has_setup,
        passive_effects_a.has_active_recovery,
        setup_info_b.has_setup,
        passive_effects_b.has_active_recovery,
    );

    // Calculate boosted damage based on optimal setup stages
    let a_boosted_damage = calculate_boosted_damage_optimal(
        damage_a_to_b,
        setup_info_a.optimal_setup_stages * setup_info_a.attack_per_stage,
        setup_info_a.optimal_setup_stages * setup_info_a.special_attack_per_stage,
    );

    let b_boosted_damage = calculate_boosted_damage_optimal(
        damage_b_to_a,
        setup_info_b.optimal_setup_stages * setup_info_b.attack_per_stage,
        setup_info_b.optimal_setup_stages * setup_info_b.special_attack_per_stage,
    );

    handle_setup_with_mcts(state)

    // // Dispatch to appropriate handler based on category
    // match category {
    //     MatchupCategory::PureDamageRace => analyze_damage_race(
    //         hp_a,
    //         hp_b,
    //         damage_a_to_b,
    //         damage_b_to_a,
    //         priority_damage_a,
    //         priority_damage_b,
    //         passive_effects_a,
    //         passive_effects_b,
    //         a_is_faster,
    //         a_has_stat_lowering_move,
    //         a_second_best_damage,
    //         a_stat_drop_percentage,
    //         b_has_stat_lowering_move,
    //         b_second_best_damage,
    //         b_stat_drop_percentage,
    //     ),

    //     // For setup-only categories, use MCTS instead of the mathematical approach
    //     MatchupCategory::ASetupOnly => analyze_one_setup_matchup_with_mcts(true, state),

    //     MatchupCategory::BSetupOnly => {
    //         // Use MCTS for Side B setup

    //         analyze_one_setup_matchup_with_mcts(false, state)
    //     }

    //     // For categories with setup on both sides or with complex interactions, use MCTS
    //     MatchupCategory::ASetupBSetup
    //     | MatchupCategory::ASetupARecovery
    //     | MatchupCategory::BSetupBRecovery
    //     | MatchupCategory::ASetupBRecovery
    //     | MatchupCategory::ARecoveryBSetup
    //     | MatchupCategory::AFullBSetup
    //     | MatchupCategory::AFullBRecovery
    //     | MatchupCategory::ASetupBFull
    //     | MatchupCategory::ARecoveryBFull
    //     | MatchupCategory::BothFull => handle_setup_with_mcts(state),

    //     // For recovery-only categories, keep using the mathematical approach
    //     MatchupCategory::ARecoveryOnly => {
    //         analyze_one_recovery_matchup(
    //             true,
    //             hp_a,
    //             hp_b,
    //             damage_a_to_b,
    //             damage_b_to_a,
    //             priority_damage_a,
    //             priority_damage_b,
    //             passive_effects_a,
    //             passive_effects_b,
    //             setup_info_a.optimal_setup_stages,
    //             a_boosted_damage, // Pass optimal setup info
    //             a_is_faster,
    //             a_has_stat_lowering_move,
    //             a_second_best_damage,
    //             a_stat_drop_percentage,
    //             b_has_stat_lowering_move,
    //             b_second_best_damage,
    //             b_stat_drop_percentage,
    //         )
    //     }

    //     MatchupCategory::BRecoveryOnly => {
    //         -analyze_one_recovery_matchup(
    //             false,
    //             hp_b,
    //             hp_a,
    //             damage_b_to_a,
    //             damage_a_to_b,
    //             priority_damage_b,
    //             priority_damage_a,
    //             passive_effects_b,
    //             passive_effects_a,
    //             setup_info_b.optimal_setup_stages,
    //             b_boosted_damage, // Pass optimal setup info
    //             !a_is_faster,
    //             b_has_stat_lowering_move,
    //             b_second_best_damage,
    //             b_stat_drop_percentage,
    //             a_has_stat_lowering_move,
    //             a_second_best_damage,
    //             a_stat_drop_percentage,
    //         )
    //     }

    //     MatchupCategory::ARecoveryBRecovery => analyze_damage_race(
    //         hp_a,
    //         hp_b,
    //         damage_a_to_b,
    //         damage_b_to_a,
    //         priority_damage_a,
    //         priority_damage_b,
    //         passive_effects_a,
    //         passive_effects_b,
    //         a_is_faster,
    //         a_has_stat_lowering_move,
    //         a_second_best_damage,
    //         a_stat_drop_percentage,
    //         b_has_stat_lowering_move,
    //         b_second_best_damage,
    //         b_stat_drop_percentage,
    //     ),
    // }
}

/// Calculate boosted damage based on optimal stages
pub fn calculate_boosted_damage_optimal(
    base_damage: i16,
    attack_stages: i32,
    special_attack_stages: i32,
) -> i16 {
    // Use the higher of the two boost values
    let effective_stages = attack_stages.max(special_attack_stages);

    if effective_stages <= 0 {
        return base_damage;
    }

    // Pokémon stat boost formula: +1 stage = 1.5x, +2 stages = 2x, etc.
    let multiplier = match effective_stages {
        1 => 1.5,
        2 => 2.0,
        3 => 2.5,
        4 => 3.0,
        5 => 3.5,
        6 => 4.0,
        _ => 4.0, // Cap at +6 stages
    };

    (base_damage as f32 * multiplier) as i16
}

/// Analyze a pure damage race matchup
fn analyze_damage_race(
    hp_a: i16,
    hp_b: i16,
    damage_a_to_b: i16,
    damage_b_to_a: i16,
    priority_damage_a: i16,
    priority_damage_b: i16,
    passive_effects_a: &PassiveEffects,
    passive_effects_b: &PassiveEffects,
    a_is_faster: bool,
    a_has_stat_lowering_move: bool,
    a_second_best_damage: i16,
    a_stat_drop_percentage: f32,
    b_has_stat_lowering_move: bool,
    b_second_best_damage: i16,
    b_stat_drop_percentage: f32,
) -> i8 {
    // Calculate net passive effects per turn
    let net_passive_a =
        passive_effects_a.passive_recovery_amount - passive_effects_a.passive_damage_incoming;

    let net_passive_b =
        passive_effects_b.passive_recovery_amount - passive_effects_b.passive_damage_incoming;

    // Calculate effective damage accounting for passive effects
    let effective_damage_a_to_b =
        damage_a_to_b + passive_effects_a.passive_damage_outgoing - net_passive_b;
    let effective_damage_b_to_a =
        damage_b_to_a + passive_effects_b.passive_damage_outgoing - net_passive_a;

    // Ensure damage is at least 0 (no negative damage)
    let effective_damage_a_to_b = effective_damage_a_to_b.max(0);
    let effective_damage_b_to_a = effective_damage_b_to_a.max(0);

    // If neither can do effective damage, it's a stalemate
    if effective_damage_a_to_b == 0 && effective_damage_b_to_a == 0 {
        return 0; // Neutral - neither can KO
    }

    // Handle one-sided damage cases
    if effective_damage_a_to_b == 0 && effective_damage_b_to_a > 0 {
        return -2; // Side B can damage A but A can't damage B
    }

    if effective_damage_a_to_b > 0 && effective_damage_b_to_a == 0 {
        return 2; // Side A can damage B but B can't damage A
    }

    // Calculate optimal KO paths with stat drops and priority
    let (turns_for_a, a_uses_priority) = calculate_optimal_ko_path(
        hp_b,
        effective_damage_a_to_b,
        priority_damage_a,
        a_has_stat_lowering_move,
        a_second_best_damage,
        a_stat_drop_percentage,
    );

    let (turns_for_b, b_uses_priority) = calculate_optimal_ko_path(
        hp_a,
        effective_damage_b_to_a,
        priority_damage_b,
        b_has_stat_lowering_move,
        b_second_best_damage,
        b_stat_drop_percentage,
    );

    // Compare KO speeds, factoring in priority for the last hit
    let a_effective_turns = turns_for_a;
    let b_effective_turns = turns_for_b;

    // Determine who effectively KOs first, considering speed and priority
    let a_kos_first = if a_effective_turns < b_effective_turns {
        true
    } else if a_effective_turns > b_effective_turns {
        false
    } else {
        // Equal number of turns - factor in priority and speed
        if a_uses_priority && !b_uses_priority {
            true // A's priority move goes first in the final turn
        } else if !a_uses_priority && b_uses_priority {
            false // B's priority move goes first in the final turn
        } else if a_uses_priority && b_uses_priority {
            // Both use priority on last turn - speed tie
            a_is_faster
        } else {
            // Neither uses priority - speed determines
            a_is_faster
        }
    };

    let turn_differential = a_effective_turns - b_effective_turns;

    // Calculate classification based on turn differential and who KOs first
    if turn_differential < -2 {
        return 2; // A KOs 3+ turns faster
    } else if turn_differential == -2 {
        return 2; // A KOs 2 turns faster
    } else if turn_differential == -1 {
        return 1; // A KOs 1 turn faster
    } else if turn_differential == 0 {
        // Same turns to KO, so priority/speed matters
        return if a_kos_first { 1 } else { -1 };
    } else if turn_differential == 1 {
        return -1; // B KOs 1 turn faster
    } else if turn_differential == 2 {
        return -2; // B KOs 2 turns faster
    } else {
        return -2; // B KOs 3+ turns faster
    }
}

/// Analyze a matchup where one side has active recovery
fn analyze_one_recovery_matchup(
    a_has_recovery: bool,
    hp_recovery: i16,
    hp_other: i16,
    damage_recovery_to_other: i16,
    damage_other_to_recovery: i16,
    priority_damage_recovery: i16,
    priority_damage_other: i16,
    passive_effects_recovery: &PassiveEffects,
    passive_effects_other: &PassiveEffects,
    setup_turns_recovery: i32,
    boosted_damage_recovery: i16,
    recovery_is_faster: bool,
    recovery_has_stat_lowering: bool,
    recovery_second_best: i16,
    recovery_stat_drop_pct: f32,
    other_has_stat_lowering: bool,
    other_second_best: i16,
    other_stat_drop_pct: f32,
) -> i8 {
    // Calculate net passive effects per turn for both sides
    let net_passive_recovery = passive_effects_recovery.passive_recovery_amount
        - passive_effects_recovery.passive_damage_incoming;

    let net_passive_other = passive_effects_other.passive_recovery_amount
        - passive_effects_other.passive_damage_incoming;

    // Calculate effective damage accounting for passive effects
    let effective_damage_recovery_to_other = damage_recovery_to_other
        + passive_effects_recovery.passive_damage_outgoing
        - net_passive_other;

    let effective_damage_other_to_recovery = damage_other_to_recovery
        + passive_effects_other.passive_damage_outgoing
        - net_passive_recovery;

    // Ensure damage values are at least 0
    let effective_damage_recovery_to_other = effective_damage_recovery_to_other.max(0);
    let effective_damage_other_to_recovery = effective_damage_other_to_recovery.max(0);

    // If neither can do effective damage, it's a stalemate
    if effective_damage_recovery_to_other == 0 && effective_damage_other_to_recovery == 0 {
        return 0; // Neutral - neither can KO
    }

    // Handle one-sided damage cases
    if effective_damage_recovery_to_other == 0 && effective_damage_other_to_recovery > 0 {
        // The recovery side can't damage but takes damage
        // Check if active recovery can sustain against damage
        if passive_effects_recovery.active_recovery_amount > effective_damage_other_to_recovery {
            return 0; // Stalemate - can recover faster than damaged
        } else {
            return -2; // Eventually loses to damage
        }
    }

    if effective_damage_recovery_to_other > 0 && effective_damage_other_to_recovery == 0 {
        return 2; // Can damage opponent but takes no damage
    }

    // First check if active recovery dominates incoming damage
    let recovery_dominates =
        passive_effects_recovery.active_recovery_amount > effective_damage_other_to_recovery;

    if recovery_dominates && passive_effects_recovery.active_recovery_pp >= 5 {
        // Recovery completely negates damage, now check if can deal damage
        if effective_damage_recovery_to_other > 0 {
            return 2; // Can recover all damage and deal damage
        } else {
            return 0; // Stalemate
        }
    }

    // Calculate sustainable damage and recovery
    let recovery_frequency_needed = effective_damage_other_to_recovery as f32
        / passive_effects_recovery.active_recovery_amount as f32;

    // Check if recovery is sustainable
    let is_sustainable = recovery_frequency_needed <= 0.5; // Can attack at least half the time

    if !is_sustainable {
        // Recovery not sufficient, revert to damage race
        return analyze_damage_race(
            hp_recovery,
            hp_other,
            effective_damage_recovery_to_other,
            effective_damage_other_to_recovery,
            priority_damage_recovery,
            priority_damage_other,
            passive_effects_recovery,
            passive_effects_other,
            recovery_is_faster,
            recovery_has_stat_lowering,
            recovery_second_best,
            recovery_stat_drop_pct,
            other_has_stat_lowering,
            other_second_best,
            other_stat_drop_pct,
        );
    }

    // Calculate effective damage output with recovery
    let optimal_frequency = recovery_frequency_needed.min(0.5);
    let effective_damage = effective_damage_recovery_to_other as f32 * (1.0 - optimal_frequency);

    // Check if priority move can be used with this strategy
    let effective_ko_turns = if effective_damage > 0.0 {
        (hp_other as f32 / effective_damage).ceil() as i32
    } else {
        99
    };

    // Determine if priority can be used for the final hit
    let uses_priority = priority_damage_recovery > 0
        && hp_other - (effective_damage * (effective_ko_turns as f32 - 1.0)) as i16
            <= priority_damage_recovery;

    let effective_ko_turns_with_priority = if uses_priority {
        effective_ko_turns - 1 + 1 // -1 for last regular attack, +1 for priority move
    } else {
        effective_ko_turns
    };

    // Calculate opponent's effective damage accounting for recovery
    let opponent_net_damage = effective_damage_other_to_recovery
        - (passive_effects_recovery.active_recovery_amount as f32 * optimal_frequency) as i16;

    // If opponent's net damage is negative or zero, they can't KO
    if opponent_net_damage <= 0 {
        return 2; // Can't be KO'd
    }

    // Calculate opponent's KO timing
    let opponent_ko_turns = if opponent_net_damage > 0 {
        (hp_recovery as f32 / opponent_net_damage as f32).ceil() as i32
    } else {
        99 // Can't KO
    };

    // Determine if opponent uses priority for final hit
    let opponent_uses_priority = priority_damage_other > 0
        && hp_recovery - (opponent_net_damage * (opponent_ko_turns - 1) as i16)
            <= priority_damage_other;

    let opponent_ko_turns_with_priority = if opponent_uses_priority {
        opponent_ko_turns - 1 + 1 // -1 for last regular attack, +1 for priority move
    } else {
        opponent_ko_turns
    };

    // Determine who KOs first
    let recovery_kos_first = if effective_ko_turns_with_priority < opponent_ko_turns_with_priority {
        true
    } else if effective_ko_turns_with_priority > opponent_ko_turns_with_priority {
        false
    } else {
        // Same number of turns - priority and speed matter
        if uses_priority && !opponent_uses_priority {
            true
        } else if !uses_priority && opponent_uses_priority {
            false
        } else if uses_priority && opponent_uses_priority {
            // Both use priority - speed decides
            recovery_is_faster
        } else {
            // Neither uses priority
            recovery_is_faster
        }
    };

    // Compare KO timing
    let turn_differential = effective_ko_turns_with_priority - opponent_ko_turns_with_priority;

    if opponent_ko_turns_with_priority >= 99 {
        return 2; // Can't be KO'd
    } else if turn_differential <= -2 {
        return 2; // KOs much faster
    } else if turn_differential == -1 {
        return 1; // KOs 1 turn faster
    } else if turn_differential == 0 {
        return if recovery_kos_first { 1 } else { -1 };
    } else if turn_differential == 1 {
        return -1; // Opponent KOs 1 turn faster
    } else {
        return -2; // Opponent KOs much faster
    }
}

/// Analyze a matchup with setup using MCTS
pub fn analyze_setup_matchup_with_mcts(state: &mut State) -> i8 {
    // Use MCTS to analyze the matchup
    compute_matchup_mcts(state)
}

/// Analyze a one-sided setup matchup using MCTS (replaces your analyze_one_setup_matchup)
pub fn analyze_one_setup_matchup_with_mcts(
    for_side_one: bool, // Whether setup is for side 1 (true) or side 2 (false)
    state: &mut State,
) -> i8 {
    // Get the raw MCTS result
    let result = compute_matchup_mcts(state);

    // If analyzing for side 2, invert the result
    if !for_side_one {
        return -result;
    }

    result
}

// /// Get win percentage from MCTS simulation
// pub fn get_matchup_win_percentage(state: &State, duration_ms: u64) -> f32 {
//     // Run MCTS simulation
//     analyze_matchup_with_mcts(
//         state,
//         10000, // iterations
//         Duration::from_millis(duration_ms),
//     )
// }

/// Helper method to be used within your existing classify_matchup_by_category function
/// to handle setup cases using MCTS
pub fn handle_setup_with_mcts(state: &mut State) -> i8 {
    compute_matchup_mcts(state)
}
