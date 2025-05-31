use crate::choices::{Choice, Choices, MoveCategory, MOVES};
use crate::engine::battle_environment::{
    initialize_battle_state, BattleEnvironment, BattleResult, DamageMaximizer, FirstMovePlayer, 
    MctsPlayer, Player, RandomPlayer,
};
use crate::engine::evaluate::evaluate;
use crate::engine::generate_instructions::{
    calculate_both_damage_rolls, generate_instructions_from_move_pair,
};
use crate::engine::state::MoveChoice;
use crate::instruction::{Instruction, StateInstructions};
use crate::matchup_calc_mcts::perform_mcts_for_matchup;
use crate::matchup_mcts::initialize_team_matchup_cache;
use crate::matchup_mcts::{analyze_matchup_cache, perform_mcts_with_team_matchups};
use crate::matchup_visualization_tool::MatchupVisualizer;
use crate::mcts::{perform_mcts, MctsResult};
use crate::search::{expectiminimax_search, iterative_deepen_expectiminimax, pick_safest};
use crate::state::{PokemonIndex, SideReference, State};
use clap::Parser;
use std::fs;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use std::process::exit;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

struct IOData {
    state: State,
    instruction_list: Vec<Vec<Instruction>>,
    last_instructions_generated: Vec<StateInstructions>,
}

#[derive(Parser)]
struct Cli {
    #[clap(short, long, default_value = "")]
    state: String,

    #[clap(subcommand)]
    subcmd: Option<SubCommand>,
}

#[derive(Parser)]
enum SubCommand {
    Expectiminimax(Expectiminimax),
    IterativeDeepening(IterativeDeepening),
    MonteCarloTreeSearch(MonteCarloTreeSearch),
    MonteCarloTreeSearchMU(MonteCarloTreeSearchMU),
    CalculateDamage(CalculateDamage),
    GenerateInstructions(GenerateInstructions),
    AnalyzeMatchups(AnalyzeMatchups),
    MU(MU),
    Battle(Battle),
}

#[derive(Parser)]
pub struct AnalyzeMatchups {}

#[derive(Parser)]
struct MU {
    #[clap(short, long, default_value_t = 100)]
    iterations: u32,
}

#[derive(Parser)]
struct Expectiminimax {
    #[clap(short, long, required = true)]
    state: String,

    #[clap(short, long, default_value_t = false)]
    ab_prune: bool,

    #[clap(short, long, default_value_t = 2)]
    depth: i8,
}

#[derive(Parser)]
struct IterativeDeepening {
    #[clap(short, long, required = true)]
    state: String,

    #[clap(short, long, default_value_t = 5000)]
    time_to_search_ms: u64,
}

#[derive(Parser)]
struct MonteCarloTreeSearch {
    #[clap(short, long, required = true)]
    state: String,

    #[clap(short, long, default_value_t = 5000)]
    time_to_search_ms: u64,
}

#[derive(Parser)]
struct MonteCarloTreeSearchMU {
    #[clap(short, long, required = false)]
    state: String,
    #[clap(short, long, default_value_t = 5000)]
    time_to_search_ms: u64,
}

#[derive(Parser)]
struct CalculateDamage {
    #[clap(short, long, required = true)]
    state: String,

    #[clap(short = 'o', long, required = true)]
    side_one_move: String,

    #[clap(short = 't', long, required = true)]
    side_two_move: String,

    #[clap(short = 'm', long, required = false, default_value_t = false)]
    side_one_moves_first: bool,
}

#[derive(Parser)]
struct GenerateInstructions {
    #[clap(short, long, required = true)]
    state: String,

    #[clap(short = 'o', long, required = true)]
    side_one_move: String,

    #[clap(short = 't', long, required = true)]
    side_two_move: String,
}

