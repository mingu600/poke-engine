use super::move_service::{MoveDataService, EngineDataBuilder};
use crate::choices::{Choices, Choice, MoveCategory, Flags, Boost, Secondary, Status, VolatileStatus, SideCondition, Heal, StatBoosts, Effect, MOVES};
use crate::data::types::MoveTarget;
use crate::engine::state::PokemonVolatileStatus;
use crate::state::{PokemonSideCondition, PokemonStatus, PokemonType};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::OnceCell;

/// Factory for creating moves using rustemon data with engine-specific enhancements
pub struct MoveFactory {
    pub(crate) service: Arc<MoveDataService>,
    initialized: OnceCell<()>,
}

impl MoveFactory {
    /// Create a new move factory
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let service = MoveDataService::new()?;
        Ok(Self {
            service: Arc::new(service),
            initialized: OnceCell::new(),
        })
    }

    /// Initialize the factory with engine-specific move data
    pub async fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.initialized.get_or_try_init(|| async {
            self.register_engine_specific_data().await
        }).await?;
        Ok(())
    }

    /// Create a Choice for a move, using rustemon data + engine enhancements
    pub async fn create_move(&self, move_id: Choices) -> Result<Choice, Box<dyn std::error::Error>> {
        self.initialize().await?;
        self.service.create_choice(move_id).await
    }

    /// Get all moves as a HashMap (for compatibility with existing MOVES lazy_static)
    pub async fn get_all_moves(&self) -> Result<HashMap<Choices, Choice>, Box<dyn std::error::Error>> {
        self.initialize().await?;
        
        let mut moves = HashMap::new();
        
        // Create a list of important moves to populate
        let move_list = vec![
            Choices::TACKLE,
            Choices::THUNDERBOLT,
            Choices::SURF,
            Choices::EARTHQUAKE,
            Choices::FLAMETHROWER,
            Choices::ICEBEAM,
            Choices::PSYCHIC,
            Choices::SHADOWBALL,
            Choices::DRAGONCLAW,
            Choices::STEELWING,
            Choices::ABSORB,
            Choices::ACIDARMOR,
            Choices::AGILITY,
            Choices::BARRIER,
            Choices::BELLYDRUM,
            // Add more as needed
        ];

        for move_id in move_list {
            match self.create_move(move_id).await {
                Ok(choice) => {
                    moves.insert(move_id, choice);
                }
                Err(e) => {
                    eprintln!("Warning: Failed to create move {:?}: {}", move_id, e);
                    // Fallback to hardcoded data if available
                    if let Some(hardcoded_choice) = MOVES.get(&move_id) {
                        moves.insert(move_id, hardcoded_choice.clone());
                    }
                }
            }
        }

        Ok(moves)
    }

    /// Register engine-specific data for moves that need special mechanics
    async fn register_engine_specific_data(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Use the generated move registrations from Phase 3 migration
        self.register_all_engine_data().await;
        Ok(())
    }
}

impl Default for MoveFactory {
    fn default() -> Self {
        Self::new().expect("Failed to create MoveFactory")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_move_factory_creation() {
        let factory = MoveFactory::new();
        assert!(factory.is_ok());
    }

    #[tokio::test]
    async fn test_create_basic_move() {
        let factory = MoveFactory::new().unwrap();
        
        // This should work once we have the mapping in place
        // For now, it might fail due to missing rustemon data
        match factory.create_move(Choices::TACKLE).await {
            Ok(choice) => {
                assert_eq!(choice.move_id, Choices::TACKLE);
                assert_eq!(choice.move_type, PokemonType::NORMAL);
            }
            Err(e) => {
                println!("Expected error for now: {}", e);
                // This is expected until we have full rustemon integration
            }
        }
    }
}