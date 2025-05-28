use crate::choices::Choice;
use crate::engine::generate_instructions::moves_first;
use crate::instruction::StateInstructions;
use crate::matchup_calc::{
    analyze_passive_effects, analyze_setup_capabilities, calculate_boosted_damage,
    calculate_boosted_damage_optimal, classify_matchup_by_category, classify_matchup_result,
    find_best_move, get_best_priority_move_damage, get_damage_range, get_matchup_category,
    get_second_best_move_damage, get_stat_drop_percentage, is_setup_viable, is_stat_lowering_move,
};
use crate::matchup_mcts::create_simulation_state;
use crate::matchup_mcts::BattleConditions;
use crate::state::SideMovesFirst;
use crate::state::{PokemonIndex, SideReference, State};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;

/// Structure to track reasoning steps during matchup analysis
#[derive(Debug, Clone)]
pub struct MatchupReasoning {
    pub s1_name: String,
    pub s2_name: String,
    pub metrics: MatchupMetrics,
    pub win_percentage: f32,
    pub classification: i8,
    pub reasoning_steps: Vec<String>,
    pub primary_reason: String,
}

/// Stores all the computed metrics for the matchup
#[derive(Debug, Clone)]
pub struct MatchupMetrics {
    // Basic Pokemon info
    pub s1_hp: i16,
    pub s1_max_hp: i16,
    pub s2_hp: i16,
    pub s2_max_hp: i16,
    pub s1_moves: Vec<String>, // Added to store move sets
    pub s2_moves: Vec<String>, // Added to store move sets

    // Moves and damage
    pub s1_best_move: String,
    pub s2_best_move: String,
    pub s1_avg_damage: i16,
    pub s1_max_damage: i16,
    pub s2_avg_damage: i16,
    pub s2_max_damage: i16,

    // KO potential
    pub s1_ohko_chance: f32,
    pub s2_ohko_chance: f32,
    pub s1_turns_to_ko: i32,
    pub s2_turns_to_ko: i32,

    // Priority
    pub s1_priority_damage: i16,
    pub s2_priority_damage: i16,
    pub s1_priority_ko: bool,
    pub s2_priority_ko: bool,

    // Recovery
    pub s1_recovery_per_turn: i16,
    pub s2_recovery_per_turn: i16,
    pub s1_recovery_sufficient: bool,
    pub s1_recovery_dominates: bool,
    pub s2_recovery_sufficient: bool,
    pub s2_recovery_dominates: bool,

    // Setup
    pub s1_has_setup: bool,
    pub s2_has_setup: bool,
    pub s1_can_setup_safely: bool,
    pub s2_can_setup_safely: bool,
    pub s1_boosts_speed: bool,
    pub s2_boosts_speed: bool,
    pub s1_setup_ohko: bool,
    pub s2_setup_ohko: bool,

    // Speed and turn order
    pub s1_moves_first: bool,
    pub speed_tie: bool,

    // Advanced metrics for mathematical model
    pub s1_boosted_damage: i16,
    pub s2_boosted_damage: i16,
    pub s1_setup_viable: bool,
    pub s2_setup_viable: bool,
    pub s1_setup_turns: i32,
    pub s2_setup_turns: i32,
    pub s1_recovery_strategy: (bool, i32, f32), // (sustainable, turns_to_ko, optimal_frequency)
    pub s2_recovery_strategy: (bool, i32, f32), // (sustainable, turns_to_ko, optimal_frequency)
    pub s1_best_ko_turns: i32,
    pub s2_best_ko_turns: i32,
    pub s1_best_strategy: String,
    pub s2_best_strategy: String,

    pub s1_has_stat_lowering_move: bool,
    pub s2_has_stat_lowering_move: bool,
    pub s1_stat_drop_percentage: f32,
    pub s2_stat_drop_percentage: f32,
    pub s1_second_best_damage: i16,
    pub s2_second_best_damage: i16,
    pub s1_post_drop_damage: i16,
    pub s2_post_drop_damage: i16,

    // New passive effects fields
    pub s1_has_active_recovery: bool,
    pub s2_has_active_recovery: bool,
    pub s1_passive_recovery_amount: i16,
    pub s2_passive_recovery_amount: i16,
    pub s1_passive_damage_outgoing: i16,
    pub s2_passive_damage_outgoing: i16,
    pub s1_passive_damage_incoming: i16,
    pub s2_passive_damage_incoming: i16,
}

impl MatchupReasoning {
    /// Create a new MatchupReasoning instance
    pub fn new(state: &State, s1_idx: PokemonIndex, s2_idx: PokemonIndex) -> Self {
        MatchupReasoning {
            s1_name: state.side_one.pokemon[s1_idx].id.to_string(),
            s2_name: state.side_two.pokemon[s2_idx].id.to_string(),
            metrics: MatchupMetrics::default(),
            win_percentage: 0.0,
            classification: 0,
            reasoning_steps: Vec::new(),
            primary_reason: String::new(),
        }
    }

    /// Add a reasoning step with context
    pub fn add_step(&mut self, step: String) {
        self.reasoning_steps.push(step);
    }

    /// Set the primary reason for the final classification
    pub fn set_primary_reason(&mut self, reason: String) {
        self.primary_reason = reason;
    }

    /// Update the win percentage and classification
    pub fn set_result(&mut self, classification: i8) {
        self.classification = classification;
    }

    /// Save the detailed reasoning to a file (useful for batch analysis)
    pub fn save_to_file(&self, file_path: &str) -> std::io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(file_path)?;