#[derive(Parser)]
struct Battle {
    #[clap(
        short = 'p',
        long,
        required = true,
        help = "Player 1 type: random, firstmove, damage, mcts"
    )]
    player_one: String,
    
    #[clap(
        short = 'q',
        long,
        required = true,
        help = "Player 2 type: random, firstmove, damage, mcts"
    )]
    player_two: String,
    
    #[clap(short, long, default_value_t = 100)]
    max_turns: u16,
    
    #[clap(short, long, default_value_t = 1)]
    runs: usize,
    
    #[clap(short, long, default_value_t = false)]
    verbose: bool,
    
    #[clap(short, long)]
    log_file: Option<String>,
    
    #[clap(short = 'j', long, default_value_t = 1)]
    threads: usize,
    
    #[clap(short = 't', long, default_value_t = 100)]
    mcts_time: u64,
    
    #[clap(long)]
    p1_mcts_time: Option<u64>,
    
    #[clap(long)]
    p2_mcts_time: Option<u64>,
}

impl Default for IOData {
    fn default() -> Self {
        IOData {
            state: State::default(),
            instruction_list: Vec::new(),
            last_instructions_generated: Vec::new(),
        }
    }
}

fn pprint_expectiminimax_result(
    result: &Vec<f32>,
    s1_options: &Vec<MoveChoice>,
    s2_options: &Vec<MoveChoice>,
    safest_choice: &(usize, f32),
    state: &State,
) {
    let s1_len = s1_options.len();
    let s2_len = s2_options.len();

    print!("{: <12}", " ");

    for s2_move in s2_options.iter() {
        print!("{: >12}", s2_move.to_string(&state.side_two));
    }
    print!("\n");

    for i in 0..s1_len {
        let s1_move_str = s1_options[i];
        print!("{:<12}", s1_move_str.to_string(&state.side_one));
        for j in 0..s2_len {
            let index = i * s2_len + j;
            print!("{number:>11.2} ", number = result[index]);
        }
        print!("\n");
    }
    print!(
        "{:<12}",
        s1_options[safest_choice.0].to_string(&state.side_one)
    );
}

fn print_mcts_result(state: &State, result: MctsResult) {
    let s1_joined_options = result
        .s1
        .iter()
        .map(|x| {
            format!(
                "{},{:.2},{}",
                x.move_choice.to_string(&state.side_one),
                x.total_score,
                x.visits
            )
        })
        .collect::<Vec<String>>()
        .join("|");
    let s2_joined_options = result
        .s2
        .iter()
        .map(|x| {
            format!(
                "{},{:.2},{}",
                x.move_choice.to_string(&state.side_two),
                x.total_score,
                x.visits
            )
        })
        .collect::<Vec<String>>()
        .join("|");

    println!("Total Iterations: {}", result.iteration_count);
    println!("side one: {}", s1_joined_options);
    println!("side two: {}", s2_joined_options);
}

fn pprint_mcts_result(state: &State, result: MctsResult) {
    println!("{}", state.pprint());
    println!("\nTotal Iterations: {}\n", result.iteration_count);
    println!("Maximum Depth: {}", result.max_depth);
    println!("Side One:");
    println!(
        "\t{:<25}{:>12}{:>12}{:>10}{:>10}",
        "Move", "Total Score", "Avg Score", "Visits", "% Visits"
    );
    for x in result.s1.iter() {
        println!(
            "\t{:<25}{:>12.2}{:>12.2}{:>10}{:>10.2}",
            x.move_choice.to_string(&state.side_one),
            x.total_score,
            x.total_score / x.visits as f32,
            x.visits,
            (x.visits as f32 / result.iteration_count as f32) * 100.0
        );
    }

    println!("Side Two:");
    println!(
        "\t{:<25}{:>12}{:>12}{:>10}{:>10}",
        "Move", "Total Score", "Avg Score", "Visits", "% Visits"
    );
    for x in result.s2.iter() {
        println!(
            "\t{:<25}{:>12.2}{:>12.2}{:>10}{:>10.2}",
            x.move_choice.to_string(&state.side_two),
            x.total_score,
            x.total_score / x.visits as f32,
            x.visits,
            (x.visits as f32 / result.iteration_count as f32) * 100.0
        );
    }
}

fn pprint_state_instruction_vector(instructions: &Vec<StateInstructions>) {
    for (i, instruction) in instructions.iter().enumerate() {
        println!("Index: {}", i);
        println!("StateInstruction: {:?}", instruction);
    }
}

