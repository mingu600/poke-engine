use crate::battle_format::BattleFormat;
use crate::choices::Choices;
use crate::engine::abilities::Abilities;
use crate::engine::generate_instructions::generate_instructions_from_move_pair;
use crate::engine::items::Items;
use crate::engine::state::{MoveChoice, Terrain, Weather};
use crate::instruction::StateInstructions;
use crate::mcts::perform_mcts;
use crate::pokemon::PokemonName;
use crate::state::{LastUsedMove, SideReference, VolatileStatusDurations};
use crate::state::{
    Move, Pokemon, PokemonIndex, PokemonMoves, PokemonNature, PokemonStatus, PokemonType, Side,
    SideConditions, SidePokemon, State, StateTerrain, StateTrickRoom, StateWeather,
};
use deunicode::deunicode;
use rand::{thread_rng, Rng};
use serde::Deserialize;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Write;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Deserialize)]
struct PackedTeamEntry {
    #[serde(rename = "packedTeam")]
    packed_team: String,
}

#[derive(Deserialize)]
struct PokemonDexEntry {
    types: Vec<String>,
    #[serde(rename = "baseStats")]
    base_stats: BaseStats,
    weightkg: f32,
}

#[derive(Deserialize)]
struct BaseStats {
    hp: u16,
    attack: u16,
    defense: u16,
    #[serde(rename = "special-attack")]
    special_attack: u16,
    #[serde(rename = "special-defense")]
    special_defense: u16,
    speed: u16,
}

fn normalize_name(name: &str) -> String {
    deunicode(name) // Handle unicode characters
        .replace(" ", "")
        .replace("-", "")
        .replace(".", "")
        .replace("'", "")
        .replace("%", "")
        .replace("*", "")
        .replace(":", "")
        .replace("(", "")
        .replace(")", "")
        .trim()
        .to_lowercase()
}

#[derive(Debug)]
struct PackedPokemon {
    nickname: String,
    species: Option<String>, // None if identical to nickname
    item: String,
    ability: String,
    moves: Vec<String>,
    nature: String,
    evs: Vec<u8>,
    tera_type: String,
}

#[derive(Deserialize)]
struct MoveDexEntry {
    pp: i8,
    #[serde(flatten)]
    _other: HashMap<String, serde_json::Value>,
}

fn parse_packed_team(packed_team: &str) -> Vec<PackedPokemon> {
    packed_team
        .split(']')
        .filter(|s| !s.is_empty())
        .map(|pokemon_str| {
            let parts: Vec<&str> = pokemon_str.split('|').collect();

            // Parse EVs
            let evs = if parts[6].is_empty() {
                vec![0, 0, 0, 0, 0, 0]
            } else {
                let mut ev_list = parts[6]
                    .split(',')
                    .map(|s| s.parse::<u8>().unwrap_or(0))
                    .collect::<Vec<_>>();
                while ev_list.len() < 6 {
                    ev_list.push(0);
                }
                ev_list
            };

            // Parse IVs
            // let ivs = if parts.len() > 8 && !parts[8].is_empty() {
            //     let mut iv_list = parts[8]
            //         .split(',')
            //         .map(|s| s.parse::<u8>().unwrap_or(31))
            //         .collect::<Vec<_>>();
            //     while iv_list.len() < 6 {
            //         iv_list.push(31);
            //     }
            //     iv_list
            // } else {
            //     vec![31, 31, 31, 31, 31, 31]
            // };

            // // Get level
            // let level = if parts.len() > 10 && !parts[10].is_empty() {
            //     parts[10].parse().unwrap_or(100)
            // } else {
            //     100
            // };

            // Get tera type from last comma-separated value in the last part
            let tera_type = if parts.last().unwrap().contains(',') {
                parts
                    .last()
                    .unwrap()
                    .split(',')
                    .last()
                    .unwrap_or("")
                    .to_string()
            } else {
                "".to_string()
            };

            PackedPokemon {
                nickname: normalize_name(parts[0]),
                species: if parts[1].is_empty() {
                    None
                } else {
                    Some(normalize_name(parts[1]))
                },
                item: normalize_name(parts[2]),
                ability: normalize_name(parts[3]),
                moves: parts[4].split(',').map(normalize_name).collect(),
                nature: if parts[5].is_empty() {
                    "serious".to_string()
                } else {
                    normalize_name(parts[5])
                },
                evs,
                // gender: parts[7].to_string(),
                // ivs,
                // shiny: parts.len() > 9 && parts[9] == "S",
                // level,
                tera_type,
            }
        })
        .collect()
}