        writeln!(
            file,
            "=== MATCHUP: {} vs {} ===",
            self.s1_name, self.s2_name
        )?;
        writeln!(file, "Classification: {}", self.classification_to_string())?;
        writeln!(file, "Primary Reason: {}", self.primary_reason)?;

        writeln!(file, "\n-- KEY METRICS --")?;
        self.metrics.write_to_file(&mut file)?;

        writeln!(file, "\n-- REASONING STEPS --")?;
        for (i, step) in self.reasoning_steps.iter().enumerate() {
            writeln!(file, "{}. {}", i + 1, step)?;
        }

        writeln!(file, "\n\n")?;
        Ok(())
    }

    /// Convert classification value to string description
    pub fn classification_to_string(&self) -> String {
        match self.classification {
            2 => "COUNTER (Strong Favorable)".to_string(),
            1 => "CHECK (Favorable)".to_string(),
            0 => "NEUTRAL".to_string(),
            -1 => "CHECKED (Unfavorable)".to_string(),
            -2 => "COUNTERED (Strong Unfavorable)".to_string(),
            _ => format!("Unknown ({})", self.classification),
        }
    }

    /// Generate a human-readable summary of the matchup
    pub fn summary(&self) -> String {
        let s1_moves_str = if self.metrics.s1_moves.is_empty() {
            "No moves".to_string()
        } else {
            format!("Moves: {}", self.metrics.s1_moves.join(", "))
        };

        let s2_moves_str = if self.metrics.s2_moves.is_empty() {
            "No moves".to_string()
        } else {
            format!("Moves: {}", self.metrics.s2_moves.join(", "))
        };

        let result = format!(
            "{} ({}) vs {} ({}) - {} (Win rate: {:.1}%)\n  Primary reason: {}\n  Key metrics: S1 damage: {}, S2 damage: {}, S1 TTK: {}, S2 TTK: {}, {} moves first",
            self.s1_name,
            s1_moves_str,
            self.s2_name,
            s2_moves_str,
            self.classification_to_string(),
            self.win_percentage * 100.0,
            self.primary_reason,
            self.metrics.s1_avg_damage,
            self.metrics.s2_avg_damage,
            self.metrics.s1_turns_to_ko,
            self.metrics.s2_turns_to_ko,
            if self.metrics.s1_moves_first { self.s1_name.clone() } else { self.s2_name.clone() }
        );
        result
    }
}

impl Default for MatchupMetrics {
    fn default() -> Self {
        MatchupMetrics {
            s1_hp: 0,
            s1_max_hp: 0,
            s2_hp: 0,
            s2_max_hp: 0,
            s1_moves: Vec::new(), // Initialize empty move sets
            s2_moves: Vec::new(), // Initialize empty move sets
            s1_best_move: String::new(),
            s2_best_move: String::new(),
            s1_avg_damage: 0,
            s1_max_damage: 0,
            s2_avg_damage: 0,
            s2_max_damage: 0,
            s1_ohko_chance: 0.0,
            s2_ohko_chance: 0.0,
            s1_turns_to_ko: 0,
            s2_turns_to_ko: 0,
            s1_priority_damage: 0,
            s2_priority_damage: 0,
            s1_priority_ko: false,
            s2_priority_ko: false,
            s1_recovery_per_turn: 0,
            s2_recovery_per_turn: 0,
            s1_recovery_sufficient: false,
            s1_recovery_dominates: false,
            s2_recovery_sufficient: false,
            s2_recovery_dominates: false,
            s1_has_setup: false,
            s2_has_setup: false,
            s1_can_setup_safely: false,
            s2_can_setup_safely: false,
            s1_boosts_speed: false,
            s2_boosts_speed: false,
            s1_setup_ohko: false,
            s2_setup_ohko: false,
            s1_moves_first: false,
            speed_tie: false,
            s1_boosted_damage: 0,
            s2_boosted_damage: 0,
            s1_setup_viable: false,
            s2_setup_viable: false,
            s1_setup_turns: 0,
            s2_setup_turns: 0,
            s1_recovery_strategy: (false, 0, 0.0),
            s2_recovery_strategy: (false, 0, 0.0),
            s1_best_ko_turns: 0,
            s2_best_ko_turns: 0,
            s1_best_strategy: String::new(),
            s2_best_strategy: String::new(),
            s1_has_stat_lowering_move: false,
            s2_has_stat_lowering_move: false,
            s1_stat_drop_percentage: 1.0,
            s2_stat_drop_percentage: 1.0,
            s1_second_best_damage: 0,
            s2_second_best_damage: 0,
            s1_post_drop_damage: 0,
            s2_post_drop_damage: 0,
            s1_has_active_recovery: false,
            s2_has_active_recovery: false,
            s1_passive_recovery_amount: 0,
            s2_passive_recovery_amount: 0,
            s1_passive_damage_outgoing: 0,
            s2_passive_damage_outgoing: 0,
            s1_passive_damage_incoming: 0,
            s2_passive_damage_incoming: 0,
        }
    }
}