fn print_subcommand_result(
    result: &Vec<f32>,
    side_one_options: &Vec<MoveChoice>,
    side_two_options: &Vec<MoveChoice>,
    state: &State,
) {
    let safest = pick_safest(&result, side_one_options.len(), side_two_options.len());
    let move_choice = side_one_options[safest.0];

    let joined_side_one_options = side_one_options
        .iter()
        .map(|x| format!("{}", x.to_string(&state.side_one)))
        .collect::<Vec<String>>()
        .join(",");
    println!("side one options: {}", joined_side_one_options);

    let joined_side_two_options = side_two_options
        .iter()
        .map(|x| format!("{}", x.to_string(&state.side_two)))
        .collect::<Vec<String>>()
        .join(",");
    println!("side two options: {}", joined_side_two_options);

    let joined = result
        .iter()
        .map(|x| format!("{:.2}", x))
        .collect::<Vec<String>>()
        .join(",");
    println!("matrix: {}", joined);
    println!("choice: {}", move_choice.to_string(&state.side_one));
    println!("evaluation: {}", safest.1);
}

/// Load state from file or create it from data files
fn load_or_create_state() -> State {
    // Otherwise, create a new state from data files
    let data_dir = PathBuf::from("data");
    // Try to load from a saved state file if it exists
    if let Ok(state_str) = fs::read_to_string(data_dir.join("saved_state.txt")) {
        return State::deserialize(&state_str);
    }

    let random_teams = match fs::read_to_string(data_dir.join("random_teams.json")) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Failed to read teams file: {}", err);
            panic!("Cannot continue without teams data");
        }
    };

    let pokedex = match fs::read_to_string(data_dir.join("pokedex.json")) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Failed to read pokedex file: {}", err);
            panic!("Cannot continue without pokedex data");
        }
    };

    let movedex = match fs::read_to_string(data_dir.join("moves.json")) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Failed to read moves file: {}", err);
            panic!("Cannot continue without moves data");
        }
    };

    // Create battle state from the loaded data
    let mut state = create_battle_state(&random_teams, &pokedex, &movedex);

    // Save the state for future use
    let serialized = state.serialize();
    let _ = fs::write(data_dir.join("saved_state.txt"), &serialized);
    state = State::deserialize(&serialized);
    state
}

/// Create a battle state from JSON data
fn create_battle_state(random_teams: &str, pokedex: &str, movedex: &str) -> State {
    // This function would parse the JSON data and initialize a State
    // For simplicity, this example uses initialize_battle_state from the battle environment module
    initialize_battle_state(random_teams, pokedex, movedex)
}