fn create_pokemon(
    packed: &PackedPokemon,
    pokedex: &HashMap<String, PokemonDexEntry>,
    movedex: &HashMap<String, MoveDexEntry>,
) -> Pokemon {
    // Previous pokedex lookup logic remains the same
    let pokemon_name = packed.species.as_ref().unwrap_or(&packed.nickname);
    let dex_entry = pokedex
        .get(pokemon_name)
        .unwrap_or_else(|| panic!("Pokemon not found in pokedex: {}", pokemon_name));

    // Type conversion remains the same
    let type1 =
        PokemonType::from_str(&normalize_name(&dex_entry.types[0])).unwrap_or(PokemonType::NORMAL);
    let type2 = if dex_entry.types.len() > 1 {
        PokemonType::from_str(&normalize_name(&dex_entry.types[1])).unwrap_or(PokemonType::TYPELESS)
    } else {
        PokemonType::TYPELESS
    };

    let mut moves = PokemonMoves {
        m0: Move::default(),
        m1: Move::default(),
        m2: Move::default(),
        m3: Move::default(),
    };

    // Set up moves with PP from movedex
    for (i, move_name) in packed.moves.iter().enumerate() {
        let normalized_move = normalize_name(move_name);
        let move_choice = Choices::from_str(&normalized_move).unwrap_or(Choices::NONE);

        // Look up PP in movedex, default to 32 if not found
        let pp = movedex
            .get(&normalized_move)
            .map(|entry| (entry.pp as f32 * 1.6) as i8)
            .unwrap_or(32);

        // Get the full choice data from MOVES constant
        let choice = if let Some(move_info) = crate::choices::MOVES.get(&move_choice) {
            move_info.clone()
        } else {
            Default::default()
        };

        let move_data = Move {
            id: move_choice,
            disabled: false,
            pp,
            choice,
        };

        match i {
            0 => moves.m0 = move_data,
            1 => moves.m1 = move_data,
            2 => moves.m2 = move_data,
            3 => moves.m3 = move_data,
            _ => break,
        }
    }

    // Rest of the Pokemon creation remains the same
    Pokemon {
        id: PokemonName::from_str(
            &packed
                .species
                .as_ref()
                .unwrap_or(&packed.nickname)
                .to_uppercase(),
        )
        .unwrap_or(PokemonName::NONE),
        level: 100,
        types: (type1, type2),
        base_types: (type1, type2),
        hp: ((2 * dex_entry.base_stats.hp + 31 + (packed.evs[0] as u16 / 4)) * 100 / 100 + 110)
            as i16,
        maxhp: ((2 * dex_entry.base_stats.hp + 31 + (packed.evs[0] as u16 / 4)) * 100 / 100 + 110)
            as i16,
        ability: Abilities::from_str(&normalize_name(&packed.ability)).unwrap_or(Abilities::NONE),
        base_ability: Abilities::from_str(&normalize_name(&packed.ability))
            .unwrap_or(Abilities::NONE),
        item: Items::from_str(&normalize_name(&packed.item)).unwrap_or(Items::NONE),
        nature: PokemonNature::from_str(&normalize_name(&packed.nature))
            .unwrap_or(PokemonNature::SERIOUS),
        evs: (
            packed.evs[0],
            packed.evs[1],
            packed.evs[2],
            packed.evs[3],
            packed.evs[4],
            packed.evs[5],
        ),
        attack: ((2 * dex_entry.base_stats.attack + 31 + (packed.evs[1] as u16 / 4)) * 100 / 100
            + 5) as i16,
        defense: ((2 * dex_entry.base_stats.defense + 31 + (packed.evs[2] as u16 / 4)) * 100 / 100
            + 5) as i16,
        special_attack: ((2 * dex_entry.base_stats.special_attack
            + 31
            + (packed.evs[3] as u16 / 4))
            * 100
            / 100
            + 5) as i16,
        special_defense: ((2 * dex_entry.base_stats.special_defense
            + 31
            + (packed.evs[4] as u16 / 4))
            * 100
            / 100
            + 5) as i16,
        speed: ((2 * dex_entry.base_stats.speed + 31 + (packed.evs[5] as u16 / 4)) * 100 / 100 + 5)
            as i16,
        status: PokemonStatus::NONE,
        rest_turns: 0,
        sleep_turns: 0,
        weight_kg: dex_entry.weightkg,
        terastallized: false,
        tera_type: PokemonType::from_str(&normalize_name(&packed.tera_type))
            .unwrap_or(PokemonType::NORMAL),
        moves,
    }
}