impl MatchupMetrics {
    /// Write metrics to file in a readable format
    fn write_to_file(&self, file: &mut File) -> std::io::Result<()> {
        writeln!(
            file,
            "HP: {} ({}/{}) vs {} ({}/{})",
            self.s1_hp, self.s1_hp, self.s1_max_hp, self.s2_hp, self.s2_hp, self.s2_max_hp
        )?;

        writeln!(
            file,
            "Best Moves: {} vs {}",
            self.s1_best_move, self.s2_best_move
        )?;

        // Write move sets
        writeln!(file, "Side 1 Moves: {}", self.s1_moves.join(", "))?;
        writeln!(file, "Side 2 Moves: {}", self.s2_moves.join(", "))?;

        writeln!(
            file,
            "Damage: Avg={}/Max={} vs Avg={}/Max={}",
            self.s1_avg_damage, self.s1_max_damage, self.s2_avg_damage, self.s2_max_damage
        )?;

        writeln!(
            file,
            "OHKO Chance: {:.1}% vs {:.1}%",
            self.s1_ohko_chance * 100.0,
            self.s2_ohko_chance * 100.0
        )?;

        writeln!(
            file,
            "Turns to KO: {} vs {}",
            self.s1_turns_to_ko, self.s2_turns_to_ko
        )?;

        writeln!(
            file,
            "Priority: Damage={} (KO={}) vs Damage={} (KO={})",
            self.s1_priority_damage,
            self.s1_priority_ko,
            self.s2_priority_damage,
            self.s2_priority_ko
        )?;

        writeln!(file, "Recovery: {}/turn (Sufficient={}, Dominates={}) vs {}/turn (Sufficient={}, Dominates={})",
            self.s1_recovery_per_turn, self.s1_recovery_sufficient, self.s1_recovery_dominates,
            self.s2_recovery_per_turn, self.s2_recovery_sufficient, self.s2_recovery_dominates)?;

        writeln!(
            file,
            "Setup: Has={} (Safe={}, Speed={}, OHKO={}) vs Has={} (Safe={}, Speed={}, OHKO={})",
            self.s1_has_setup,
            self.s1_can_setup_safely,
            self.s1_boosts_speed,
            self.s1_setup_ohko,
            self.s2_has_setup,
            self.s2_can_setup_safely,
            self.s2_boosts_speed,
            self.s2_setup_ohko
        )?;

        writeln!(
            file,
            "Speed: S1 Moves First={}, Speed Tie={}",
            self.s1_moves_first, self.speed_tie
        )?;

        // Write advanced metrics
        writeln!(file, "\nAdvanced Metrics (Mathematical Model):")?;

        writeln!(
            file,
            "Optimal Strategy: S1={} (in {} turns) vs S2={} (in {} turns)",
            self.s1_best_strategy,
            self.s1_best_ko_turns,
            self.s2_best_strategy,
            self.s2_best_ko_turns
        )?;

        writeln!(
            file,
            "Setup Boosted Damage: S1={} (Viable={}) vs S2={} (Viable={})",
            self.s1_boosted_damage,
            self.s1_setup_viable,
            self.s2_boosted_damage,
            self.s2_setup_viable
        )?;

        writeln!(
            file,
            "Recovery Strategy: S1 (Sustainable={}, OptFreq={:.2}, TTK={}) vs S2 (Sustainable={}, OptFreq={:.2}, TTK={})",
            self.s1_recovery_strategy.0, self.s1_recovery_strategy.2, self.s1_recovery_strategy.1,
            self.s2_recovery_strategy.0, self.s2_recovery_strategy.2, self.s2_recovery_strategy.1
        )?;

        // Add this section to write the stat-lowering move information
        if self.s1_has_stat_lowering_move || self.s2_has_stat_lowering_move {
            writeln!(
                        file,
                        "\nStat-Lowering Moves: S1={} (Drop={:.2}, Post={}, Second={}), S2={} (Drop={:.2}, Post={}, Second={})",
                        self.s1_has_stat_lowering_move,
                        self.s1_stat_drop_percentage,
                        self.s1_post_drop_damage,
                        self.s1_second_best_damage,
                        self.s2_has_stat_lowering_move,
                        self.s2_stat_drop_percentage,
                        self.s2_post_drop_damage,
                        self.s2_second_best_damage
                    )?;
        }

        Ok(())
    }
}

// Helper function to convert a choice to a string
pub fn choice_to_string(choice: &Choice) -> String {
    format!("{:?}", choice.move_id).to_lowercase()
}

/// Calculate a recovery strategy
/// Returns (sustainable, turns_to_ko, optimal_frequency)
fn calculate_recovery_strategy(
    incoming_damage: i16,
    recovery_amount: i16,
    recovery_pp: i8,
    outgoing_damage: i16,
    target_hp: i16,
    is_faster: bool,
) -> (bool, i32, f32) {
    if recovery_amount <= 0 || recovery_pp <= 0 || incoming_damage <= 0 || outgoing_damage <= 0 {
        return (false, 99, 0.0);
    }

    // Calculate theoretical frequency needed to sustain
    let theoretical_frequency = incoming_damage as f32 / recovery_amount as f32;

    // Adjust based on turn order
    let adjusted_frequency = if is_faster {
        theoretical_frequency
    } else {
        // When slower, we need more recovery
        theoretical_frequency * 1.2
    };

    // Recovery is sustainable if we can attack at least 50% of the time
    let is_sustainable = adjusted_frequency <= 0.5;

    // If not sustainable, return negative result
    if !is_sustainable {
        return (false, 99, 0.0);
    }

    // Optimal frequency is at least what's needed, but no more than 0.5
    let optimal_frequency = adjusted_frequency.min(0.5).max(0.1);

    // Calculate effective damage output while recovering
    let effective_damage_output = outgoing_damage as f32 * (1.0 - optimal_frequency);

    // Calculate turns to KO with recovery
    let theoretical_turns = (target_hp as f32 / effective_damage_output).ceil() as i32;

    // Check if we have enough PP
    let recovery_uses_needed = (theoretical_turns as f32 * optimal_frequency).ceil() as i16;

    if recovery_uses_needed <= recovery_pp as i16 {
        // We have enough PP for the entire battle
        (true, theoretical_turns, optimal_frequency)
    } else {
        // Limited by recovery PP
        let sustainable_turns = (recovery_pp as f32 / optimal_frequency).ceil() as i32;
        let damage_dealt =
            (sustainable_turns as f32 * (1.0 - optimal_frequency) * outgoing_damage as f32) as i16;

        if damage_dealt >= target_hp {
            // Can KO within PP constraints
            (true, sustainable_turns, optimal_frequency)
        } else {
            // Not enough PP to KO
            (false, 99, optimal_frequency)
        }
    }
}

