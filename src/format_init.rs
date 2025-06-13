use crate::battle_format::{BattleFormat, BattlePosition};
use crate::format_config::{BattleFormatFactory, ConfigBasedValidator};
use crate::format_enforcement::{FormatEnforcer, FormatStateTracker};
use crate::state::{State, SideReference};

/// Battle initialization with format support
pub struct BattleInitializer;

impl BattleInitializer {
    /// Initialize a battle with a specific format
    pub fn initialize_battle(
        state: &mut State,
        format: BattleFormat,
        config_name: Option<&str>,
    ) -> Result<BattleContext, String> {
        // Validate teams against format rules
        crate::battle_format::FormatValidator::validate_battle_state(state, &format)?;
        
        // Additional validation with configuration if provided
        if let Some(config) = config_name {
            ConfigBasedValidator::validate_with_config(&state.side_one, &format, config)?;
            ConfigBasedValidator::validate_with_config(&state.side_two, &format, config)?;
        }
        
        // Initialize battle context
        let enforcer = FormatEnforcer::new(format.clone());
        let mut tracker = FormatStateTracker::new(format.clone());
        tracker.update_from_state(state);
        
        Ok(BattleContext {
            format,
            config_name: config_name.map(|s| s.to_string()),
            enforcer,
            tracker,
        })
    }
    
    /// Detect format from state (heuristic-based)
    pub fn detect_format(state: &State) -> BattleFormat {
        // Count active Pokemon slots that could be used
        let side_one_active_count = if state.side_one.pokemon.p1.id != crate::pokemon::PokemonName::NONE
            && state.side_one.pokemon.p1.hp > 0 { 2 } else { 1 };
        let side_two_active_count = if state.side_two.pokemon.p1.id != crate::pokemon::PokemonName::NONE 
            && state.side_two.pokemon.p1.hp > 0 { 2 } else { 1 };
        
        // If either side has 2 active Pokemon, assume doubles
        if side_one_active_count > 1 || side_two_active_count > 1 {
            BattleFormat::Doubles
        } else {
            BattleFormat::Singles
        }
    }
    
    /// Create format from a string identifier
    pub fn format_from_string(format_str: &str) -> Result<BattleFormat, String> {
        match format_str.to_lowercase().as_str() {
            "singles" => Ok(BattleFormat::Singles),
            "doubles" => Ok(BattleFormat::Doubles),
            "vgc" => Ok(BattleFormat::VGC),
            config_name => {
                // Try to create from config
                BattleFormatFactory::from_config_name(config_name)
                    .ok_or_else(|| format!("Unknown format: {}", format_str))
            }
        }
    }
}

/// Context for managing a battle with format rules
#[derive(Debug)]
pub struct BattleContext {
    pub format: BattleFormat,
    pub config_name: Option<String>,
    pub enforcer: FormatEnforcer,
    pub tracker: FormatStateTracker,
}

impl BattleContext {
    /// Get the user's position in battle
    pub fn get_user_position(&self, side: SideReference, slot: usize) -> Option<BattlePosition> {
        let rules = self.format.get_rules();
        if slot < rules.active_pokemon {
            Some(BattlePosition::new(side, slot))
        } else {
            None
        }
    }
    
    /// Update context after state changes
    pub fn update(&mut self, state: &State) {
        self.tracker.update_from_state(state);
    }
}

/// Extension trait for State to support formats
pub trait StateFormatExt {
    fn get_format(&self) -> Option<&BattleFormat>;
    fn set_format(&mut self, format: BattleFormat);
    fn get_active_positions(&self, side: SideReference) -> Vec<BattlePosition>;
}

// Note: In a real implementation, we would extend the State struct to include format info.
// For now, we'll use a separate BattleContext to manage format-related state.

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_format_detection() {
        let mut state = State::default();
        
        // Singles detection (default)
        let format = BattleInitializer::detect_format(&state);
        assert_eq!(format, BattleFormat::Singles);
        
        // Doubles detection (when P1 slot has a Pokemon)
        state.side_one.pokemon.p1.id = crate::pokemon::PokemonName::PIKACHU;
        state.side_one.pokemon.p1.hp = 100;
        let format = BattleInitializer::detect_format(&state);
        assert_eq!(format, BattleFormat::Doubles);
    }
    
    #[test]
    fn test_format_from_string() {
        assert_eq!(
            BattleInitializer::format_from_string("singles").unwrap(),
            BattleFormat::Singles
        );
        assert_eq!(
            BattleInitializer::format_from_string("DOUBLES").unwrap(),
            BattleFormat::Doubles
        );
        assert_eq!(
            BattleInitializer::format_from_string("vgc").unwrap(),
            BattleFormat::VGC
        );
        assert!(BattleInitializer::format_from_string("invalid").is_err());
    }
}