pub fn initialize_battle_state(
    random_teams_json: &str,
    pokedex_json: &str,
    movedex_json: &str,
) -> State {
    let random_teams: Vec<PackedTeamEntry> =
        serde_json::from_str(random_teams_json).expect("Failed to parse random teams JSON");

    let pokedex: HashMap<String, PokemonDexEntry> =
        serde_json::from_str(pokedex_json).expect("Failed to parse pokedex JSON");

    let movedex: HashMap<String, MoveDexEntry> =
        serde_json::from_str(movedex_json).expect("Failed to parse movedex JSON");

    // Team selection logic remains the same
    use rand::seq::SliceRandom;
    let mut rng = rand::thread_rng();
    let selected_teams: Vec<_> = random_teams.choose_multiple(&mut rng, 2).collect();

    let team1 = parse_packed_team(&selected_teams[0].packed_team);
    let team2 = parse_packed_team(&selected_teams[1].packed_team);

    assert_eq!(team1.len(), 6, "Team 1 doesn't have 6 Pokemon: {:?}", team1);
    assert_eq!(team2.len(), 6, "Team 2 doesn't have 6 Pokemon: {:?}", team2);

    // Create Pokemon with movedex
    let side_one_pokemon = SidePokemon {
        p0: create_pokemon(&team1[0], &pokedex, &movedex),
        p1: create_pokemon(&team1[1], &pokedex, &movedex),
        p2: create_pokemon(&team1[2], &pokedex, &movedex),
        p3: create_pokemon(&team1[3], &pokedex, &movedex),
        p4: create_pokemon(&team1[4], &pokedex, &movedex),
        p5: create_pokemon(&team1[5], &pokedex, &movedex),
    };

    let side_two_pokemon = SidePokemon {
        p0: create_pokemon(&team2[0], &pokedex, &movedex),
        p1: create_pokemon(&team2[1], &pokedex, &movedex),
        p2: create_pokemon(&team2[2], &pokedex, &movedex),
        p3: create_pokemon(&team2[3], &pokedex, &movedex),
        p4: create_pokemon(&team2[4], &pokedex, &movedex),
        p5: create_pokemon(&team2[5], &pokedex, &movedex),
    };

    // Rest of the state initialization remains the same
    let mut side_one = Side::new_singles();
    side_one.reserve = side_one_pokemon;
    side_one.pokemon = side_one.reserve.clone();

    let mut side_two = Side::new_singles();
    side_two.reserve = side_two_pokemon;
    side_two.pokemon = side_two.reserve.clone();

    State {
        format: BattleFormat::Singles,
        side_one,
        side_two,
        weather: StateWeather {
            weather_type: Weather::NONE,
            turns_remaining: 0,
        },
        terrain: StateTerrain {
            terrain_type: Terrain::NONE,
            turns_remaining: 0,
        },
        trick_room: StateTrickRoom {
            active: false,
            turns_remaining: 0,
        },
        team_preview: false,
        use_last_used_move: false,
        use_damage_dealt: false,
    }
}

