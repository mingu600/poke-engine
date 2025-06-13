/// Format-aware move targeting system for Phase 4
/// Integrates rustemon MoveTarget enum with actual battle execution
use crate::battle_format::{BattleFormat, BattlePosition, FormatTargetResolver, TargetResolver};
use crate::choices::Choices;
use crate::choices::MOVES;
use crate::data::types::MoveTarget;
use super::state::{FormatAwareMoveChoice, MoveChoice};
use crate::state::{PokemonMoveIndex, SideReference, State};

/// Move target resolution service for format-aware battles
pub struct FormatMoveTargetResolver {
    target_resolver: FormatTargetResolver,
}

impl FormatMoveTargetResolver {
    pub fn new(format: BattleFormat) -> Self {
        Self {
            target_resolver: FormatTargetResolver::new(format),
        }
    }

    /// Get access to the underlying target resolver for advanced operations
    pub fn target_resolver(&self) -> &FormatTargetResolver {
        &self.target_resolver
    }

    /// Resolve targets for a move choice based on the current battle format
    pub fn resolve_move_targets(
        &self,
        user_side: SideReference,
        user_slot: usize,
        move_choice: &MoveChoice,
        state: &State,
    ) -> Result<Vec<BattlePosition>, String> {
        let move_index = match move_choice {
            MoveChoice::Move(index) | MoveChoice::MoveTera(index) => *index,
            MoveChoice::Switch(_) | MoveChoice::None => {
                return Ok(vec![]); // No targets for switches or no-moves
            }
        };

        let user_position = BattlePosition::new(user_side, user_slot);
        
        // Get the move ID from the Pokemon's moveset
        let side = state.get_side_immutable(&user_side);
        let active_pokemon = side.get_active_immutable();
        let move_id = active_pokemon.moves[&move_index].id;
        
        // Get the move target from rustemon data or engine-specific data
        let move_target = self.get_move_target(move_id)?;
        
        // Resolve targets using the format-specific resolver
        let targets = self.target_resolver.resolve_targets(
            user_position,
            move_target,
            state,
        );
        
        // Validate targets
        let valid_targets: Vec<BattlePosition> = targets
            .into_iter()
            .filter(|&target| self.target_resolver.is_valid_target(target, state))
            .collect();
        
        Ok(valid_targets)
    }

    /// Convert legacy MoveChoice to FormatAwareMoveChoice with resolved targets
    pub fn enhance_move_choice(
        &self,
        choice: MoveChoice,
        user_side: SideReference,
        user_slot: usize,
        state: &State,
    ) -> Result<FormatAwareMoveChoice, String> {
        let mut enhanced = FormatAwareMoveChoice::from_legacy(choice);
        
        // Resolve targets if this is a move choice
        if let Some(_move_index) = enhanced.get_move_index() {
            let targets = self.resolve_move_targets(user_side, user_slot, &choice, state)?;
            enhanced = enhanced.with_targets(targets);
        }
        
        Ok(enhanced)
    }

    /// Get the MoveTarget for a given move ID
    pub fn get_move_target(&self, move_id: Choices) -> Result<MoveTarget, String> {
        // For now, use default targeting until we fully integrate Phase 3 data
        // TODO: Integrate with rustemon move data and Phase 3 migration results
        self.get_default_move_target(move_id)
    }

    /// Get default move target for moves without specific rustemon data
    fn get_default_move_target(&self, move_id: Choices) -> Result<MoveTarget, String> {
        // Most moves target selected opponent by default
        match move_id {
            // Self-targeting moves
            Choices::SWORDSDANCE | Choices::AGILITY | Choices::RECOVER | Choices::REST => {
                Ok(MoveTarget::User)
            }
            // Field moves
            Choices::SUNNYDAY | Choices::SANDSTORM | Choices::HAIL | Choices::RAINDANCE => {
                Ok(MoveTarget::EntireField)
            }
            // Multi-target moves
            Choices::EARTHQUAKE | Choices::SURF | Choices::ROCKSLIDE => {
                Ok(MoveTarget::AllOtherPokemon)
            }
            // Status moves that affect all opponents
            Choices::SPORE | Choices::SLEEPPOWDER => {
                Ok(MoveTarget::AllOpponents)
            }
            // Default: target selected opponent
            _ => Ok(MoveTarget::SelectedPokemon),
        }
    }

