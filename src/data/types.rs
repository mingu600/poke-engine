use crate::choices::Choices;
use crate::engine::abilities::Abilities;
use crate::engine::items::Items;
use crate::pokemon::PokemonName;
use crate::state::PokemonType;
use rustemon::model::pokemon::Pokemon as RustemonPokemon;
use rustemon::model::moves::Move as RustemonMove;
use rustemon::model::items::Item as RustemonItem;
use serde::{Deserialize, Serialize};

/// Move target enum that maps 1:1 with rustemon/PokeAPI move targets
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum MoveTarget {
    SpecificMove,           // specific-move (1)
    SelectedPokemonMeFirst, // selected-pokemon-me-first (2)
    Ally,                   // ally (3)
    UsersField,             // users-field (4)
    UserOrAlly,             // user-or-ally (5)
    OpponentsField,         // opponents-field (6)
    User,                   // user (7)
    RandomOpponent,         // random-opponent (8)
    AllOtherPokemon,        // all-other-pokemon (9)
    SelectedPokemon,        // selected-pokemon (10)
    AllOpponents,           // all-opponents (11)
    EntireField,            // entire-field (12)
    UserAndAllies,          // user-and-allies (13)
    AllPokemon,             // all-pokemon (14)
    AllAllies,              // all-allies (15)
    FaintingPokemon,        // fainting-pokemon (16)
}

impl MoveTarget {
    /// Convert from rustemon move target name to engine enum
    pub fn from_rustemon_name(name: &str) -> Option<Self> {
        match name {
            "specific-move" => Some(Self::SpecificMove),
            "selected-pokemon-me-first" => Some(Self::SelectedPokemonMeFirst),
            "ally" => Some(Self::Ally),
            "users-field" => Some(Self::UsersField),
            "user-or-ally" => Some(Self::UserOrAlly),
            "opponents-field" => Some(Self::OpponentsField),
            "user" => Some(Self::User),
            "random-opponent" => Some(Self::RandomOpponent),
            "all-other-pokemon" => Some(Self::AllOtherPokemon),
            "selected-pokemon" => Some(Self::SelectedPokemon),
            "all-opponents" => Some(Self::AllOpponents),
            "entire-field" => Some(Self::EntireField),
            "user-and-allies" => Some(Self::UserAndAllies),
            "all-pokemon" => Some(Self::AllPokemon),
            "all-allies" => Some(Self::AllAllies),
            "fainting-pokemon" => Some(Self::FaintingPokemon),
            _ => None,
        }
    }

    /// Convert engine enum to rustemon move target name
    pub fn to_rustemon_name(&self) -> &'static str {
        match self {
            Self::SpecificMove => "specific-move",
            Self::SelectedPokemonMeFirst => "selected-pokemon-me-first",
            Self::Ally => "ally",
            Self::UsersField => "users-field",
            Self::UserOrAlly => "user-or-ally",
            Self::OpponentsField => "opponents-field",
            Self::User => "user",
            Self::RandomOpponent => "random-opponent",
            Self::AllOtherPokemon => "all-other-pokemon",
            Self::SelectedPokemon => "selected-pokemon",
            Self::AllOpponents => "all-opponents",
            Self::EntireField => "entire-field",
            Self::UserAndAllies => "user-and-allies",
            Self::AllPokemon => "all-pokemon",
            Self::AllAllies => "all-allies",
            Self::FaintingPokemon => "fainting-pokemon",
        }
    }
}

/// Base stats block for Pokemon
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BaseStatBlock {
    pub hp: u16,
    pub attack: u16,
    pub defense: u16,
    pub special_attack: u16,
    pub special_defense: u16,
    pub speed: u16,
}