// Player trait for different agent types
pub trait Player: Send + Sync + 'static {
    fn choose_move(
        &self,
        state: &State,
        side_ref: SideReference,
        options: &[MoveChoice],
    ) -> MoveChoice;
    fn name(&self) -> &str;
}

// Random player implementation
pub struct RandomPlayer {
    name: String,
}

impl RandomPlayer {
    pub fn new(name: String) -> Self {
        RandomPlayer { name }
    }
}

impl Player for RandomPlayer {
    fn choose_move(
        &self,
        _state: &State,
        _side_ref: SideReference,
        options: &[MoveChoice],
    ) -> MoveChoice {
        let mut rng = thread_rng();
        options[rng.gen_range(0..options.len())]
    }

    fn name(&self) -> &str {
        &self.name
    }
}

// First move player - always picks the first available move
pub struct FirstMovePlayer {
    name: String,
}

impl FirstMovePlayer {
    pub fn new(name: String) -> Self {
        FirstMovePlayer { name }
    }
}

impl Player for FirstMovePlayer {
    fn choose_move(
        &self,
        _state: &State,
        _side_ref: SideReference,
        options: &[MoveChoice],
    ) -> MoveChoice {
        options[0]
    }

    fn name(&self) -> &str {
        &self.name
    }
}

// Simple damage maximizer - picks the move that would do most damage
pub struct DamageMaximizer {
    name: String,
}

impl DamageMaximizer {
    pub fn new(name: String) -> Self {
        DamageMaximizer { name }
    }

    fn estimate_damage(
        &self,
        state: &State,
        side_ref: SideReference,
        move_choice: &MoveChoice,
    ) -> f32 {
        match move_choice {
            MoveChoice::Move(move_index) | MoveChoice::MoveTera(move_index) => {
                let side = match side_ref {
                    SideReference::SideOne => &state.side_one,
                    SideReference::SideTwo => &state.side_two,
                };
                let active = side.get_active_immutable();
                let move_data = &active.moves[move_index];

                // Get move info from MOVES constant
                if let Some(move_info) = crate::choices::MOVES.get(&move_data.id) {
                    // Simple damage estimate based on base power
                    move_info.base_power as f32
                } else {
                    0.0
                }
            }
            MoveChoice::Switch(_) => -10.0, // Slight penalty for switching
            MoveChoice::None => 0.0,
        }
    }
}

impl Player for DamageMaximizer {
    fn choose_move(
        &self,
        state: &State,
        side_ref: SideReference,
        options: &[MoveChoice],
    ) -> MoveChoice {
        let mut best_move = options[0];
        let mut best_damage = self.estimate_damage(state, side_ref, &options[0]);

        for option in options.iter().skip(1) {
            let damage = self.estimate_damage(state, side_ref, option);
            if damage > best_damage {
                best_damage = damage;
                best_move = *option;
            }
        }

        best_move
    }

    fn name(&self) -> &str {
        &self.name
    }
}

// MCTS player - uses Monte Carlo Tree Search to choose moves
pub struct MctsPlayer {
    name: String,
    search_time_ms: u64,
}

impl MctsPlayer {
    pub fn new(name: String, search_time_ms: u64) -> Self {
        MctsPlayer {
            name,
            search_time_ms,
        }
    }
}