// Modified compute_matchup function that returns detailed reasoning
pub fn compute_matchup_with_reasoning(
    state: &State,
    s1_idx: PokemonIndex,
    s2_idx: PokemonIndex,
    conditions: &BattleConditions,
) -> (i8, MatchupReasoning) {
    // Create a simulation state with the specified conditions
    let mut sim_state = create_simulation_state(state, s1_idx, s2_idx, conditions);

    // Initialize reasoning tracker
    let mut reasoning = MatchupReasoning::new(state, s1_idx, s2_idx);

    // Capture move sets for both Pokémon
    let s1_moves: Vec<String> = state.side_one.pokemon[s1_idx]
        .moves
        .into_iter()
        .map(|m| format!("{:?}", m.id).to_lowercase())
        .filter(|x| x != "none")
        .collect();

    let s2_moves: Vec<String> = state.side_two.pokemon[s2_idx]
        .moves
        .into_iter()
        .map(|m| format!("{:?}", m.id).to_lowercase())
        .filter(|x| x != "none")
        .collect();

    reasoning.metrics.s1_moves = s1_moves;
    reasoning.metrics.s2_moves = s2_moves;

    // Analyze matchup with detailed reasoning
    let classification = analyze_matchup_with_reasoning(&mut sim_state, &mut reasoning);

    // Update the reasoning with the classification
    reasoning.set_result(classification);

    (classification, reasoning)
}