pub fn main() {
    let args = Cli::parse();
    let mut io_data = IOData::default();

    if args.state != "" {
        let state = State::deserialize(args.state.as_str());
        io_data.state = state;
    }

    let result;
    let mut state;
    let mut side_one_options;
    let mut side_two_options;
    match args.subcmd {
        None => {
            command_loop(io_data);
            exit(0);
        }
        Some(subcmd) => match subcmd {
            SubCommand::Expectiminimax(expectiminimax) => {
                state = State::deserialize(expectiminimax.state.as_str());
                (side_one_options, side_two_options) = state.root_get_all_options();
                result = expectiminimax_search(
                    &mut state,
                    expectiminimax.depth,
                    side_one_options.clone(),
                    side_two_options.clone(),
                    expectiminimax.ab_prune,
                    &Arc::new(Mutex::new(true)),
                );
                print_subcommand_result(&result, &side_one_options, &side_two_options, &state);
            }
            SubCommand::IterativeDeepening(iterative_deepending) => {
                state = State::deserialize(iterative_deepending.state.as_str());
                (side_one_options, side_two_options) = state.root_get_all_options();
                (side_one_options, side_two_options, result, _) = iterative_deepen_expectiminimax(
                    &mut state,
                    side_one_options.clone(),
                    side_two_options.clone(),
                    std::time::Duration::from_millis(iterative_deepending.time_to_search_ms),
                );
                print_subcommand_result(&result, &side_one_options, &side_two_options, &state);
            }
            SubCommand::MonteCarloTreeSearch(mcts) => {
                state = State::deserialize(mcts.state.as_str());
                (side_one_options, side_two_options) = state.root_get_all_options();
                let result = perform_mcts(
                    &mut state,
                    side_one_options.clone(),
                    side_two_options.clone(),
                    std::time::Duration::from_millis(mcts.time_to_search_ms),
                );
                pprint_mcts_result(&state, result);
            }

            SubCommand::MU(mu) => {
                let state = load_or_create_state();

                let mut sim_state = State::default();

                // Copy the selected Pokémon
                sim_state.side_one.pokemon[PokemonIndex::P0] =
                    state.side_one.pokemon[PokemonIndex::P0].clone();
                sim_state.side_two.pokemon[PokemonIndex::P0] =
                    state.side_two.pokemon[PokemonIndex::P0].clone();

                // Set them as active
                sim_state.side_one.active_index = PokemonIndex::P0;
                sim_state.side_two.active_index = PokemonIndex::P0;

                sim_state.use_last_used_move = true;

                if !state.side_one.can_use_tera() {
                    sim_state.side_one.pokemon[PokemonIndex::P5].terastallized = true;
                }
                if !state.side_two.can_use_tera() {
                    sim_state.side_two.pokemon[PokemonIndex::P5].terastallized = true;
                }

                // // Apply the battle conditions
                // apply_conditions(&mut sim_state, conditions);

                let (s1_options, s2_options) = sim_state.get_all_options();

                // Run MCTS simulation
                let result =
                    perform_mcts_for_matchup(&mut sim_state, s1_options, s2_options, mu.iterations);
                pprint_mcts_result(&sim_state, result);
            }

            SubCommand::AnalyzeMatchups(_) => {
                // Load state from file or create it
                let mut state = load_or_create_state();

                println!("Analyzing matchups between teams...");

                // Initialize cache and visualizer
                let cache = initialize_team_matchup_cache(&mut state);
                let mut visualizer = MatchupVisualizer::new(cache);

                // Analyze all matchups with detailed reasoning
                visualizer.analyze_all_matchups(&state);

                // Print the matchup matrix
                visualizer.print_matchup_matrix(&state);

                // Generate HTML visualization with move sets
                match visualizer.generate_html_visualization(&state) {
                    Ok(_) => println!(
                        "HTML visualization with move sets has been generated in the 'matchup_analysis' directory"
                    ),
                    Err(e) => println!("Failed to generate HTML visualization: {}", e),
                };

                // Example of getting detailed analysis for specific matchup
                let s1_idx = PokemonIndex::P0; // First Pokémon on side one
                let s2_idx = PokemonIndex::P0; // Third Pokémon on side two

                println!("\nDetailed analysis for specific matchup:");
                visualizer.print_detailed_matchup(&state, s1_idx, s2_idx);

                println!(
                    "\nCommand: 'matchup-detail <s1_idx> <s2_idx>' for specific matchup details"
                );
                println!("Example: 'matchup-detail 0 2' to see details of first Pokémon vs third opponent Pokémon");
            }

            SubCommand::MonteCarloTreeSearchMU(mcts_evo) => {
                // if state is provided, use it
                let mut state = if mcts_evo.state != "" {
                    State::deserialize(mcts_evo.state.as_str())
                } else {
                    // if not, read from data files
                    let data_dir = PathBuf::from("data");
                    let random_teams = match fs::read_to_string(data_dir.join("random_teams.json"))
                    {
                        Ok(content) => content,
                        Err(err) => {
                            eprintln!("Failed to read file: {}", err);
                            return;
                        }
                    };
                    let pokedex = match fs::read_to_string(data_dir.join("pokedex.json")) {
                        Ok(content) => content,
                        Err(err) => {
                            eprintln!("Failed to read file: {}", err);
                            return;
                        }
                    };
                    let movedex = match fs::read_to_string(data_dir.join("moves.json")) {
                        Ok(content) => content,
                        Err(err) => {
                            eprintln!("Failed to read file: {}", err);
                            return;
                        }
                    };
                    initialize_battle_state(&random_teams, &pokedex, &movedex)
                };
                println!("{}", state.serialize());
                let mut matchup_cache = initialize_team_matchup_cache(&mut state);

                analyze_matchup_cache(&state, &mut matchup_cache);

                (side_one_options, side_two_options) = state.root_get_all_options();

                let result = perform_mcts_with_team_matchups(
                    &mut state,
                    side_one_options.clone(),
                    side_two_options.clone(),
                    std::time::Duration::from_millis(mcts_evo.time_to_search_ms),
                    &mut matchup_cache,
                );
                pprint_mcts_result(&state, result);
                // Print cache statistics
                let (hits, misses, total, hit_rate) = matchup_cache.get_stats();
                println!("\nCache Statistics:");
                println!("  Total queries: {}", total);
                println!("  Cache hits: {} ({:.2}%)", hits, hit_rate);
                println!("  Cache misses: {} ({:.2}%)", misses, 100.0 - hit_rate);
                println!("  Cache size: {} entries", matchup_cache.cache_size());
                let (
                    complete,
                    partial,
                    fallbacks,
                    total,
                    complete_rate,
                    partial_rate,
                    fallback_rate,
                ) = matchup_cache.get_evaluation_stats();

                println!("\nEvaluation Statistics:");
                println!("  Total evaluations: {}", total);
                println!(
                    "  Complete evaluations: {} ({:.2}%)",
                    complete, complete_rate
                );
                println!("  Partial evaluations: {} ({:.2}%)", partial, partial_rate);
                println!("  Fallbacks: {} ({:.2}%)", fallbacks, fallback_rate);
                println!(
                    "  Strategic value mean: {}",
                    matchup_cache.team_strategic_value_mean
                );
                println!("  Base value mean: {}", matchup_cache.team_base_value_mean);
            }
            SubCommand::CalculateDamage(calculate_damage) => {
                state = State::deserialize(calculate_damage.state.as_str());
                let mut s1_choice = MOVES
                    .get(&Choices::from_str(calculate_damage.side_one_move.as_str()).unwrap())
                    .unwrap()
                    .to_owned();
                let mut s2_choice = MOVES
                    .get(&Choices::from_str(calculate_damage.side_two_move.as_str()).unwrap())
                    .unwrap()
                    .to_owned();
                let s1_moves_first = calculate_damage.side_one_moves_first;
                if calculate_damage.side_one_move == "switch" {
                    s1_choice.category = MoveCategory::Switch
                }
                if calculate_damage.side_two_move == "switch" {
                    s2_choice.category = MoveCategory::Switch
                }
                calculate_damage_io(&state, s1_choice, s2_choice, s1_moves_first);
            }
            SubCommand::GenerateInstructions(generate_instructions) => {
                state = State::deserialize(generate_instructions.state.as_str());
                let (s1_movechoice, s2_movechoice);
                match MoveChoice::from_string(
                    generate_instructions.side_one_move.as_str(),
                    &state.side_one,
                ) {
                    None => {
                        println!(
                            "Invalid move choice for side one: {}",
                            generate_instructions.side_one_move
                        );
                        exit(1);
                    }
                    Some(v) => s1_movechoice = v,
                }
                match MoveChoice::from_string(
                    generate_instructions.side_two_move.as_str(),
                    &state.side_two,
                ) {
                    None => {
                        println!(
                            "Invalid move choice for side two: {}",
                            generate_instructions.side_two_move
                        );
                        exit(1);
                    }
                    Some(v) => s2_movechoice = v,
                }
                let instructions = generate_instructions_from_move_pair(
                    &mut state,
                    &s1_movechoice,
                    &s2_movechoice,
                    true,
                );
                pprint_state_instruction_vector(&instructions);
            }
            SubCommand::Battle(battle) => {
                run_battle_command(battle);
            }
        },
    }

    exit(0);
}