impl Player for MctsPlayer {
    fn choose_move(
        &self,
        state: &State,
        side_ref: SideReference,
        options: &[MoveChoice],
    ) -> MoveChoice {
        // Clone the state for MCTS
        let mut mcts_state = state.clone();

        // Get both sides' options
        let (side_one_options, side_two_options) = mcts_state.get_all_options();

        // Run MCTS
        let mcts_result = perform_mcts(
            &mut mcts_state,
            side_one_options.clone(),
            side_two_options.clone(),
            Duration::from_millis(self.search_time_ms),
        );

        // Log MCTS results
        if let Ok(log_file_path) = std::env::var("BATTLE_LOG_FILE") {
            use std::fs::OpenOptions;
            use std::io::Write;
            match OpenOptions::new().create(true).append(true).open(&log_file_path) {
                Ok(mut file) => {
                    writeln!(file, "\n=== STANDARD MCTS RESULTS ===").ok();
                    writeln!(file, "Standard MCTS ({}) - Iterations: {}", self.name, mcts_result.iteration_count).ok();
                writeln!(file, "Move evaluations:").ok();
                
                // Get the results for this side
                let results = match side_ref {
                    SideReference::SideOne => &mcts_result.s1,
                    SideReference::SideTwo => &mcts_result.s2,
                };
                
                // Sort by visit count (descending)
                let mut sorted_results = results.clone();
                sorted_results.sort_by(|a, b| b.visits.cmp(&a.visits));
                
                // Get the side for move string conversion
                let side = match side_ref {
                    SideReference::SideOne => &state.side_one,
                    SideReference::SideTwo => &state.side_two,
                };
                
                for result in &sorted_results {
                    let avg_score = if result.visits > 0 {
                        result.total_score / result.visits as f32
                    } else {
                        0.0
                    };
                    
                    writeln!(file, "  {:<20} - visits: {:>6}, avg: {:>5.3}, total: {:>8.1}",
                        result.move_choice.to_string(side),
                        result.visits,
                        avg_score,
                        result.total_score
                    ).ok();
                }
                
                // Highlight the selected move
                if let Some(best_move) = sorted_results.first() {
                    writeln!(file, "\n>>> SELECTED: {} (most visits) <<<", best_move.move_choice.to_string(side)).ok();
                }
                    writeln!(file, "==================\n").ok();
                }
                Err(_) => {}
            }
        }
        // eprintln!(
        //     "Standard MCTS ({}) - Iterations: {}",
        //     self.name, mcts_result.iteration_count
        // );

        // Select the move with the most visits
        let results = match side_ref {
            SideReference::SideOne => &mcts_result.s1,
            SideReference::SideTwo => &mcts_result.s2,
        };

        // Find the move with the most visits
        let best_move = results
            .iter()
            .max_by_key(|result| result.visits)
            .map(|result| result.move_choice)
            .unwrap_or(options[0]);

        best_move
    }

    fn name(&self) -> &str {
        &self.name
    }
}

// Battle result
#[derive(Debug, Clone)]
pub struct BattleResult {
    pub winner: Option<SideReference>,
    pub turn_count: usize,
    pub final_state: State,
    pub turn_history: Vec<TurnInfo>,
}

// Turn information for observability
#[derive(Debug, Clone)]
pub struct TurnInfo {
    pub turn_number: usize,
    pub state_before: State,
    pub side_one_choice: MoveChoice,
    pub side_two_choice: MoveChoice,
    pub instructions_generated: Vec<StateInstructions>,
    pub state_after: State,
}

// Battle environment
pub struct BattleEnvironment {
    pub player_one: Box<dyn Player>,
    pub player_two: Box<dyn Player>,
    pub max_turns: usize,
    pub verbose: bool,
    pub log_file: Option<String>,
}

impl BattleEnvironment {
    pub fn new(
        player_one: Box<dyn Player>,
        player_two: Box<dyn Player>,
        max_turns: usize,
        verbose: bool,
    ) -> Self {
        BattleEnvironment {
            player_one,
            player_two,
            max_turns,
            verbose,
            log_file: None,
        }
    }

    pub fn with_log_file(mut self, log_file: String) -> Self {
        self.log_file = Some(log_file);
        self
    }

