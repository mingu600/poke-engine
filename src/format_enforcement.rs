use crate::battle_format::{BattleFormat, FormatClause};
use crate::state::{State, SideReference, PokemonStatus};
use crate::instruction::Instruction;
use crate::choices::Choices;

/// Enforces format rules during battle
#[derive(Debug)]
pub struct FormatEnforcer {
    format: BattleFormat,
}

impl FormatEnforcer {
    pub fn new(format: BattleFormat) -> Self {
        FormatEnforcer { format }
    }
    
    /// Check if an instruction violates format rules
    pub fn validate_instruction(
        &self,
        instruction: &Instruction,
        state: &State,
    ) -> Result<(), String> {
        let rules = self.format.get_rules();
        
        match instruction {
            Instruction::ChangeStatus(status_instruction) => {
                // Check sleep clause
                if rules.format_clauses.contains(&FormatClause::SleepClause) 
                    && status_instruction.new_status == PokemonStatus::SLEEP {
                    
                    let side = state.get_side_immutable(&status_instruction.side_ref);
                    let sleeping_count = side.pokemon.into_iter()
                        .filter(|p| p.status == PokemonStatus::SLEEP && p.hp > 0)
                        .count();
                    
                    // Typically sleep clause allows only 1 sleeping Pokemon
                    if sleeping_count >= 1 {
                        return Err("Sleep Clause: Cannot put more than one opponent's Pokemon to sleep".to_string());
                    }
                }
                
                // Check freeze clause
                if rules.format_clauses.contains(&FormatClause::FreezeClause) 
                    && status_instruction.new_status == PokemonStatus::FREEZE {
                    
                    let side = state.get_side_immutable(&status_instruction.side_ref);
                    let frozen_count = side.pokemon.into_iter()
                        .filter(|p| p.status == PokemonStatus::FREEZE && p.hp > 0)
                        .count();
                    
                    // Typically freeze clause allows only 1 frozen Pokemon
                    if frozen_count >= 1 {
                        return Err("Freeze Clause: Cannot freeze more than one opponent's Pokemon".to_string());
                    }
                }
            }
            _ => {}
        }
        
        Ok(())
    }
    
    /// Check if a move choice is valid for the format
    pub fn validate_move_choice(
        &self,
        move_choice: &Choices,
        _user_side: SideReference,
        _state: &State,
    ) -> Result<(), String> {
        let rules = self.format.get_rules();
        
        // Check OHKO clause
        if rules.format_clauses.contains(&FormatClause::OHKOClause) {
            if matches!(move_choice, 
                Choices::FISSURE | 
                Choices::GUILLOTINE | 
                Choices::HORNDRILL | 
                Choices::SHEERCOLD
            ) {
                return Err("OHKO Clause: One-hit KO moves are banned".to_string());
            }
        }
        
        // Check evasion clause
        if rules.format_clauses.contains(&FormatClause::EvasionClause) {
            if matches!(move_choice,
                Choices::DOUBLETEAM |
                Choices::MINIMIZE
            ) {
                return Err("Evasion Clause: Evasion-boosting moves are banned".to_string());
            }
        }
        
        Ok(())
    }
    
    /// Get the maximum number of active Pokemon per side for this format
    pub fn get_active_pokemon_limit(&self) -> usize {
        self.format.get_rules().active_pokemon
    }
    
    /// Check if the battle state is valid for the format
    pub fn validate_battle_state(&self, state: &State) -> Result<(), String> {
        crate::battle_format::FormatValidator::validate_battle_state(state, &self.format)
    }
}

/// Tracks format-specific state during battle
#[derive(Debug)]
pub struct FormatStateTracker {
    format: BattleFormat,
    sleep_count_side_one: usize,
    sleep_count_side_two: usize,
    freeze_count_side_one: usize,
    freeze_count_side_two: usize,
}

impl FormatStateTracker {
    pub fn new(format: BattleFormat) -> Self {
        FormatStateTracker {
            format,
            sleep_count_side_one: 0,
            sleep_count_side_two: 0,
            freeze_count_side_one: 0,
            freeze_count_side_two: 0,
        }
    }
    
    /// Update tracker based on state changes
    pub fn update_from_state(&mut self, state: &State) {
        self.sleep_count_side_one = state.side_one.pokemon.into_iter()
            .filter(|p| p.status == PokemonStatus::SLEEP && p.hp > 0)
            .count();
            
        self.sleep_count_side_two = state.side_two.pokemon.into_iter()
            .filter(|p| p.status == PokemonStatus::SLEEP && p.hp > 0)
            .count();
            
        self.freeze_count_side_one = state.side_one.pokemon.into_iter()
            .filter(|p| p.status == PokemonStatus::FREEZE && p.hp > 0)
            .count();
            
        self.freeze_count_side_two = state.side_two.pokemon.into_iter()
            .filter(|p| p.status == PokemonStatus::FREEZE && p.hp > 0)
            .count();
    }
    
    /// Check if a status change would violate format rules
    pub fn can_inflict_status(
        &self,
        status: PokemonStatus,
        target_side: SideReference,
    ) -> bool {
        let rules = self.format.get_rules();
        
        match status {
            PokemonStatus::SLEEP => {
                if rules.format_clauses.contains(&FormatClause::SleepClause) {
                    match target_side {
                        SideReference::SideOne => self.sleep_count_side_one < 1,
                        SideReference::SideTwo => self.sleep_count_side_two < 1,
                    }
                } else {
                    true
                }
            }
            PokemonStatus::FREEZE => {
                if rules.format_clauses.contains(&FormatClause::FreezeClause) {
                    match target_side {
                        SideReference::SideOne => self.freeze_count_side_one < 1,
                        SideReference::SideTwo => self.freeze_count_side_two < 1,
                    }
                } else {
                    true
                }
            }
            _ => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sleep_clause_enforcement() {
        let format = BattleFormat::Singles;
        let enforcer = FormatEnforcer::new(format);
        let mut state = State::default();
        
        // Put first Pokemon to sleep - should be allowed
        state.side_one.pokemon.p0.status = PokemonStatus::SLEEP;
        state.side_one.pokemon.p0.hp = 100;
        
        // Try to put second Pokemon to sleep - should be blocked
        let instruction = Instruction::ChangeStatus(crate::instruction::ChangeStatusInstruction {
            side_ref: SideReference::SideOne,
            pokemon_index: crate::state::PokemonIndex::P1,
            old_status: PokemonStatus::NONE,
            new_status: PokemonStatus::SLEEP,
        });
        
        let result = enforcer.validate_instruction(&instruction, &state);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Sleep Clause"));
    }
    
    #[test]
    fn test_ohko_clause() {
        let format = BattleFormat::Singles;
        let enforcer = FormatEnforcer::new(format);
        let state = State::default();
        
        // OHKO moves should be blocked
        let result = enforcer.validate_move_choice(
            &Choices::FISSURE,
            SideReference::SideOne,
            &state
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("OHKO Clause"));
        
        // Regular moves should be allowed
        let result = enforcer.validate_move_choice(
            &Choices::TACKLE,
            SideReference::SideOne,
            &state
        );
        assert!(result.is_ok());
    }
}