fn calculate_damage_io(
    state: &State,
    s1_choice: Choice,
    s2_choice: Choice,
    side_one_moves_first: bool,
) {
    let (damages_dealt_s1, damages_dealt_s2) =
        calculate_both_damage_rolls(state, s1_choice, s2_choice, side_one_moves_first);

    for dmg in [damages_dealt_s1, damages_dealt_s2] {
        match dmg {
            Some(damages_vec) => {
                let joined = damages_vec
                    .iter()
                    .map(|x| format!("{:?}", x))
                    .collect::<Vec<String>>()
                    .join(",");
                println!("Damage Rolls: {}", joined);
            }
            None => {
                println!("Damage Rolls: 0");
            }
        }
    }
}

fn command_loop(mut io_data: IOData) {
    loop {
        print!("> ");
        let _ = io::stdout().flush();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(error) => {
                println!("Error reading input: {}", error);
                continue;
            }
        }
        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap_or("");
        let mut args = parts;

        match command {
            "state" | "s" => {
                let state_string;
                match args.next() {
                    Some(s) => {
                        state_string = s;
                        let state = State::deserialize(state_string);
                        io_data.state = state;
                        println!("state initialized");
                    }
                    None => {
                        println!("Expected state string");
                    }
                }
                println!("{:?}", io_data.state);
            }
            "serialize" | "ser" => {
                println!("{}", io_data.state.serialize());
            }
            "matchup" | "m" => {
                println!("{}", io_data.state.pprint());
            }
            "generate-instructions" | "g" => {
                let (s1_move, s2_move);
                match args.next() {
                    Some(s) => match MoveChoice::from_string(s, &io_data.state.side_one) {
                        Some(m) => {
                            s1_move = m;
                        }
                        None => {
                            println!("Invalid move choice for side one: {}", s);
                            continue;
                        }
                    },
                    None => {
                        println!("Usage: generate-instructions <side-1 move> <side-2 move>");
                        continue;
                    }
                }
                match args.next() {
                    Some(s) => match MoveChoice::from_string(s, &io_data.state.side_two) {
                        Some(m) => {
                            s2_move = m;
                        }
                        None => {
                            println!("Invalid move choice for side two: {}", s);
                            continue;
                        }
                    },
                    None => {
                        println!("Usage: generate-instructions <side-1 choice> <side-2 choice>");
                        continue;
                    }
                }
                let instructions = generate_instructions_from_move_pair(
                    &mut io_data.state,
                    &s1_move,
                    &s2_move,
                    true,
                );
                pprint_state_instruction_vector(&instructions);
                io_data.last_instructions_generated = instructions;
            }
            "calculate-damage" | "d" => {
                let (mut s1_choice, mut s2_choice);
                match args.next() {
                    Some(s) => {
                        s1_choice = MOVES
                            .get(&Choices::from_str(s).unwrap())
                            .unwrap()
                            .to_owned();
                        if s == "switch" {
                            s1_choice.category = MoveCategory::Switch
                        }
                    }
                    None => {
                        println!("Usage: calculate-damage <side-1 move> <side-2 move> <side-1-moves-first>");
                        continue;
                    }
                }
                match args.next() {
                    Some(s) => {
                        s2_choice = MOVES
                            .get(&Choices::from_str(s).unwrap())
                            .unwrap()
                            .to_owned();
                        if s == "switch" {
                            s2_choice.category = MoveCategory::Switch
                        }
                    }
                    None => {
                        println!("Usage: calculate-damage <side-1 move> <side-2 move> <side-1-moves-first>");
                        continue;
                    }
                }
                let s1_moves_first: bool;
                match args.next() {
                    Some(s) => {
                        s1_moves_first = s.parse::<bool>().unwrap();
                    }
                    None => {
                        println!("Usage: calculate-damage <side-1 move> <side-2 move> <side-1-moves-first>");
                        continue;
                    }
                }
                calculate_damage_io(&io_data.state, s1_choice, s2_choice, s1_moves_first);
            }
            "instructions" | "i" => {
                println!("{:?}", io_data.last_instructions_generated);
            }
            "evaluate" | "ev" => {
                println!("Evaluation: {}", evaluate(&io_data.state));
            }
            "iterative-deepening" | "id" => match args.next() {
                Some(s) => {
                    let max_time_ms = s.parse::<u64>().unwrap();
                    let (side_one_options, side_two_options) = io_data.state.root_get_all_options();

                    let start_time = std::time::Instant::now();
                    let (s1_moves, s2_moves, result, depth_searched) =
                        iterative_deepen_expectiminimax(
                            &mut io_data.state,
                            side_one_options.clone(),
                            side_two_options.clone(),
                            std::time::Duration::from_millis(max_time_ms),
                        );
                    let elapsed = start_time.elapsed();

                    let safest_choice = pick_safest(&result, s1_moves.len(), s2_moves.len());

                    pprint_expectiminimax_result(
                        &result,
                        &s1_moves,
                        &s2_moves,
                        &safest_choice,
                        &io_data.state,
                    );
                    println!("Took: {:?}", elapsed);
                    println!("Depth Searched: {}", depth_searched);
                }
                None => {
                    println!("Usage: iterative-deepening <timeout_ms>");
                    continue;
                }
            },
            "monte-carlo-tree-search" | "mcts" => match args.next() {
                Some(s) => {
                    let max_time_ms = s.parse::<u64>().unwrap();
                    let (side_one_options, side_two_options) = io_data.state.root_get_all_options();

                    let start_time = std::time::Instant::now();
                    let result = perform_mcts(
                        &mut io_data.state,
                        side_one_options.clone(),
                        side_two_options.clone(),
                        std::time::Duration::from_millis(max_time_ms),
                    );
                    let elapsed = start_time.elapsed();
                    pprint_mcts_result(&io_data.state, result);

                    println!("\nTook: {:?}", elapsed);
                }
                None => {
                    println!("Usage: monte-carlo-tree-search <timeout_ms>");
                    continue;
                }
            },
            "apply" | "a" => match args.next() {
                Some(s) => {
                    let index = s.parse::<usize>().unwrap();
                    let instructions = io_data.last_instructions_generated.remove(index);
                    io_data
                        .state
                        .apply_instructions(&instructions.instruction_list);
                    io_data.instruction_list.push(instructions.instruction_list);
                    io_data.last_instructions_generated = Vec::new();
                    println!("Applied instructions at index {}", index)
                }
                None => {
                    println!("Usage: apply <instruction index>");
                    continue;
                }
            },
            "pop" | "p" => {
                if io_data.instruction_list.is_empty() {
                    println!("No instructions to pop");
                    continue;
                }
                let instructions = io_data.instruction_list.pop().unwrap();
                io_data.state.reverse_instructions(&instructions);
                println!("Popped last applied instructions");
            }
            "pop-all" | "pa" => {
                for i in io_data.instruction_list.iter().rev() {
                    io_data.state.reverse_instructions(i);
                }
                io_data.instruction_list.clear();
                println!("Popped all applied instructions");
            }
            "expectiminimax" | "e" => match args.next() {
                Some(s) => {
                    let mut ab_prune = false;
                    match args.next() {
                        Some(s) => ab_prune = s.parse::<bool>().unwrap(),
                        None => {}
                    }
                    let depth = s.parse::<i8>().unwrap();
                    let (side_one_options, side_two_options) = io_data.state.root_get_all_options();
                    let start_time = std::time::Instant::now();
                    let result = expectiminimax_search(
                        &mut io_data.state,
                        depth,
                        side_one_options.clone(),
                        side_two_options.clone(),
                        ab_prune,
                        &Arc::new(Mutex::new(true)),
                    );
                    let elapsed = start_time.elapsed();

                    let safest_choice =
                        pick_safest(&result, side_one_options.len(), side_two_options.len());
                    pprint_expectiminimax_result(
                        &result,
                        &side_one_options,
                        &side_two_options,
                        &safest_choice,
                        &io_data.state,
                    );
                    println!("\nTook: {:?}", elapsed);
                }
                None => {
                    println!("Usage: expectiminimax <depth> <ab_prune=false>");
                    continue;
                }
            },
            "" => {
                continue;
            }
            "exit" | "quit" | "q" => {
                break;
            }
            command => {
                println!("Unknown command: {}", command);
            }
        }
    }
}