// Enhanced version of matchup analysis with improved setup analysis
pub fn analyze_matchup_with_reasoning(
    sim_state: &mut State,
    reasoning: &mut MatchupReasoning,
) -> i8 {
    // Returns classification (i8) directly
    // Find the best move choices for each Pokémon
    let (s1_best_choice, s1_best_move) = find_best_move(sim_state, &SideReference::SideOne);
    let (s2_best_choice, s2_best_move) = find_best_move(sim_state, &SideReference::SideTwo);

    // Start populating reasoning metrics
    reasoning.metrics.s1_best_move = choice_to_string(&s1_best_choice);
    reasoning.metrics.s2_best_move = choice_to_string(&s2_best_choice);

    // Add detection for stat-lowering moves
    let s1_has_stat_lowering = is_stat_lowering_move(&s1_best_move);
    let s2_has_stat_lowering = is_stat_lowering_move(&s2_best_move);

    reasoning.metrics.s1_has_stat_lowering_move = s1_has_stat_lowering;
    reasoning.metrics.s2_has_stat_lowering_move = s2_has_stat_lowering;

    // Analyze speed to determine who goes first
    let mut s1_move_choice = s1_best_choice.clone();
    let mut s2_move_choice = s2_best_choice.clone();
    s1_move_choice.first_move = true;
    s2_move_choice.first_move = true;

    let mut instructions = StateInstructions::default();
    let first_mover = moves_first(
        sim_state,
        &s1_move_choice,
        &s2_move_choice,
        &mut instructions,
    );

    // Determine who moves first (accounting for speed ties)
    let moves_first_side_one = match first_mover {
        SideMovesFirst::SideOne => {
            reasoning.add_step("Side One moves first due to higher speed".to_string());
            reasoning.metrics.s1_moves_first = true;
            reasoning.metrics.speed_tie = false;
            true
        }
        SideMovesFirst::SideTwo => {
            reasoning.add_step("Side Two moves first due to higher speed".to_string());
            reasoning.metrics.s1_moves_first = false;
            reasoning.metrics.speed_tie = false;
            false
        }
        SideMovesFirst::SpeedTie => {
            reasoning.add_step("Speed tie detected - favoring Side One for analysis".to_string());
            reasoning.metrics.s1_moves_first = true;
            reasoning.metrics.speed_tie = true;
            true
        }
    };

    // Get Pokémon stats
    let s1_pokemon = sim_state.side_one.get_active_immutable();
    let s2_pokemon = sim_state.side_two.get_active_immutable();

    reasoning.metrics.s1_hp = s1_pokemon.hp;
    reasoning.metrics.s1_max_hp = s1_pokemon.maxhp;
    reasoning.metrics.s2_hp = s2_pokemon.hp;
    reasoning.metrics.s2_max_hp = s2_pokemon.maxhp;

    // Calculate damage ranges
    let s1_damage_range = get_damage_range(sim_state, &s1_best_choice, &SideReference::SideOne);
    let s2_damage_range = get_damage_range(sim_state, &s2_best_choice, &SideReference::SideTwo);

    // Analyze passive effects for both Pokémon
    let passive_effects_s1 =
        analyze_passive_effects(sim_state, &SideReference::SideOne, &SideReference::SideTwo);

    let passive_effects_s2 =
        analyze_passive_effects(sim_state, &SideReference::SideTwo, &SideReference::SideOne);

    // Update reasoning metrics with passive effects info
    reasoning.metrics.s1_recovery_per_turn = passive_effects_s1.active_recovery_amount;
    reasoning.metrics.s2_recovery_per_turn = passive_effects_s2.active_recovery_amount;

    // Store passive effects in metrics
    reasoning.metrics.s1_has_active_recovery = passive_effects_s1.has_active_recovery;
    reasoning.metrics.s2_has_active_recovery = passive_effects_s2.has_active_recovery;
    reasoning.metrics.s1_passive_recovery_amount = passive_effects_s1.passive_recovery_amount;
    reasoning.metrics.s2_passive_recovery_amount = passive_effects_s2.passive_recovery_amount;
    reasoning.metrics.s1_passive_damage_outgoing = passive_effects_s1.passive_damage_outgoing;
    reasoning.metrics.s2_passive_damage_outgoing = passive_effects_s2.passive_damage_outgoing;
    reasoning.metrics.s1_passive_damage_incoming = passive_effects_s1.passive_damage_incoming;
    reasoning.metrics.s2_passive_damage_incoming = passive_effects_s2.passive_damage_incoming;

    // Get average and max damage
    let (s1_avg_damage, s1_max_damage) = if s1_damage_range.is_empty() {
        reasoning.add_step("Side One has no effective direct damage output".to_string());
        (0, 0)
    } else {
        let avg =
            (s1_damage_range.iter().sum::<i16>() as f32 / s1_damage_range.len() as f32) as i16;
        let max = *s1_damage_range.iter().max().unwrap_or(&0);
        reasoning.add_step(format!(
            "Side One damage output: Avg={}, Max={} ({:.1}% of opponent's HP)",
            avg,
            max,
            (avg as f32 / s2_pokemon.hp as f32) * 100.0
        ));
        (avg, max)
    };

    let (s2_avg_damage, s2_max_damage) = if s2_damage_range.is_empty() {
        reasoning.add_step("Side Two has no effective direct damage output".to_string());
        (0, 0)
    } else {
        let avg =
            (s2_damage_range.iter().sum::<i16>() as f32 / s2_damage_range.len() as f32) as i16;
        let max = *s2_damage_range.iter().max().unwrap_or(&0);
        reasoning.add_step(format!(
            "Side Two damage output: Avg={}, Max={} ({:.1}% of opponent's HP)",
            avg,
            max,
            (avg as f32 / s1_pokemon.hp as f32) * 100.0
        ));
        (avg, max)
    };

    reasoning.metrics.s1_avg_damage = s1_avg_damage;
    reasoning.metrics.s1_max_damage = s1_max_damage;
    reasoning.metrics.s2_avg_damage = s2_avg_damage;
    reasoning.metrics.s2_max_damage = s2_max_damage;

    // Add reasoning steps about passive effects
    if passive_effects_s1.passive_recovery_amount > 0 {
        reasoning.add_step(format!(
            "Side One has passive recovery: {} HP per turn",
            passive_effects_s1.passive_recovery_amount
        ));
    }

    if passive_effects_s2.passive_recovery_amount > 0 {
        reasoning.add_step(format!(
            "Side Two has passive recovery: {} HP per turn",
            passive_effects_s2.passive_recovery_amount
        ));
    }

    if passive_effects_s1.passive_damage_outgoing > 0 {
        reasoning.add_step(format!(
            "Side One inflicts {} passive damage per turn",
            passive_effects_s1.passive_damage_outgoing
        ));
    }

    if passive_effects_s2.passive_damage_outgoing > 0 {
        reasoning.add_step(format!(
            "Side Two inflicts {} passive damage per turn",
            passive_effects_s2.passive_damage_outgoing
        ));
    }

    if passive_effects_s1.passive_damage_incoming > 0 {
        reasoning.add_step(format!(
            "Side One takes {} passive damage per turn",
            passive_effects_s1.passive_damage_incoming
        ));
    }

    if passive_effects_s2.passive_damage_incoming > 0 {
        reasoning.add_step(format!(
            "Side Two takes {} passive damage per turn",
            passive_effects_s2.passive_damage_incoming
        ));
    }

    // Calculate priority move damage
    let s1_priority_damage = get_best_priority_move_damage(sim_state, &SideReference::SideOne);
    let s2_priority_damage = get_best_priority_move_damage(sim_state, &SideReference::SideTwo);

    reasoning.metrics.s1_priority_damage = s1_priority_damage;
    reasoning.metrics.s2_priority_damage = s2_priority_damage;

    // If moves lower stats, get second best moves and calculate percentages
    if s1_has_stat_lowering {
        reasoning.metrics.s1_stat_drop_percentage = get_stat_drop_percentage(&s1_best_move);
        reasoning.metrics.s1_second_best_damage =
            get_second_best_move_damage(sim_state, &SideReference::SideOne);

        // Add reasoning step about stat-lowering move
        reasoning.add_step(format!(
            "Side One's best move ({}) lowers stats. After use, damage reduced to {:.1}%",
            reasoning.metrics.s1_best_move,
            reasoning.metrics.s1_stat_drop_percentage * 100.0
        ));
    }

    if s2_has_stat_lowering {
        reasoning.metrics.s2_stat_drop_percentage = get_stat_drop_percentage(&s2_best_move);
        reasoning.metrics.s2_second_best_damage =
            get_second_best_move_damage(sim_state, &SideReference::SideTwo);

        // Add reasoning step about stat-lowering move
        reasoning.add_step(format!(
            "Side Two's best move ({}) lowers stats. After use, damage reduced to {:.1}%",
            reasoning.metrics.s2_best_move,
            reasoning.metrics.s2_stat_drop_percentage * 100.0
        ));
    }

    // Calculate damage after stat drops
    if s1_has_stat_lowering {
        reasoning.metrics.s1_post_drop_damage = (reasoning.metrics.s1_avg_damage as f32
            * reasoning.metrics.s1_stat_drop_percentage)
            as i16;

        // Add reasoning about optimal move sequence
        if reasoning.metrics.s1_second_best_damage + reasoning.metrics.s1_avg_damage
            >= s2_pokemon.hp
        {
            reasoning.add_step(format!(
                    "Side One optimal sequence: second best move ({} damage) then stat-lowering move for 2HKO",
                    reasoning.metrics.s1_second_best_damage
                ));
        } else {
            reasoning.add_step(format!(
                "Side One using stat-lowering move first, then reduced damage ({} → {})",
                reasoning.metrics.s1_avg_damage, reasoning.metrics.s1_post_drop_damage
            ));
        }
    }

    if s2_has_stat_lowering {
        reasoning.metrics.s2_post_drop_damage = (reasoning.metrics.s2_avg_damage as f32
            * reasoning.metrics.s2_stat_drop_percentage)
            as i16;

        // Add reasoning about optimal move sequence
        if reasoning.metrics.s2_second_best_damage + reasoning.metrics.s2_avg_damage
            >= s1_pokemon.hp
        {
            reasoning.add_step(format!(
                    "Side Two optimal sequence: second best move ({} damage) then stat-lowering move for 2HKO",
                    reasoning.metrics.s2_second_best_damage
                ));
        } else {
            reasoning.add_step(format!(
                "Side Two using stat-lowering move first, then reduced damage ({} → {})",
                reasoning.metrics.s2_avg_damage, reasoning.metrics.s2_post_drop_damage
            ));
        }
    }

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

    reasoning.add_step(format!(
        "Side One effective damage: {} (direct: {}, passive: {}, opponent recovery: {})",
        effective_damage_s1_to_s2,
        s1_avg_damage,
        passive_effects_s1.passive_damage_outgoing,
        s2_net_passive
    ));

    reasoning.add_step(format!(
        "Side Two effective damage: {} (direct: {}, passive: {}, opponent recovery: {})",
        effective_damage_s2_to_s1,
        s2_avg_damage,
        passive_effects_s2.passive_damage_outgoing,
        s1_net_passive
    ));

    // Calculate turns to KO (THIS IS MISSING)
    // Add these lines to calculate and set the turns to KO metrics
    let s1_turns_to_ko = if effective_damage_s1_to_s2 > 0 {
        ((s2_pokemon.hp as f32 / effective_damage_s1_to_s2 as f32).ceil() as i32).max(1)
    } else {
        99 // Can't KO
    };

    let s2_turns_to_ko = if effective_damage_s2_to_s1 > 0 {
        ((s1_pokemon.hp as f32 / effective_damage_s2_to_s1 as f32).ceil() as i32).max(1)
    } else {
        99 // Can't KO
    };

    reasoning.metrics.s1_turns_to_ko = s1_turns_to_ko;
    reasoning.metrics.s2_turns_to_ko = s2_turns_to_ko;

    // Add reasoning steps about turns to KO
    reasoning.add_step(format!(
        "Side One needs {} turns to KO (dealing {} damage per turn)",
        s1_turns_to_ko, effective_damage_s1_to_s2
    ));

    reasoning.add_step(format!(
        "Side Two needs {} turns to KO (dealing {} damage per turn)",
        s2_turns_to_ko, effective_damage_s2_to_s1
    ));

    // Calculate OHKO chances (THIS IS MISSING)
    reasoning.metrics.s1_ohko_chance = if s1_max_damage >= s2_pokemon.hp {
        // Simplified OHKO chance estimate
        let ohko_rolls = s1_damage_range
            .iter()
            .filter(|&&d| d >= s2_pokemon.hp)
            .count();
        if !s1_damage_range.is_empty() {
            ohko_rolls as f32 / s1_damage_range.len() as f32
        } else {
            0.0
        }
    } else {
        0.0
    };

    reasoning.metrics.s2_ohko_chance = if s2_max_damage >= s1_pokemon.hp {
        // Simplified OHKO chance estimate
        let ohko_rolls = s2_damage_range
            .iter()
            .filter(|&&d| d >= s1_pokemon.hp)
            .count();
        if !s2_damage_range.is_empty() {
            ohko_rolls as f32 / s2_damage_range.len() as f32
        } else {
            0.0
        }
    } else {
        0.0
    };

    // Use improved setup analysis that calculates optimal setup stages
    let setup_info_s1 = analyze_setup_capabilities(
        sim_state,
        &SideReference::SideOne,
        &SideReference::SideTwo,
        effective_damage_s2_to_s1,
        effective_damage_s1_to_s2,
        moves_first_side_one,
        &passive_effects_s1,
        &passive_effects_s2,
    );

    let setup_info_s2 = analyze_setup_capabilities(
        sim_state,
        &SideReference::SideTwo,
        &SideReference::SideOne,
        effective_damage_s1_to_s2,
        effective_damage_s2_to_s1,
        !moves_first_side_one,
        &passive_effects_s2,
        &passive_effects_s1,
    );

    // Update reasoning with setup information
    reasoning.metrics.s1_has_setup = setup_info_s1.has_setup;
    reasoning.metrics.s2_has_setup = setup_info_s2.has_setup;
    reasoning.metrics.s1_boosts_speed = setup_info_s1.boosts_speed;
    reasoning.metrics.s2_boosts_speed = setup_info_s2.boosts_speed;
    reasoning.metrics.s1_setup_turns = setup_info_s1.optimal_setup_stages;
    reasoning.metrics.s2_setup_turns = setup_info_s2.optimal_setup_stages;

    // Add reasoning steps about optimal setup
    if setup_info_s1.has_setup {
        if setup_info_s1.optimal_setup_stages > 0 {
            reasoning.add_step(format!(
                "Side One can safely set up for {} turns (optimal strategy)",
                setup_info_s1.optimal_setup_stages
            ));
            reasoning.metrics.s1_setup_viable = true;
        } else {
            reasoning.add_step(
                "Side One has setup moves but setup is not optimal in this matchup".to_string(),
            );
            reasoning.metrics.s1_setup_viable = false;
        }
    }

    if setup_info_s2.has_setup {
        if setup_info_s2.optimal_setup_stages > 0 {
            reasoning.add_step(format!(
                "Side Two can safely set up for {} turns (optimal strategy)",
                setup_info_s2.optimal_setup_stages
            ));
            reasoning.metrics.s2_setup_viable = true;
        } else {
            reasoning.add_step(
                "Side Two has setup moves but setup is not optimal in this matchup".to_string(),
            );
            reasoning.metrics.s2_setup_viable = false;
        }
    }

    // Calculate boosted damage after optimal setup
    let s1_boosted_damage = calculate_boosted_damage_optimal(
        s1_avg_damage,
        setup_info_s1.optimal_setup_stages * setup_info_s1.attack_per_stage,
        setup_info_s1.optimal_setup_stages * setup_info_s1.special_attack_per_stage,
    );

    let s2_boosted_damage = calculate_boosted_damage_optimal(
        s2_avg_damage,
        setup_info_s2.optimal_setup_stages * setup_info_s2.attack_per_stage,
        setup_info_s2.optimal_setup_stages * setup_info_s2.special_attack_per_stage,
    );

    reasoning.metrics.s1_boosted_damage = s1_boosted_damage;
    reasoning.metrics.s2_boosted_damage = s2_boosted_damage;

    reasoning.add_step(format!(
        "After optimal setup: Side One damage = {} ({:.1}% boost), Side Two damage = {} ({:.1}% boost)",
        s1_boosted_damage,
        if reasoning.metrics.s1_avg_damage > 0 {
            (s1_boosted_damage as f32 / reasoning.metrics.s1_avg_damage as f32 - 1.0) * 100.0
        } else {
            0.0
        },
        s2_boosted_damage,
        if reasoning.metrics.s2_avg_damage > 0 {
            (s2_boosted_damage as f32 / reasoning.metrics.s2_avg_damage as f32 - 1.0) * 100.0
        } else {
            0.0
        }
    ));

    // Check for potential OHKO after setup
    if s1_boosted_damage >= s2_pokemon.hp {
        reasoning.add_step("Side One can achieve OHKO after setup".to_string());
        reasoning.metrics.s1_setup_ohko = true;
    }

    if s2_boosted_damage >= s1_pokemon.hp {
        reasoning.add_step("Side Two can achieve OHKO after setup".to_string());
        reasoning.metrics.s2_setup_ohko = true;
    }

    // Identify the matchup category for better reasoning
    // Now based on move capabilities, not viability
    let category = get_matchup_category(
        setup_info_s1.has_setup,
        passive_effects_s1.has_active_recovery,
        setup_info_s2.has_setup,
        passive_effects_s2.has_active_recovery,
    );

    reasoning.add_step(format!("Matchup category: {:?}", category));

    // Apply our new category-based analysis approach
    let classification = classify_matchup_by_category(
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
        reasoning.metrics.s1_has_stat_lowering_move,
        reasoning.metrics.s1_second_best_damage,
        reasoning.metrics.s1_stat_drop_percentage,
        reasoning.metrics.s2_has_stat_lowering_move,
        reasoning.metrics.s2_second_best_damage,
        reasoning.metrics.s2_stat_drop_percentage,
        sim_state,
    );

    reasoning.add_step(format!(
        "Final classification: {}",
        match classification {
            2 => "Strong Counter",
            1 => "Check",
            0 => "Neutral",
            -1 => "Checked",
            -2 => "Hard Countered",
            _ => "Unknown",
        }
    ));

    // Determine the primary reason
    determine_primary_reason(reasoning, classification);

    // Update the reasoning struct with the classification directly
    reasoning.set_result(classification);

    // Return the classification directly
    classification
}

