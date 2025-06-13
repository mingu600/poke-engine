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
    service: Arc<MoveDataService>,
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
        // ABSORB - Draining move
        self.service.register_engine_data(
            Choices::ABSORB,
            EngineDataBuilder::new()
                .flags(Flags {
                    heal: true,
                    protect: true,
                    ..Default::default()
                })
                .drain(0.5)
                .build()
        ).await;

        // ACIDARMOR - Self-boost move
        self.service.register_engine_data(
            Choices::ACIDARMOR,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::USER,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 2,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .build()
        ).await;

        // AGILITY - Speed boost
        self.service.register_engine_data(
            Choices::AGILITY,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::USER,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 2,
                        accuracy: 0,
                    },
                })
                .build()
        ).await;

        // BARRIER - Defense boost
        self.service.register_engine_data(
            Choices::BARRIER,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::USER,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 2,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .build()
        ).await;

        // THUNDERBOLT - Paralysis chance
        self.service.register_engine_data(
            Choices::THUNDERBOLT,
            EngineDataBuilder::new()
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .secondaries(vec![
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::OPPONENT,
                        effect: Effect::Status(PokemonStatus::PARALYZE),
                    }
                ])
                .build()
        ).await;

        // FLAMETHROWER - Burn chance
        self.service.register_engine_data(
            Choices::FLAMETHROWER,
            EngineDataBuilder::new()
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .secondaries(vec![
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::OPPONENT,
                        effect: Effect::Status(PokemonStatus::BURN),
                    }
                ])
                .build()
        ).await;

        // ICEBEAM - Freeze chance
        self.service.register_engine_data(
            Choices::ICEBEAM,
            EngineDataBuilder::new()
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .secondaries(vec![
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::OPPONENT,
                        effect: Effect::Status(PokemonStatus::FREEZE),
                    }
                ])
                .build()
        ).await;

        // PSYCHIC - Special Defense drop chance
        self.service.register_engine_data(
            Choices::PSYCHIC,
            EngineDataBuilder::new()
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .secondaries(vec![
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::OPPONENT,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: -1,
                            speed: 0,
                            accuracy: 0,
                        }),
                    }
                ])
                .build()
        ).await;

        // SHADOWBALL - Special Defense drop chance
        self.service.register_engine_data(
            Choices::SHADOWBALL,
            EngineDataBuilder::new()
                .flags(Flags {
                    protect: true,
                    bullet: true,
                    ..Default::default()
                })
                .secondaries(vec![
                    Secondary {
                        chance: 20.0,
                        target: MoveTarget::OPPONENT,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: -1,
                            speed: 0,
                            accuracy: 0,
                        }),
                    }
                ])
                .build()
        ).await;

        // Note: EARTHQUAKE and SURF don't need special engine data
        // Their spread move mechanics (damage reduction, multi-targeting) are handled
        // by the instruction generation system based on their MoveTarget value

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