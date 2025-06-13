use crate::state::{SideReference, State, Side};
use crate::choices::Choices;
use crate::data::types::MoveTarget;
use crate::pokemon::PokemonName;
use crate::engine::items::Items;

#[derive(Debug, Clone, PartialEq)]
pub enum BattleFormat {
    /// Standard singles format (uses singles_ou from registry)
    Singles,
    /// Standard doubles format (uses doubles_ou from registry)
    Doubles,
    /// VGC format (uses vgc_2024_reg_g from registry)
    VGC,
    /// Named format from the format registry
    Named(String),
    /// Custom format with user-defined rules
    Custom(BattleRules),
}

impl BattleFormat {
    pub fn get_rules(&self) -> BattleRules {
        match self {
            BattleFormat::Singles => {
                crate::format_registry::FormatRegistry::get_battle_rules("singles_ou")
                    .unwrap_or_else(|| BattleRules {
                        party_size: 6,
                        team_size: 6,
                        active_pokemon: 1,
                        format_clauses: vec![FormatClause::SpeciesClause, FormatClause::SleepClause],
                    })
            },
            BattleFormat::Doubles => {
                crate::format_registry::FormatRegistry::get_battle_rules("doubles_ou")
                    .unwrap_or_else(|| BattleRules {
                        party_size: 6,
                        team_size: 6,
                        active_pokemon: 2,
                        format_clauses: vec![FormatClause::SpeciesClause, FormatClause::SleepClause],
                    })
            },
            BattleFormat::VGC => {
                crate::format_registry::FormatRegistry::get_battle_rules("vgc_2024_reg_g")
                    .unwrap_or_else(|| BattleRules {
                        party_size: 6,
                        team_size: 4,
                        active_pokemon: 2,
                        format_clauses: vec![FormatClause::SpeciesClause, FormatClause::ItemClause],
                    })
            },
            BattleFormat::Named(name) => {
                crate::format_registry::FormatRegistry::get_battle_rules(name)
                    .unwrap_or_else(|| panic!("Unknown format: {}", name))
            },
            BattleFormat::Custom(rules) => rules.clone(),
        }
    }
    
