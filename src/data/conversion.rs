use super::types::{BattlePokemonData, BattleMoveData, BattleItemData, BaseStatBlock, MoveTarget};
use crate::choices::{Choices, MoveCategory};
use crate::engine::abilities::Abilities;
use crate::engine::items::Items;
use crate::pokemon::PokemonName;
use crate::state::PokemonType;
use rustemon::model::pokemon::{Pokemon as RustemonPokemon, PokemonStat};
use rustemon::model::moves::Move as RustemonMove;
use rustemon::model::items::Item as RustemonItem;
use std::str::FromStr;

/// Error type for conversion operations
#[derive(Debug)]
pub enum ConversionError {
    UnknownPokemon(String),
    UnknownMove(String),
    UnknownItem(String),
    UnknownType(String),
    UnknownAbility(String),
    UnknownMoveTarget(String),
    UnknownMoveCategory(String),
    MissingData(String),
}

impl std::fmt::Display for ConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownPokemon(name) => write!(f, "Unknown Pokemon: {}", name),
            Self::UnknownMove(name) => write!(f, "Unknown Move: {}", name),
            Self::UnknownItem(name) => write!(f, "Unknown Item: {}", name),
            Self::UnknownType(name) => write!(f, "Unknown Type: {}", name),
            Self::UnknownAbility(name) => write!(f, "Unknown Ability: {}", name),
            Self::UnknownMoveTarget(name) => write!(f, "Unknown Move Target: {}", name),
            Self::UnknownMoveCategory(name) => write!(f, "Unknown Move Category: {}", name),
            Self::MissingData(field) => write!(f, "Missing required data: {}", field),
        }
    }
}

impl std::error::Error for ConversionError {}

/// Convert rustemon Pokemon to engine BattlePokemonData
pub fn convert_pokemon(rustemon_pokemon: RustemonPokemon) -> Result<BattlePokemonData, ConversionError> {
    let name = &rustemon_pokemon.name;

    // Convert name to engine PokemonName enum
    let normalized_name = normalize_pokemon_name(name);
    let pokemon_id = PokemonName::from_str(&normalized_name.to_uppercase())
        .map_err(|_| ConversionError::UnknownPokemon(name.clone()))?;

    // Convert types
    let types = convert_pokemon_types(&rustemon_pokemon)?;

    // Convert base stats
    let base_stats = convert_base_stats(&rustemon_pokemon.stats)?;

    // Convert abilities
    let abilities = convert_pokemon_abilities(&rustemon_pokemon)?;

    // Get weight (rustemon weight is in hectograms, convert to kg)
    let weight_kg = rustemon_pokemon.weight as f32 / 10.0;

    Ok(BattlePokemonData::new(
        pokemon_id,
        types,
        base_stats,
        abilities,
        weight_kg,
    ).with_rustemon_data(rustemon_pokemon))
}

/// Convert rustemon Move to engine BattleMoveData
pub fn convert_move(rustemon_move: RustemonMove) -> Result<BattleMoveData, ConversionError> {
    let name = &rustemon_move.name;

    // Convert name to engine Choices enum
    let normalized_name = normalize_move_name(name);
    let move_id = Choices::from_str(&normalized_name.to_uppercase())
        .map_err(|_| ConversionError::UnknownMove(name.clone()))?;

    // Get move properties
    let power = rustemon_move.power.map(|p| p as u8);
    let accuracy = rustemon_move.accuracy.map(|a| a as u8);
    let pp = rustemon_move.pp.unwrap_or(5) as u8;

    // Convert damage class to category
    let category = convert_move_category(&rustemon_move)?;

    // Convert type
    let move_type = convert_move_type(&rustemon_move)?;

    // Convert target
    let target = convert_move_target(&rustemon_move)?;

    Ok(BattleMoveData::new(
        move_id,
        power,
        accuracy,
        pp,
        category,
        move_type,
        target,
    ).with_rustemon_data(rustemon_move))
}

/// Convert rustemon Item to engine BattleItemData
pub fn convert_item(rustemon_item: RustemonItem) -> Result<BattleItemData, ConversionError> {
    let name = &rustemon_item.name;

    // Convert name to engine Items enum
    let normalized_name = normalize_item_name(name);
    let item_id = Items::from_str(&normalized_name.to_uppercase())
        .map_err(|_| ConversionError::UnknownItem(name.clone()))?;

    // Get fling properties
    let fling_power = rustemon_item.fling_power.map(|p| p as u8);
    let fling_effect = rustemon_item.fling_effect
        .as_ref()
        .map(|effect| effect.name.clone());

    Ok(BattleItemData::new(
        item_id,
        name.clone(),
        fling_power,
        fling_effect,
    ).with_rustemon_data(rustemon_item))
}

// Helper functions for specific conversions

fn convert_pokemon_types(pokemon: &RustemonPokemon) -> Result<(PokemonType, PokemonType), ConversionError> {
    let mut types = (PokemonType::TYPELESS, PokemonType::TYPELESS);
    
    for pokemon_type in &pokemon.types {
        let type_name = &pokemon_type.type_.name;
        let engine_type = convert_type_name(type_name)?;
        
        if pokemon_type.slot == 1 {
            types.0 = engine_type;
        } else if pokemon_type.slot == 2 {
            types.1 = engine_type;
        }
    }
    
    // If only one type, set both to the same type
    if types.1 == PokemonType::TYPELESS && types.0 != PokemonType::TYPELESS {
        types.1 = types.0;
    }
    
    Ok(types)
}