    /// Check if a move requires manual target selection in the current format
    pub fn requires_target_selection(
        &self,
        move_id: Choices,
        format: &BattleFormat,
    ) -> Result<bool, String> {
        let move_target = self.get_move_target(move_id)?;
        let rules = format.get_rules();
        
        // In singles, most moves auto-target
        if rules.active_pokemon == 1 {
            return Ok(false);
        }
        
        // In doubles/multi, some moves require selection
        match move_target {
            MoveTarget::SelectedPokemon => Ok(true), // Player must choose target
            MoveTarget::Ally => Ok(rules.active_pokemon > 1), // Only in multi-format
            MoveTarget::User => Ok(false), // Always auto-target
            MoveTarget::AllOpponents => Ok(false), // Auto-target all
            MoveTarget::AllOtherPokemon => Ok(false), // Auto-target all except user
            MoveTarget::AllPokemon => Ok(false), // Auto-target everyone
            MoveTarget::RandomOpponent => Ok(false), // Engine picks random
            _ => Ok(false), // Most other targets are auto-resolved
        }
    }
}

/// Auto-targeting logic for moves that don't require manual selection
pub struct AutoTargetingEngine {
    resolver: FormatMoveTargetResolver,
}

impl AutoTargetingEngine {
    pub fn new(format: BattleFormat) -> Self {
        Self {
            resolver: FormatMoveTargetResolver::new(format),
        }
    }

    /// Automatically resolve targets for a move that doesn't require user selection
    pub fn auto_resolve_targets(
        &self,
        move_id: Choices,
        user_side: SideReference,
        user_slot: usize,
        state: &State,
    ) -> Result<Vec<BattlePosition>, String> {
        let move_target = self.resolver.get_move_target(move_id)?;
        let user_position = BattlePosition::new(user_side, user_slot);
        
        let targets = self.resolver.target_resolver.resolve_targets(
            user_position,
            move_target,
            state,
        );
        
        // For RandomOpponent, pick one random target
        if move_target == MoveTarget::RandomOpponent && targets.len() > 1 {
            // Use a deterministic "random" selection based on state
            // In a real implementation, this might use a seeded RNG
            // Use a simple deterministic selection based on available state
            let selected_index = 0; // For now, pick first target. TODO: Add proper randomization
            return Ok(vec![targets[selected_index]]);
        }
        
        Ok(targets)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::State;

    #[test]
    fn test_move_target_resolution_singles() {
        let format = BattleFormat::Singles;
        let resolver = FormatMoveTargetResolver::new(format);
        let state = State::default();
        
        // Test basic move targeting in singles
        let choice = MoveChoice::Move(crate::state::PokemonMoveIndex::M0);
        let result = resolver.resolve_move_targets(
            SideReference::SideOne,
            0,
            &choice,
            &state,
        );
        
        // Should resolve successfully (exact targets depend on move)
        assert!(result.is_ok());
    }

    #[test]
    fn test_move_target_resolution_doubles() {
        let format = BattleFormat::Doubles;
        let resolver = FormatMoveTargetResolver::new(format);
        let state = State::default();
        
        // Test move targeting in doubles
        let choice = MoveChoice::Move(crate::state::PokemonMoveIndex::M0);
        let result = resolver.resolve_move_targets(
            SideReference::SideOne,
            0,
            &choice,
            &state,
        );
        
        // Should resolve successfully
        assert!(result.is_ok());
    }

    #[test]
    fn test_enhanced_move_choice_conversion() {
        let format = BattleFormat::Singles;
        let resolver = FormatMoveTargetResolver::new(format);
        let state = State::default();
        
        let legacy_choice = MoveChoice::Move(crate::state::PokemonMoveIndex::M0);
        let enhanced = resolver.enhance_move_choice(
            legacy_choice,
            SideReference::SideOne,
            0,
            &state,
        );
        
        assert!(enhanced.is_ok());
        let enhanced = enhanced.unwrap();
        
        // Should convert back to original choice
        assert_eq!(enhanced.to_legacy(), legacy_choice);
    }

    #[test]
    fn test_auto_targeting_engine() {
        let format = BattleFormat::Doubles;
        let engine = AutoTargetingEngine::new(format);
        let state = State::default();
        
        // Test auto-targeting for a multi-target move
        let result = engine.auto_resolve_targets(
            Choices::EARTHQUAKE,
            SideReference::SideOne,
            0,
            &state,
        );
        
        assert!(result.is_ok());
        let targets = result.unwrap();
        
        // Earthquake should hit multiple targets in doubles
        assert!(!targets.is_empty());
    }
}