/// Engine-specific Pokemon data that composes rustemon data
/// Based on current v1 Pokemon struct fields from src/state.rs
#[derive(Debug, Clone)]
pub struct BattlePokemonData {
    pub id: PokemonName,                    // Engine Pokemon name enum
    pub types: (PokemonType, PokemonType),  // Engine type enum
    pub base_stats: BaseStatBlock,          // HP, Atk, Def, SpA, SpD, Spe
    pub abilities: Vec<Abilities>,          // Engine ability enum
    pub weight_kg: f32,                     // For weight-based moves
    // Rustemon source data for validation/reference
    pub rustemon_data: Option<RustemonPokemon>,
}

/// Engine-specific move data that composes rustemon data
#[derive(Debug, Clone)]
pub struct BattleMoveData {
    pub id: Choices,                        // Engine move enum
    pub power: Option<u8>,                  // Base power
    pub accuracy: Option<u8>,               // Accuracy percentage
    pub pp: u8,                            // Power Points
    pub category: crate::choices::MoveCategory, // Physical/Special/Status
    pub move_type: PokemonType,             // Move type
    pub target: MoveTarget,                 // Engine target enum (1:1 with rustemon)
    // Rustemon source data for validation/reference
    pub rustemon_data: Option<RustemonMove>,
}

/// Engine-specific item data that composes rustemon data
#[derive(Debug, Clone)]
pub struct BattleItemData {
    pub id: Items,                          // Engine item enum
    pub name: String,                       // Item name
    pub fling_power: Option<u8>,            // For Fling move
    pub fling_effect: Option<String>,       // For Fling move
    // Rustemon source data for validation/reference
    pub rustemon_data: Option<RustemonItem>,
}

/// Complete data bundle for engine initialization
#[derive(Debug, Clone)]
pub struct EngineDataBundle {
    pub pokemon: Vec<BattlePokemonData>,
    pub moves: Vec<BattleMoveData>,
    pub items: Vec<BattleItemData>,
    pub generation_metadata: GenerationMetadata,
}

/// Metadata about the data generation process
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationMetadata {
    pub rustemon_version: String,
    pub generation_timestamp: String,
    pub total_pokemon: usize,
    pub total_moves: usize,
    pub total_items: usize,
    pub data_source: String, // "rustemon" or "cached"
}

impl Default for BaseStatBlock {
    fn default() -> Self {
        Self {
            hp: 1,
            attack: 1,
            defense: 1,
            special_attack: 1,
            special_defense: 1,
            speed: 1,
        }
    }
}

impl BattlePokemonData {
    /// Create a new BattlePokemonData with minimal required fields
    pub fn new(
        id: PokemonName,
        types: (PokemonType, PokemonType),
        base_stats: BaseStatBlock,
        abilities: Vec<Abilities>,
        weight_kg: f32,
    ) -> Self {
        Self {
            id,
            types,
            base_stats,
            abilities,
            weight_kg,
            rustemon_data: None,
        }
    }

    /// Set the rustemon source data
    pub fn with_rustemon_data(mut self, data: RustemonPokemon) -> Self {
        self.rustemon_data = Some(data);
        self
    }
}

impl BattleMoveData {
    /// Create a new BattleMoveData with minimal required fields
    pub fn new(
        id: Choices,
        power: Option<u8>,
        accuracy: Option<u8>,
        pp: u8,
        category: crate::choices::MoveCategory,
        move_type: PokemonType,
        target: MoveTarget,
    ) -> Self {
        Self {
            id,
            power,
            accuracy,
            pp,
            category,
            move_type,
            target,
            rustemon_data: None,
        }
    }

    /// Set the rustemon source data
    pub fn with_rustemon_data(mut self, data: RustemonMove) -> Self {
        self.rustemon_data = Some(data);
        self
    }
}

impl BattleItemData {
    /// Create a new BattleItemData with minimal required fields
    pub fn new(
        id: Items,
        name: String,
        fling_power: Option<u8>,
        fling_effect: Option<String>,
    ) -> Self {
        Self {
            id,
            name,
            fling_power,
            fling_effect,
            rustemon_data: None,
        }
    }

    /// Set the rustemon source data
    pub fn with_rustemon_data(mut self, data: RustemonItem) -> Self {
        self.rustemon_data = Some(data);
        self
    }
}