fn convert_base_stats(base_stats: &[PokemonStat]) -> Result<BaseStatBlock, ConversionError> {
    let mut stats = BaseStatBlock::default();
    
    for stat in base_stats {
        let stat_name = &stat.stat.name;
        let base_stat = stat.base_stat as u16;
        
        match stat_name.as_str() {
            "hp" => stats.hp = base_stat,
            "attack" => stats.attack = base_stat,
            "defense" => stats.defense = base_stat,
            "special-attack" => stats.special_attack = base_stat,
            "special-defense" => stats.special_defense = base_stat,
            "speed" => stats.speed = base_stat,
            _ => {} // Ignore unknown stats
        }
    }
    
    Ok(stats)
}

fn convert_pokemon_abilities(pokemon: &RustemonPokemon) -> Result<Vec<Abilities>, ConversionError> {
    let mut abilities = Vec::new();
    
    for ability in &pokemon.abilities {
        let ability_name = &ability.ability.name;
        let normalized_name = normalize_ability_name(ability_name);
        if let Ok(engine_ability) = Abilities::from_str(&normalized_name.to_uppercase()) {
            abilities.push(engine_ability);
        }
    }
    
    // Ensure at least one ability
    if abilities.is_empty() {
        abilities.push(Abilities::NONE);
    }
    
    Ok(abilities)
}

fn convert_move_category(rustemon_move: &RustemonMove) -> Result<MoveCategory, ConversionError> {
    let class_name = &rustemon_move.damage_class.name;
    match class_name.as_str() {
        "physical" => Ok(MoveCategory::Physical),
        "special" => Ok(MoveCategory::Special),
        "status" => Ok(MoveCategory::Status),
        _ => Err(ConversionError::UnknownMoveCategory(class_name.clone())),
    }
}

fn convert_move_type(rustemon_move: &RustemonMove) -> Result<PokemonType, ConversionError> {
    let type_name = &rustemon_move.type_.name;
    convert_type_name(type_name)
}

fn convert_move_target(rustemon_move: &RustemonMove) -> Result<MoveTarget, ConversionError> {
    let target_name = &rustemon_move.target.name;
    MoveTarget::from_rustemon_name(target_name)
        .ok_or_else(|| ConversionError::UnknownMoveTarget(target_name.to_string()))
}

pub fn convert_type_name(type_name: &str) -> Result<PokemonType, ConversionError> {
    match type_name {
        "normal" => Ok(PokemonType::NORMAL),
        "fire" => Ok(PokemonType::FIRE),
        "water" => Ok(PokemonType::WATER),
        "electric" => Ok(PokemonType::ELECTRIC),
        "grass" => Ok(PokemonType::GRASS),
        "ice" => Ok(PokemonType::ICE),
        "fighting" => Ok(PokemonType::FIGHTING),
        "poison" => Ok(PokemonType::POISON),
        "ground" => Ok(PokemonType::GROUND),
        "flying" => Ok(PokemonType::FLYING),
        "psychic" => Ok(PokemonType::PSYCHIC),
        "bug" => Ok(PokemonType::BUG),
        "rock" => Ok(PokemonType::ROCK),
        "ghost" => Ok(PokemonType::GHOST),
        "dragon" => Ok(PokemonType::DRAGON),
        "dark" => Ok(PokemonType::DARK),
        "steel" => Ok(PokemonType::STEEL),
        "fairy" => Ok(PokemonType::FAIRY),
        "stellar" => Ok(PokemonType::STELLAR),
        _ => Err(ConversionError::UnknownType(type_name.to_string())),
    }
}

// Name normalization functions (similar to battle_environment.rs)

fn normalize_pokemon_name(name: &str) -> String {
    name.replace(" ", "")
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

fn normalize_move_name(name: &str) -> String {
    name.replace(" ", "")
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

fn normalize_item_name(name: &str) -> String {
    name.replace(" ", "")
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

fn normalize_ability_name(name: &str) -> String {
    name.replace(" ", "")
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_target_conversion() {
        assert_eq!(
            MoveTarget::from_rustemon_name("all-opponents"),
            Some(MoveTarget::AllOpponents)
        );
        assert_eq!(
            MoveTarget::AllOpponents.to_rustemon_name(),
            "all-opponents"
        );
    }

    #[test]
    fn test_type_conversion() {
        assert_eq!(convert_type_name("fire").unwrap(), PokemonType::FIRE);
        assert_eq!(convert_type_name("water").unwrap(), PokemonType::WATER);
        assert!(convert_type_name("unknown").is_err());
    }

    #[test]
    fn test_name_normalization() {
        assert_eq!(normalize_pokemon_name("Mr. Mime"), "mrmime");
        assert_eq!(normalize_move_name("Thunder Bolt"), "thunderbolt");
        assert_eq!(normalize_item_name("Poké Ball"), "pokeball");
    }
}