fn run_battle_command(battle: Battle) {
    use std::time::Instant;
    use rayon::prelude::*;
    
    // Set up logging if requested
    if let Some(log_file) = &battle.log_file {
        std::env::set_var("BATTLE_LOG_FILE", log_file);
    }
    
    // Create player functions based on types
    let create_player = |player_type: &str, name: String, search_time: u64| -> Box<dyn Player> {
        match player_type {
            "random" => Box::new(RandomPlayer::new(name)),
            "firstmove" => Box::new(FirstMovePlayer::new(name)),
            "damage" => Box::new(DamageMaximizer::new(name)),
            "mcts" => Box::new(MctsPlayer::new(name, search_time)),
            _ => {
                eprintln!("Unknown player type: {}. Using random player.", player_type);
                Box::new(RandomPlayer::new(name))
            }
        }
    };
    
    // Determine MCTS search times
    let p1_time = battle.p1_mcts_time.unwrap_or(battle.mcts_time);
    let p2_time = battle.p2_mcts_time.unwrap_or(battle.mcts_time);
    
    if battle.runs == 1 {
        // Single battle - run directly
        let player1 = create_player(&battle.player_one, format!("Player 1 ({})", battle.player_one), p1_time);
        let player2 = create_player(&battle.player_two, format!("Player 2 ({})", battle.player_two), p2_time);
        
        let mut env = BattleEnvironment::new(player1, player2, battle.max_turns as usize, battle.verbose);
        
        if let Some(log_file) = &battle.log_file {
            env = env.with_log_file(log_file.clone());
        }
        
        // Create initial state
        let data_dir = PathBuf::from("data");
        let random_teams = fs::read_to_string(data_dir.join("random_teams.json"))
            .expect("Failed to read teams file");
        let pokedex = fs::read_to_string(data_dir.join("pokedex.json"))
            .expect("Failed to read pokedex file");
        let movedex = fs::read_to_string(data_dir.join("moves.json"))
            .expect("Failed to read moves file");
        let initial_state = initialize_battle_state(&random_teams, &pokedex, &movedex);
        
        let result = env.run_battle(initial_state);
        
        println!("\nBattle complete!");
        match result.winner {
            Some(SideReference::SideOne) => println!("Winner: Player 1 ({})", battle.player_one),
            Some(SideReference::SideTwo) => println!("Winner: Player 2 ({})", battle.player_two),
            None => println!("Winner: Draw"),
        }
        println!("Total turns: {}", result.turn_count);
        
        if battle.verbose {
            println!("\nFinal state:");
            println!("{}", result.final_state.pprint());
        }
    } else {
        // Multiple battles - run in parallel
        println!("Running {} battles with {} threads...", battle.runs, battle.threads);
        let start_time = Instant::now();
        
        // Pre-generate states for all battles
        let states: Vec<State> = (0..battle.runs)
            .map(|_| {
                let data_dir = PathBuf::from("data");
                let random_teams = fs::read_to_string(data_dir.join("random_teams.json"))
                    .expect("Failed to read teams file");
                let pokedex = fs::read_to_string(data_dir.join("pokedex.json"))
                    .expect("Failed to read pokedex file");
                let movedex = fs::read_to_string(data_dir.join("moves.json"))
                    .expect("Failed to read moves file");
                initialize_battle_state(&random_teams, &pokedex, &movedex)
            })
            .collect();
        
        // Run battles in parallel
        let results: Vec<_> = states
            .into_par_iter()
            .enumerate()
            .map(|(battle_idx, state)| {
                let p1_name = format!("Player 1 ({}-{})", battle.player_one, battle_idx);
                let p2_name = format!("Player 2 ({}-{})", battle.player_two, battle_idx);
                
                let player1 = create_player(&battle.player_one, p1_name, p1_time);
                let player2 = create_player(&battle.player_two, p2_name, p2_time);
                
                let env = BattleEnvironment::new(player1, player2, battle.max_turns as usize, false);
                env.run_battle(state)
            })
            .collect();
        
        // Aggregate results
        let mut p1_wins = 0;
        let mut p2_wins = 0;
        let mut draws = 0;
        let mut total_turns = 0;
        
        for battle_result in results {
            total_turns += battle_result.turn_count;
            match battle_result.winner {
                Some(SideReference::SideOne) => p1_wins += 1,
                Some(SideReference::SideTwo) => p2_wins += 1,
                None => draws += 1,
            }
        }
        
        let elapsed = start_time.elapsed();
        
        println!("\nBattle Results:");
        println!("===============");
        println!("{} wins: {} ({:.1}%)", battle.player_one, p1_wins, 
                 (p1_wins as f64 / battle.runs as f64) * 100.0);
        println!("{} wins: {} ({:.1}%)", battle.player_two, p2_wins, 
                 (p2_wins as f64 / battle.runs as f64) * 100.0);
        println!("Draws: {} ({:.1}%)", draws, 
                 (draws as f64 / battle.runs as f64) * 100.0);
        println!("\nAverage turns per battle: {:.1}", 
                 total_turns as f64 / battle.runs as f64);
        println!("Total time: {:.2}s", elapsed.as_secs_f64());
        println!("Time per battle: {:.3}s", elapsed.as_secs_f64() / battle.runs as f64);
    }
}