    // Generate initial switch-in instructions
    fn generate_initial_instructions(state: &mut State) -> Vec<StateInstructions> {
        // Both players' first Pokemon are already active (P0)
        // We need to generate initial instructions for abilities like Intimidate, weather, etc.
        // We'll simulate this by having both sides "do nothing" which will trigger any
        // start-of-battle effects
        let no_move_s1 = MoveChoice::None;
        let no_move_s2 = MoveChoice::None;

        generate_instructions_from_move_pair(
            state,
            &no_move_s1,
            &no_move_s2,
            true, // branch on damage
        )
    }

    // Run a complete battle
    pub fn run_battle(&self, initial_state: State) -> BattleResult {
        let mut state = initial_state.clone();
        let mut turn_history = Vec::new();
        let mut turn_count = 0;

        // Create log file if verbose using OpenOptions for consistency
        let mut log_file = if self.verbose && self.log_file.is_some() {
            use std::fs::OpenOptions;
            Some(OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(self.log_file.as_ref().unwrap())
                .expect("Failed to create log file"))
        } else {
            None
        };
        
        // Set environment variable for players to use AFTER creating the file
        if self.log_file.is_some() {
            std::env::set_var("BATTLE_LOG_FILE", self.log_file.as_ref().unwrap());
        }

        // Apply initial switch-in effects
        if self.verbose {
            let start_msg = format!(
                "\n=== Battle Starting ===\nPlayer 1 ({}) vs Player 2 ({})\n",
                self.player_one.name(),
                self.player_two.name()
            );

            if let Some(ref mut file) = log_file {
                writeln!(file, "{}", start_msg).unwrap();
                file.flush().unwrap();
            } else {
                println!("{}", start_msg);
            }
        }

        // Generate and apply initial switch-in instructions
        let initial_instructions = Self::generate_initial_instructions(&mut state);
        if !initial_instructions.is_empty() {
            let chosen_index = self.sample_instruction_index(&initial_instructions);
            state.apply_instructions(&initial_instructions[chosen_index].instruction_list);
        }

        // Main battle loop
        while state.battle_is_over() == 0.0 && turn_count < self.max_turns {
            turn_count += 1;
            let state_before = state.clone();

            // Get available options for both players
            let (side_one_options, side_two_options) = state.get_all_options();

            // Debug: Check if we have any options
            if side_one_options.is_empty() || side_two_options.is_empty() {
                if self.verbose {
                    if let Some(ref mut file) = log_file {
                        writeln!(file, "WARNING: No options available for one or both sides!").ok();
                    }
                }
                break;
            }

            // Write turn header and state BEFORE players make moves
            if self.verbose {
                let turn_header = format!(
                    "\n========== Turn {} ==========\n{}\n\nSerialized State:\n{}\n",
                    turn_count,
                    state.pprint(),
                    state.serialize()
                );

                if let Some(ref mut file) = log_file {
                    write!(file, "{}", turn_header).unwrap();
                    file.flush().unwrap();
                } else {
                    print!("{}", turn_header);
                }
            }
            
            // Close the log file so players can append to it
            if log_file.is_some() {
                drop(log_file.take());
            }

            // Now players choose their moves (and write MCTS results)
            let side_one_choice =
                self.player_one
                    .choose_move(&state, SideReference::SideOne, &side_one_options);
            let side_two_choice =
                self.player_two
                    .choose_move(&state, SideReference::SideTwo, &side_two_options);

            // Reopen log file to write the selected moves
            if self.verbose && self.log_file.is_some() {
                use std::fs::OpenOptions;
                log_file = Some(OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(self.log_file.as_ref().unwrap())
                    .expect("Failed to reopen log file"));
                    
                let moves_msg = format!(
                    "\nMoves Selected:\n  Side 1: {}\n  Side 2: {}\n=============================\n",
                    side_one_choice.to_string(&state.side_one),
                    side_two_choice.to_string(&state.side_two)
                );

                if let Some(ref mut file) = log_file {
                    write!(file, "{}", moves_msg).unwrap();
                    file.flush().unwrap();
                }
            }

            // Generate instructions from the move pair
            let instructions = generate_instructions_from_move_pair(
                &mut state,
                &side_one_choice,
                &side_two_choice,
                true, // branch on damage
            );

            // Apply the instructions (sampling from possibilities)
            if !instructions.is_empty() {
                let chosen_index = self.sample_instruction_index(&instructions);
                state.apply_instructions(&instructions[chosen_index].instruction_list);
            }

            // Record turn information
            turn_history.push(TurnInfo {
                turn_number: turn_count,
                state_before,
                side_one_choice,
                side_two_choice,
                instructions_generated: instructions,
                state_after: state.clone(),
            });
        }

        // Determine winner
        let battle_result = state.battle_is_over();
        let winner = if battle_result > 0.0 {
            Some(SideReference::SideOne)
        } else if battle_result < 0.0 {
            Some(SideReference::SideTwo)
        } else {
            None
        };

        if self.verbose {
            let end_msg = format!(
                "\n=== Battle Ended ===\n{}\nTotal turns: {}\n",
                match winner {
                    Some(SideReference::SideOne) =>
                        format!("Player 1 ({}) wins!", self.player_one.name()),
                    Some(SideReference::SideTwo) =>
                        format!("Player 2 ({}) wins!", self.player_two.name()),
                    None => "Battle ended in a draw (turn limit reached)".to_string(),
                },
                turn_count
            );

            if let Some(ref mut file) = log_file {
                write!(file, "{}", end_msg).unwrap();
                file.flush().unwrap();
            } else {
                print!("{}", end_msg);
            }
        }

        // Clear the environment variable
        if self.log_file.is_some() {
            std::env::remove_var("BATTLE_LOG_FILE");
        }
        
        BattleResult {
            winner,
            turn_count,
            final_state: state,
            turn_history,
        }
    }

