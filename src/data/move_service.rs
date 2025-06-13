use super::types::BattleMoveData;
use super::rustemon_client::PokeEngineDataClient;
use super::conversion::convert_move;
use crate::choices::{Choices, Choice, Flags, Boost, Secondary, Status, VolatileStatus, SideCondition, Heal};
use crate::state::{PokemonMoveIndex, PokemonIndex};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Service for managing move data from rustemon with engine-specific enhancements
pub struct MoveDataService {
    client: Arc<PokeEngineDataClient>,
    cache: Arc<RwLock<HashMap<Choices, BattleMoveData>>>,
    engine_data: Arc<RwLock<HashMap<Choices, EngineSpecificMoveData>>>,
}

/// Engine-specific data that supplements rustemon data
#[derive(Debug, Clone, Default)]
pub struct EngineSpecificMoveData {
    pub priority: i8,
    pub flags: Flags,
    pub drain: Option<f32>,
    pub recoil: Option<f32>,
    pub crash: Option<f32>,
    pub heal: Option<Heal>,
    pub status: Option<Status>,
    pub volatile_status: Option<VolatileStatus>,
    pub side_condition: Option<SideCondition>,
    pub boost: Option<Boost>,
    pub secondaries: Option<Vec<Secondary>>,
}

impl MoveDataService {
    /// Create a new move data service
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let client = PokeEngineDataClient::new()?;
        Ok(Self {
            client: Arc::new(client),
            cache: Arc::new(RwLock::new(HashMap::new())),
            engine_data: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Get move data for a specific move, fetching from rustemon if needed
    pub async fn get_move_data(&self, move_id: Choices) -> Result<BattleMoveData, Box<dyn std::error::Error>> {
        // Check cache first
        {
            let cache = self.cache.read().await;
            if let Some(data) = cache.get(&move_id) {
                return Ok(data.clone());
            }
        }

        // Fetch from rustemon
        let move_name = self.get_rustemon_move_name(move_id);
        let rustemon_move = self.client.get_move(&move_name).await?;
        let battle_move_data = convert_move(rustemon_move)?;

        // Cache the result
        {
            let mut cache = self.cache.write().await;
            cache.insert(move_id, battle_move_data.clone());
        }

        Ok(battle_move_data)
    }

    /// Create a Choice from BattleMoveData and engine-specific data
    pub async fn create_choice(&self, move_id: Choices) -> Result<Choice, Box<dyn std::error::Error>> {
        let battle_move_data = self.get_move_data(move_id).await?;
        let engine_data = self.get_engine_data(move_id).await;

        Ok(Choice {
            move_id,
            move_index: PokemonMoveIndex::M0,
            switch_id: PokemonIndex::P0,
            move_type: battle_move_data.move_type,
            accuracy: battle_move_data.accuracy.map(|a| a as f32).unwrap_or(100.0),
            category: battle_move_data.category,
            base_power: battle_move_data.power.map(|p| p as f32).unwrap_or(0.0),
            target: battle_move_data.target,
            
            // Engine-specific data
            priority: engine_data.priority,
            flags: engine_data.flags,
            drain: engine_data.drain,
            recoil: engine_data.recoil,
            crash: engine_data.crash,
            heal: engine_data.heal,
            status: engine_data.status,
            volatile_status: engine_data.volatile_status,
            side_condition: engine_data.side_condition,
            boost: engine_data.boost,
            secondaries: engine_data.secondaries,
            
            // Runtime fields
            first_move: false,
            sleep_talk_move: false,
        })
    }

    /// Register engine-specific data for a move
    pub async fn register_engine_data(&self, move_id: Choices, data: EngineSpecificMoveData) {
        let mut engine_data = self.engine_data.write().await;
        engine_data.insert(move_id, data);
    }

    /// Get engine-specific data for a move
    async fn get_engine_data(&self, move_id: Choices) -> EngineSpecificMoveData {
        let engine_data = self.engine_data.read().await;
        engine_data.get(&move_id).cloned().unwrap_or_default()
    }

    /// Convert engine move enum to rustemon move name
    /// TODO: This should be driven by a mapping table or derived from rustemon data
    fn get_rustemon_move_name(&self, move_id: Choices) -> String {
        match move_id {
            Choices::TACKLE => "tackle".to_string(),
            Choices::THUNDERBOLT => "thunderbolt".to_string(),
            Choices::SURF => "surf".to_string(),
            Choices::EARTHQUAKE => "earthquake".to_string(),
            Choices::FLAMETHROWER => "flamethrower".to_string(),
            Choices::ICEBEAM => "ice-beam".to_string(),
            Choices::PSYCHIC => "psychic".to_string(),
            Choices::SHADOWBALL => "shadow-ball".to_string(),
            Choices::DRAGONCLAW => "dragon-claw".to_string(),
            Choices::STEELWING => "steel-wing".to_string(),
            // Add more mappings as needed
            _ => {
                // Fallback: convert enum name to kebab-case
                let name = format!("{:?}", move_id);
                name.to_lowercase().replace("_", "-")
            }
        }
    }
}

/// Builder for engine-specific move data
pub struct EngineDataBuilder {
    data: EngineSpecificMoveData,
}

impl EngineDataBuilder {
    pub fn new() -> Self {
        Self {
            data: EngineSpecificMoveData::default(),
        }
    }

    pub fn priority(mut self, priority: i8) -> Self {
        self.data.priority = priority;
        self
    }

    pub fn flags(mut self, flags: Flags) -> Self {
        self.data.flags = flags;
        self
    }

    pub fn drain(mut self, drain: f32) -> Self {
        self.data.drain = Some(drain);
        self
    }

    pub fn recoil(mut self, recoil: f32) -> Self {
        self.data.recoil = Some(recoil);
        self
    }

    pub fn crash(mut self, crash: f32) -> Self {
        self.data.crash = Some(crash);
        self
    }

    pub fn heal(mut self, heal: Heal) -> Self {
        self.data.heal = Some(heal);
        self
    }

    pub fn status(mut self, status: Status) -> Self {
        self.data.status = Some(status);
        self
    }

    pub fn volatile_status(mut self, volatile_status: VolatileStatus) -> Self {
        self.data.volatile_status = Some(volatile_status);
        self
    }

    pub fn side_condition(mut self, side_condition: SideCondition) -> Self {
        self.data.side_condition = Some(side_condition);
        self
    }

    pub fn boost(mut self, boost: Boost) -> Self {
        self.data.boost = Some(boost);
        self
    }

    pub fn secondaries(mut self, secondaries: Vec<Secondary>) -> Self {
        self.data.secondaries = Some(secondaries);
        self
    }

    pub fn build(self) -> EngineSpecificMoveData {
        self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_move_data_service_creation() {
        let service = MoveDataService::new();
        assert!(service.is_ok());
    }

    #[tokio::test] 
    async fn test_engine_data_builder() {
        let data = EngineDataBuilder::new()
            .priority(1)
            .drain(0.5)
            .build();
        
        assert_eq!(data.priority, 1);
        assert_eq!(data.drain, Some(0.5));
    }
}