    /// Get the registry name for this format (if applicable)
    pub fn get_registry_name(&self) -> Option<String> {
        match self {
            BattleFormat::Singles => Some("singles_ou".to_string()),
            BattleFormat::Doubles => Some("doubles_ou".to_string()),
            BattleFormat::VGC => Some("vgc_2024_reg_g".to_string()),
            BattleFormat::Named(name) => Some(name.clone()),
            BattleFormat::Custom(_) => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BattleRules {
    pub party_size: usize,      // Total Pokemon in party (6 for most formats)
    pub team_size: usize,       // Pokemon brought to battle (6 for singles/doubles, 4 for VGC)
    pub active_pokemon: usize,  // Active Pokemon per side (1 for singles, 2 for doubles)
    pub format_clauses: Vec<FormatClause>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum FormatClause {
    SpeciesClause,    // No duplicate species
    SleepClause,      // Limit sleeping opponents
    FreezeClause,     // Limit freezing opponents
    OHKOClause,       // Ban OHKO moves
    EvasionClause,    // Ban evasion moves
    ItemClause,       // No duplicate items
    Custom(String),   // Custom format-specific rules
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BattlePosition {
    pub side: SideReference,
    pub slot: usize,  // 0 for singles, 0-1 for doubles
}

impl BattlePosition {
    pub fn new(side: SideReference, slot: usize) -> Self {
        BattlePosition { side, slot }
    }
}

pub trait TargetResolver {
    fn resolve_targets(
        &self,
        user_position: BattlePosition,
        move_target: MoveTarget,
        battle_state: &State,
    ) -> Vec<BattlePosition>;
    
    fn is_valid_target(
        &self,
        target: BattlePosition,
        battle_state: &State,
    ) -> bool;
}

pub struct FormatTargetResolver {
    pub format: BattleFormat,
}

impl FormatTargetResolver {
    pub fn new(format: BattleFormat) -> Self {
        FormatTargetResolver { format }
    }
}

impl TargetResolver for FormatTargetResolver {
    fn resolve_targets(
        &self,
        user_position: BattlePosition,
        move_target: MoveTarget,
        battle_state: &State,
    ) -> Vec<BattlePosition> {
        let rules = self.format.get_rules();
        let mut targets = Vec::new();
        
        match move_target {
            MoveTarget::User => {
                targets.push(user_position);
            }
            MoveTarget::SelectedPokemon => {
                // In singles, target the opponent
                // In doubles, this would need user input to select target
                if rules.active_pokemon == 1 {
                    let opponent_side = user_position.side.get_other_side();
                    targets.push(BattlePosition::new(opponent_side, 0));
                } else {
                    // For doubles, would need to handle target selection
                    // For now, default to first available opponent
                    let opponent_side = user_position.side.get_other_side();
                    targets.push(BattlePosition::new(opponent_side, 0));
                }
            }
            MoveTarget::AllOpponents => {
                let opponent_side = user_position.side.get_other_side();
                for slot in 0..rules.active_pokemon {
                    targets.push(BattlePosition::new(opponent_side, slot));
                }
            }
            MoveTarget::AllOtherPokemon => {
                // All Pokemon except the user
                for slot in 0..rules.active_pokemon {
                    if !(user_position.side == SideReference::SideOne && slot == user_position.slot) {
                        targets.push(BattlePosition::new(SideReference::SideOne, slot));
                    }
                    if !(user_position.side == SideReference::SideTwo && slot == user_position.slot) {
                        targets.push(BattlePosition::new(SideReference::SideTwo, slot));
                    }
                }
            }
            MoveTarget::AllPokemon => {
                // All Pokemon including the user
                for slot in 0..rules.active_pokemon {
                    targets.push(BattlePosition::new(SideReference::SideOne, slot));
                    targets.push(BattlePosition::new(SideReference::SideTwo, slot));
                }
            }
            MoveTarget::UsersField => {
                // Field effects target the side, not specific Pokemon
                targets.push(user_position);
            }
            MoveTarget::OpponentsField => {
                let opponent_side = user_position.side.get_other_side();
                targets.push(BattlePosition::new(opponent_side, 0));
            }
            MoveTarget::EntireField => {
                // Weather/terrain affects the entire field
                targets.push(BattlePosition::new(SideReference::SideOne, 0));
            }
            MoveTarget::RandomOpponent => {
                let opponent_side = user_position.side.get_other_side();
                // In doubles, would randomly select one opponent
                targets.push(BattlePosition::new(opponent_side, 0));
            }
            MoveTarget::Ally => {
                // In doubles, target the ally
                if rules.active_pokemon > 1 && user_position.slot == 0 {
                    targets.push(BattlePosition::new(user_position.side, 1));
                } else if rules.active_pokemon > 1 && user_position.slot == 1 {
                    targets.push(BattlePosition::new(user_position.side, 0));
                }
            }
            MoveTarget::UserOrAlly => {
                targets.push(user_position);
                if rules.active_pokemon > 1 {
                    let ally_slot = if user_position.slot == 0 { 1 } else { 0 };
                    targets.push(BattlePosition::new(user_position.side, ally_slot));
                }
            }
            MoveTarget::UserAndAllies => {
                for slot in 0..rules.active_pokemon {
                    targets.push(BattlePosition::new(user_position.side, slot));
                }
            }
            MoveTarget::AllAllies => {
                for slot in 0..rules.active_pokemon {
                    if slot != user_position.slot {
                        targets.push(BattlePosition::new(user_position.side, slot));
                    }
                }
            }
            MoveTarget::SpecificMove => {
                // Moves like Mirror Move/Copycat - handled separately
                targets.push(user_position);
            }
            MoveTarget::SelectedPokemonMeFirst => {
                // Me First targets - similar to SelectedPokemon
                if rules.active_pokemon == 1 {
                    let opponent_side = user_position.side.get_other_side();
                    targets.push(BattlePosition::new(opponent_side, 0));
                } else {
                    let opponent_side = user_position.side.get_other_side();
                    targets.push(BattlePosition::new(opponent_side, 0));
                }
            }
            MoveTarget::FaintingPokemon => {
                // For moves like Healing Wish - targets user
                targets.push(user_position);
            }
        }
        
        // Filter out invalid targets (fainted Pokemon, etc.)
        targets.retain(|&pos| self.is_valid_target(pos, battle_state));
        
        targets
    }
    
    fn is_valid_target(
        &self,
        target: BattlePosition,
        _battle_state: &State,
    ) -> bool {
        let rules = self.format.get_rules();
        
        // Check if slot is valid for format
        if target.slot >= rules.active_pokemon {
            return false;
        }
        
        // For now, assume all positions are valid if within active range
        // In the future, would check if Pokemon at position is alive, not protected, etc.
        true
    }
}

pub struct FormatValidator;

impl FormatValidator {
    pub fn validate_team(side: &Side, format: &BattleFormat) -> Result<(), String> {
        let rules = format.get_rules();
        
        // Count non-empty Pokemon
        let pokemon_count = side.pokemon.into_iter()
            .filter(|p| p.id != PokemonName::NONE)
            .count();
        
        // Check party size
        if pokemon_count > rules.party_size {
            return Err(format!("Team has {} Pokemon, but format allows maximum {}", 
                pokemon_count, rules.party_size));
        }
        
        // Check format clauses
        for clause in &rules.format_clauses {
            match clause {
                FormatClause::SpeciesClause => {
                    let mut species_vec: Vec<PokemonName> = Vec::new();
                    for pokemon in side.pokemon.into_iter() {
                        if pokemon.id != PokemonName::NONE {
                            if species_vec.contains(&pokemon.id) {
                                return Err(format!("Duplicate species found: {:?}", pokemon.id));
                            }
                            species_vec.push(pokemon.id);
                        }
                    }
                }
                FormatClause::ItemClause => {
                    let mut item_vec: Vec<Items> = Vec::new();
                    for pokemon in side.pokemon.into_iter() {
                        if pokemon.item != Items::NONE {
                            if item_vec.contains(&pokemon.item) {
                                return Err(format!("Duplicate item found: {:?}", pokemon.item));
                            }
                            item_vec.push(pokemon.item);
                        }
                    }
                }
                FormatClause::OHKOClause => {
                    for pokemon in side.pokemon.into_iter() {
                        for mv in pokemon.moves.into_iter() {
                            if matches!(mv.id, 
                                Choices::FISSURE | 
                                Choices::GUILLOTINE | 
                                Choices::HORNDRILL | 
                                Choices::SHEERCOLD
                            ) {
                                return Err(format!("OHKO move {:?} is banned", mv.id));
                            }
                        }
                    }
                }
                FormatClause::EvasionClause => {
                    for pokemon in side.pokemon.into_iter() {
                        for mv in pokemon.moves.into_iter() {
                            if matches!(mv.id, 
                                Choices::DOUBLETEAM | 
                                Choices::MINIMIZE
                            ) {
                                return Err(format!("Evasion move {:?} is banned", mv.id));
                            }
                        }
                    }
                }
                _ => {
                    // Sleep/Freeze clauses are enforced during battle
                }
            }
        }
        
        Ok(())
    }
    
    pub fn validate_battle_state(state: &State, format: &BattleFormat) -> Result<(), String> {
        // Validate both teams
        FormatValidator::validate_team(&state.side_one, format)?;
        FormatValidator::validate_team(&state.side_two, format)?;
        
        // Check format-specific battle state rules
        let rules = format.get_rules();
        
        // Ensure active indices are valid for format
        if state.side_one.active_index as usize >= rules.active_pokemon {
            return Err("Side one active index exceeds format's active Pokemon limit".to_string());
        }
        if state.side_two.active_index as usize >= rules.active_pokemon {
            return Err("Side two active index exceeds format's active Pokemon limit".to_string());
        }
        
        Ok(())
    }
}