// Function to determine the primary reason for the matchup outcome
// Function to determine the primary reason for the matchup outcome
fn determine_primary_reason(reasoning: &mut MatchupReasoning, classification: i8) {
    // Extract relevant metrics
    let metrics = &reasoning.metrics;
    let s1_moves_first = metrics.s1_moves_first;
    let s1_ohko_chance = metrics.s1_ohko_chance;
    let s2_ohko_chance = metrics.s2_ohko_chance;
    let s1_priority_ko = metrics.s1_priority_ko;
    let s2_priority_ko = metrics.s2_priority_ko;
    let s1_recovery_dominates = metrics.s1_recovery_dominates;
    let s2_recovery_dominates = metrics.s2_recovery_dominates;
    let s1_setup_viable = metrics.s1_setup_viable;
    let s2_setup_viable = metrics.s2_setup_viable;
    let s1_setup_ohko = metrics.s1_setup_ohko;
    let s2_setup_ohko = metrics.s2_setup_ohko;
    let s1_best_ko_turns = metrics.s1_best_ko_turns;
    let s2_best_ko_turns = metrics.s2_best_ko_turns;
    let s1_recovery_sufficient = metrics.s1_recovery_sufficient;
    let s2_recovery_sufficient = metrics.s2_recovery_sufficient;
    let s1_has_stat_lowering = metrics.s1_has_stat_lowering_move;
    let s2_has_stat_lowering = metrics.s2_has_stat_lowering_move;

    // COUNTER CONDITIONS (2)
    if classification == 2 {
        if s1_priority_ko && !s2_priority_ko {
            reasoning.set_primary_reason("Can KO with priority before opponent moves".to_string());
            return;
        }

        if s1_recovery_dominates && !s2_recovery_dominates {
            reasoning.set_primary_reason(
                "Recovery exceeds all possible damage from opponent".to_string(),
            );
            return;
        }

        if s1_moves_first && s1_ohko_chance > 0.9 {
            reasoning.set_primary_reason("Has nearly guaranteed OHKO and moves first".to_string());
            return;
        }

        if s1_moves_first && s1_best_ko_turns <= 2 && s2_best_ko_turns >= 3 {
            reasoning.set_primary_reason("Much faster KO and moves first".to_string());
            return;
        }

        if s1_setup_viable && (metrics.s1_boosts_speed || s1_moves_first) && s1_setup_ohko {
            reasoning.set_primary_reason("Can safely set up and then OHKO".to_string());
            return;
        }

        if s1_has_stat_lowering
            && metrics.s1_second_best_damage > 0
            && s1_best_ko_turns < s2_best_ko_turns
        {
            reasoning.set_primary_reason(
                "Effectively manages stat-lowering moves with good secondary options".to_string(),
            );
            return;
        }

        reasoning.set_primary_reason("Strong advantage in battle dynamics".to_string());
        return;
    }

    // CHECK CONDITIONS (1)
    if classification == 1 {
        if s1_recovery_sufficient && !s2_recovery_sufficient {
            reasoning.set_primary_reason("Recovery offsets most but not all damage".to_string());
            return;
        }

        if s1_moves_first && s1_best_ko_turns <= s2_best_ko_turns {
            reasoning.set_primary_reason("Faster and equal/better KO speed".to_string());
            return;
        }

        if s1_setup_viable && s1_setup_ohko && !s1_moves_first {
            reasoning
                .set_primary_reason("Can set up and OHKO but doesn't control speed".to_string());
            return;
        }

        if s1_has_stat_lowering && s1_best_ko_turns <= s2_best_ko_turns {
            reasoning.set_primary_reason(
                "Maintains KO advantage despite stat-lowering moves".to_string(),
            );
            return;
        }

        reasoning.set_primary_reason("Favorable matchup with multiple advantages".to_string());
        return;
    }

    // NEUTRAL CONDITIONS (0)
    if classification == 0 {
        if (s1_moves_first && s1_best_ko_turns > s2_best_ko_turns)
            || (!s1_moves_first && s1_best_ko_turns < s2_best_ko_turns)
        {
            reasoning.set_primary_reason("Offsetting advantages (speed vs. damage)".to_string());
            return;
        }

        if s1_recovery_sufficient && s2_recovery_sufficient {
            reasoning.set_primary_reason("Both sides have sufficient recovery".to_string());
            return;
        }

        if s1_setup_viable && s2_setup_viable {
            reasoning.set_primary_reason("Both sides can set up effectively".to_string());
            return;
        }

        if s1_has_stat_lowering && s2_has_stat_lowering {
            reasoning.set_primary_reason(
                "Both sides manage stat-lowering moves effectively".to_string(),
            );
            return;
        }

        reasoning.set_primary_reason("Balanced matchup with no clear advantage".to_string());
        return;
    }

    // CHECKED CONDITIONS (-1)
    if classification == -1 {
        if !s1_moves_first && s1_best_ko_turns == s2_best_ko_turns {
            reasoning.set_primary_reason("Equal KO timing but moves second".to_string());
            return;
        }

        if s1_best_ko_turns > s2_best_ko_turns && !s1_recovery_sufficient {
            reasoning
                .set_primary_reason("Takes longer to KO and recovery isn't sufficient".to_string());
            return;
        }

        if s2_setup_viable && !s1_setup_viable {
            reasoning
                .set_primary_reason("Opponent can set up safely but Side One cannot".to_string());
            return;
        }

        if !s1_has_stat_lowering && s2_has_stat_lowering && metrics.s2_second_best_damage > 0 {
            reasoning.set_primary_reason("Opponent has more diverse offensive options".to_string());
            return;
        }

        reasoning
            .set_primary_reason("Disadvantageous matchup with multiple weaknesses".to_string());
        return;
    }

    // COUNTERED CONDITIONS (-2)
    if classification == -2 {
        if s2_recovery_dominates && !s1_recovery_dominates {
            reasoning
                .set_primary_reason("Opponent can recover more than max damage output".to_string());
            return;
        }

        if s2_priority_ko && !s1_priority_ko {
            reasoning.set_primary_reason(
                "Opponent KOs with priority before Side One can move".to_string(),
            );
            return;
        }

        if !s1_moves_first && s2_ohko_chance > 0.9 {
            reasoning.set_primary_reason(
                "Opponent almost guaranteed to OHKO and moves first".to_string(),
            );
            return;
        }

        if !s1_moves_first && s2_best_ko_turns <= 2 && s1_best_ko_turns >= 3 {
            reasoning.set_primary_reason("Opponent has much faster KO and moves first".to_string());
            return;
        }

        if s2_setup_viable && (metrics.s2_boosts_speed || !s1_moves_first) && s2_setup_ohko {
            reasoning.set_primary_reason("Opponent can safely set up and OHKO".to_string());
            return;
        }

        if s2_has_stat_lowering && s2_best_ko_turns + 1 < s1_best_ko_turns {
            reasoning.set_primary_reason(
                "Opponent efficiently uses stat-lowering moves to KO much faster".to_string(),
            );
            return;
        }

        reasoning.set_primary_reason("Severely disadvantaged in multiple aspects".to_string());
        return;
    }

    // Default case (should never reach here)
    reasoning.set_primary_reason("Undetermined matchup dynamics".to_string());
}