    // Sample from possible instruction outcomes based on their probabilities
    fn sample_instruction_index(&self, state_instructions: &[StateInstructions]) -> usize {
        if state_instructions.len() == 1 {
            return 0;
        }

        let mut rng = thread_rng();
        let total_percentage: f32 = state_instructions.iter().map(|si| si.percentage).sum();
        let mut random_value = rng.gen::<f32>() * total_percentage;

        for (index, si) in state_instructions.iter().enumerate() {
            random_value -= si.percentage;
            if random_value <= 0.0 {
                return index;
            }
        }

        state_instructions.len() - 1
    }
}

// Helper function to create a battle from JSON files and run it
pub fn run_random_battle(
    random_teams_json: &str,
    pokedex_json: &str,
    movedex_json: &str,
    player_one: Box<dyn Player>,
    player_two: Box<dyn Player>,
    max_turns: usize,
    verbose: bool,
) -> BattleResult {
    let initial_state =
        initialize_battle_state(random_teams_json, pokedex_json, movedex_json).clone();
    let env = BattleEnvironment::new(player_one, player_two, max_turns, verbose);
    env.run_battle(initial_state)
}

// Parallel battle execution
#[derive(Debug)]
pub struct ParallelBattleResults {
    pub player_one_wins: usize,
    pub player_two_wins: usize,
    pub draws: usize,
    pub total_battles: usize,
}

