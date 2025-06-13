use poke_engine::data::types::{MoveTarget, BaseStatBlock};
use poke_engine::data::conversion::{convert_type_name, ConversionError};

#[test]
fn test_move_target_rustemon_mapping() {
    // Test that all move targets map correctly
    assert_eq!(
        MoveTarget::from_rustemon_name("all-opponents"),
        Some(MoveTarget::AllOpponents)
    );
    assert_eq!(
        MoveTarget::from_rustemon_name("selected-pokemon"),
        Some(MoveTarget::SelectedPokemon)
    );
    assert_eq!(
        MoveTarget::from_rustemon_name("user"),
        Some(MoveTarget::User)
    );
    assert_eq!(
        MoveTarget::from_rustemon_name("entire-field"),
        Some(MoveTarget::EntireField)
    );
    
    // Test reverse mapping
    assert_eq!(
        MoveTarget::AllOpponents.to_rustemon_name(),
        "all-opponents"
    );
    assert_eq!(
        MoveTarget::SelectedPokemon.to_rustemon_name(),
        "selected-pokemon"
    );
    
    // Test unknown target
    assert_eq!(
        MoveTarget::from_rustemon_name("unknown-target"),
        None
    );
}

#[test]
fn test_type_conversion() {
    // Test all Pokemon types convert correctly
    assert_eq!(convert_type_name("fire").unwrap(), poke_engine::state::PokemonType::FIRE);
    assert_eq!(convert_type_name("water").unwrap(), poke_engine::state::PokemonType::WATER);
    assert_eq!(convert_type_name("grass").unwrap(), poke_engine::state::PokemonType::GRASS);
    assert_eq!(convert_type_name("electric").unwrap(), poke_engine::state::PokemonType::ELECTRIC);
    assert_eq!(convert_type_name("psychic").unwrap(), poke_engine::state::PokemonType::PSYCHIC);
    assert_eq!(convert_type_name("ice").unwrap(), poke_engine::state::PokemonType::ICE);
    assert_eq!(convert_type_name("dragon").unwrap(), poke_engine::state::PokemonType::DRAGON);
    assert_eq!(convert_type_name("dark").unwrap(), poke_engine::state::PokemonType::DARK);
    assert_eq!(convert_type_name("fairy").unwrap(), poke_engine::state::PokemonType::FAIRY);
    assert_eq!(convert_type_name("fighting").unwrap(), poke_engine::state::PokemonType::FIGHTING);
    assert_eq!(convert_type_name("poison").unwrap(), poke_engine::state::PokemonType::POISON);
    assert_eq!(convert_type_name("ground").unwrap(), poke_engine::state::PokemonType::GROUND);
    assert_eq!(convert_type_name("flying").unwrap(), poke_engine::state::PokemonType::FLYING);
    assert_eq!(convert_type_name("bug").unwrap(), poke_engine::state::PokemonType::BUG);
    assert_eq!(convert_type_name("rock").unwrap(), poke_engine::state::PokemonType::ROCK);
    assert_eq!(convert_type_name("ghost").unwrap(), poke_engine::state::PokemonType::GHOST);
    assert_eq!(convert_type_name("steel").unwrap(), poke_engine::state::PokemonType::STEEL);
    assert_eq!(convert_type_name("normal").unwrap(), poke_engine::state::PokemonType::NORMAL);
    assert_eq!(convert_type_name("stellar").unwrap(), poke_engine::state::PokemonType::STELLAR);
    
    // Test unknown type
    assert!(matches!(convert_type_name("unknown"), Err(ConversionError::UnknownType(_))));
}

#[test]
fn test_base_stat_block() {
    let stats = BaseStatBlock {
        hp: 78,
        attack: 84,
        defense: 78,
        special_attack: 109,
        special_defense: 85,
        speed: 100,
    };
    
    // Test that stats are stored correctly
    assert_eq!(stats.hp, 78);
    assert_eq!(stats.attack, 84);
    assert_eq!(stats.defense, 78);
    assert_eq!(stats.special_attack, 109);
    assert_eq!(stats.special_defense, 85);
    assert_eq!(stats.speed, 100);
}

#[test]
fn test_default_base_stat_block() {
    let stats = BaseStatBlock::default();
    
    // Test that default stats are all 1 (minimum valid stats)
    assert_eq!(stats.hp, 1);
    assert_eq!(stats.attack, 1);
    assert_eq!(stats.defense, 1);
    assert_eq!(stats.special_attack, 1);
    assert_eq!(stats.special_defense, 1);
    assert_eq!(stats.speed, 1);
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[test]
    fn test_data_structures_compile() {
        // This test ensures our data structures are properly defined
        // and can be instantiated without runtime issues
        
        use poke_engine::data::types::{BattlePokemonData, BattleMoveData, BattleItemData};
        use poke_engine::pokemon::PokemonName;
        use poke_engine::state::PokemonType;
        use poke_engine::engine::abilities::Abilities;
        use poke_engine::choices::Choices;
        use poke_engine::engine::items::Items;
        use poke_engine::choices::MoveCategory;
        
        // Test Pokemon data creation
        let pokemon_data = BattlePokemonData::new(
            PokemonName::PIKACHU,
            (PokemonType::ELECTRIC, PokemonType::ELECTRIC),
            BaseStatBlock {
                hp: 35,
                attack: 55,
                defense: 40,
                special_attack: 50,
                special_defense: 50,
                speed: 90,
            },
            vec![Abilities::STATIC],
            6.0, // 6kg
        );
        
        assert_eq!(pokemon_data.id, PokemonName::PIKACHU);
        assert_eq!(pokemon_data.types.0, PokemonType::ELECTRIC);
        assert_eq!(pokemon_data.weight_kg, 6.0);
        
        // Test Move data creation
        let move_data = BattleMoveData::new(
            Choices::THUNDERBOLT,
            Some(90), // power
            Some(100), // accuracy
            15, // pp
            MoveCategory::Special,
            PokemonType::ELECTRIC,
            MoveTarget::SelectedPokemon,
        );
        
        assert_eq!(move_data.id, Choices::THUNDERBOLT);
        assert_eq!(move_data.power, Some(90));
        assert_eq!(move_data.category, MoveCategory::Special);
        
        // Test Item data creation
        let item_data = BattleItemData::new(
            Items::LEFTOVERS,
            "Leftovers".to_string(),
            Some(10), // fling power
            None, // no special fling effect
        );
        
        assert_eq!(item_data.id, Items::LEFTOVERS);
        assert_eq!(item_data.name, "Leftovers");
        assert_eq!(item_data.fling_power, Some(10));
    }
}