pub fn run_parallel_battles_with_states<F1, F2>(
    battle_states: Vec<State>,
    num_threads: usize,
    player_one_factory: F1,
    player_two_factory: F2,
    max_turns: usize,
) -> ParallelBattleResults
where
    F1: Fn() -> Box<dyn Player> + Send + Sync + 'static,
    F2: Fn() -> Box<dyn Player> + Send + Sync + 'static,
{
    let num_battles = battle_states.len();
    let battle_states = Arc::new(battle_states);
    
    let player_one_factory = Arc::new(player_one_factory);
    let player_two_factory = Arc::new(player_two_factory);
    let results = Arc::new(Mutex::new(ParallelBattleResults {
        player_one_wins: 0,
        player_two_wins: 0,
        draws: 0,
        total_battles: 0,
    }));

    let battles_per_thread = num_battles / num_threads;
    let remainder = num_battles % num_threads;

    let handles: Vec<_> = (0..num_threads)
        .map(|thread_id| {
            let p1_factory = Arc::clone(&player_one_factory);
            let p2_factory = Arc::clone(&player_two_factory);
            let results = Arc::clone(&results);
            let states = Arc::clone(&battle_states);

            let thread_start = thread_id * battles_per_thread + thread_id.min(remainder);
            let thread_battles = if thread_id < remainder {
                battles_per_thread + 1
            } else {
                battles_per_thread
            };

            thread::spawn(move || {
                for i in 0..thread_battles {
                    let state_idx = thread_start + i;
                    let initial_state = states[state_idx].clone();
                    
                    let env = BattleEnvironment::new(
                        p1_factory(),
                        p2_factory(),
                        max_turns,
                        false, // Not verbose for parallel runs
                    );

                    let result = env.run_battle(initial_state);

                    let mut results = results.lock().unwrap();
                    results.total_battles += 1;
                    match result.winner {
                        Some(SideReference::SideOne) => results.player_one_wins += 1,
                        Some(SideReference::SideTwo) => results.player_two_wins += 1,
                        None => results.draws += 1,
                    }
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    Arc::try_unwrap(results).unwrap().into_inner().unwrap()
}

// Keep the old function for backwards compatibility
pub fn run_parallel_battles<F1, F2>(
    num_battles: usize,
    num_threads: usize,
    player_one_factory: F1,
    player_two_factory: F2,
    max_turns: usize,
    random_teams_json: &str,
    pokedex_json: &str,
    movedex_json: &str,
) -> ParallelBattleResults
where
    F1: Fn() -> Box<dyn Player> + Send + Sync + 'static,
    F2: Fn() -> Box<dyn Player> + Send + Sync + 'static,
{
    let player_one_factory = Arc::new(player_one_factory);
    let player_two_factory = Arc::new(player_two_factory);
    let results = Arc::new(Mutex::new(ParallelBattleResults {
        player_one_wins: 0,
        player_two_wins: 0,
        draws: 0,
        total_battles: 0,
    }));

    let battles_per_thread = num_battles / num_threads;
    let remainder = num_battles % num_threads;

    let random_teams_json = Arc::new(random_teams_json.to_string());
    let pokedex_json = Arc::new(pokedex_json.to_string());
    let movedex_json = Arc::new(movedex_json.to_string());

    let handles: Vec<_> = (0..num_threads)
        .map(|thread_id| {
            let p1_factory = Arc::clone(&player_one_factory);
            let p2_factory = Arc::clone(&player_two_factory);
            let results = Arc::clone(&results);
            let teams_json = Arc::clone(&random_teams_json);
            let pdex_json = Arc::clone(&pokedex_json);
            let mdex_json = Arc::clone(&movedex_json);

            let thread_battles = if thread_id < remainder {
                battles_per_thread + 1
            } else {
                battles_per_thread
            };

            thread::spawn(move || {
                for _ in 0..thread_battles {
                    let initial_state =
                        initialize_battle_state(&teams_json, &pdex_json, &mdex_json);
                    let env = BattleEnvironment::new(
                        p1_factory(),
                        p2_factory(),
                        max_turns,
                        false, // Not verbose for parallel runs
                    );

                    let result = env.run_battle(initial_state);

                    let mut results = results.lock().unwrap();
                    match result.winner {
                        Some(SideReference::SideOne) => results.player_one_wins += 1,
                        Some(SideReference::SideTwo) => results.player_two_wins += 1,
                        None => results.draws += 1,
                    }
                    results.total_battles += 1;
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    Arc::try_unwrap(results).unwrap().into_inner().unwrap()
}
