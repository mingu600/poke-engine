use poke_engine::format_config::*;
use poke_engine::battle_format::{BattleFormat, BattleRules};
use poke_engine::state::State;
use poke_engine::pokemon::PokemonName;
use poke_engine::engine::items::Items;
use poke_engine::choices::Choices;

#[test]
fn test_format_config_defaults() {
    let config = FormatConfig::default();
    assert!(config.banned_pokemon.is_empty());
    assert!(config.banned_items.is_empty());
    assert!(config.banned_moves.is_empty());
    assert!(config.restricted_pokemon.is_empty());
    assert_eq!(config.restricted_limit, 0);
}

#[test]
fn test_singles_ou_config() {
    let config = FORMAT_CONFIGS.get("singles_ou").unwrap();
    
    // Check some banned Pokemon
    assert!(config.banned_pokemon.contains(&PokemonName::MEWTWO));
    assert!(config.banned_pokemon.contains(&PokemonName::RAYQUAZA));
    assert!(config.banned_pokemon.contains(&PokemonName::ARCEUS));
    assert!(config.banned_pokemon.contains(&PokemonName::ZACIANCROWNED));
    
    // Check banned moves
    assert!(config.banned_moves.contains(&Choices::FISSURE));
    assert!(config.banned_moves.contains(&Choices::GUILLOTINE));
    assert!(config.banned_moves.contains(&Choices::DOUBLETEAM));
    assert!(config.banned_moves.contains(&Choices::MINIMIZE));
    assert!(config.banned_moves.contains(&Choices::BATONPASS));
    
    // No restricted Pokemon in OU
    assert!(config.restricted_pokemon.is_empty());
    assert_eq!(config.restricted_limit, 0);
}

#[test]
fn test_vgc_2024_reg_g_config() {
    let config = FORMAT_CONFIGS.get("vgc_2024_reg_g").unwrap();
    
    // Check mythical bans
    assert!(config.banned_pokemon.contains(&PokemonName::MEW));
    assert!(config.banned_pokemon.contains(&PokemonName::CELEBI));
    assert!(config.banned_pokemon.contains(&PokemonName::JIRACHI));
    assert!(config.banned_pokemon.contains(&PokemonName::ARCEUS));
    
    // Check OHKO moves are banned
    assert!(config.banned_moves.contains(&Choices::FISSURE));
    assert!(config.banned_moves.contains(&Choices::SHEERCOLD));
    
    // Check restricted legendaries
    assert!(config.restricted_pokemon.contains(&PokemonName::MEWTWO));
    assert!(config.restricted_pokemon.contains(&PokemonName::KYOGRE));
    assert!(config.restricted_pokemon.contains(&PokemonName::GROUDON));
    assert!(config.restricted_pokemon.contains(&PokemonName::RAYQUAZA));
    assert!(config.restricted_pokemon.contains(&PokemonName::ZACIANCROWNED));
    assert!(config.restricted_pokemon.contains(&PokemonName::CALYREXICE));
    
    // VGC allows 2 restricted Pokemon
    assert_eq!(config.restricted_limit, 2);
}

#[test]
fn test_doubles_ou_config() {
    let config = FORMAT_CONFIGS.get("doubles_ou").unwrap();
    
    // Check banned Pokemon
    assert!(config.banned_pokemon.contains(&PokemonName::MEWTWO));
    assert!(config.banned_pokemon.contains(&PokemonName::KYOGREPRIMAL));
    assert!(config.banned_pokemon.contains(&PokemonName::GROUDONPRIMAL));
    
    // Check Dark Void is banned in doubles
    assert!(config.banned_moves.contains(&Choices::DARKVOID));
    
    // No restricted Pokemon in doubles OU
    assert!(config.restricted_pokemon.is_empty());
}

#[test]
fn test_battle_format_factory() {
    // Test factory methods
    let singles = BattleFormatFactory::singles_ou();
    assert_eq!(singles, BattleFormat::Singles);
    
    let doubles = BattleFormatFactory::doubles_ou();
    assert_eq!(doubles, BattleFormat::Doubles);
    
    let vgc = BattleFormatFactory::vgc_2024_reg_g();
    assert_eq!(vgc, BattleFormat::VGC);
    
    // Test custom format
    let custom_rules = BattleRules {
        party_size: 3,
        team_size: 3,
        active_pokemon: 1,
        format_clauses: vec![],
    };
    let custom = BattleFormatFactory::custom(custom_rules.clone());
    match custom {
        BattleFormat::Custom(rules) => assert_eq!(rules, custom_rules),
        _ => panic!("Expected Custom format"),
    }
}

#[test]
fn test_from_config_name() {
    // Test valid config names
    assert!(BattleFormatFactory::from_config_name("singles_ou").is_some());
    assert!(BattleFormatFactory::from_config_name("doubles_ou").is_some());
    assert!(BattleFormatFactory::from_config_name("vgc_2024_reg_g").is_some());
    
    // Test invalid config name
    assert!(BattleFormatFactory::from_config_name("invalid_format").is_none());
}

#[test]
fn test_config_based_validator_banned_pokemon() {
    let mut state = State::default();
    let format = BattleFormat::Singles;
    
    // Set up a banned Pokemon
    state.side_one.pokemon.p0.id = PokemonName::MEWTWO;
    
    let result = ConfigBasedValidator::validate_with_config(
        &state.side_one,
        &format,
        "singles_ou"
    );
    assert!(result.is_err());
    let error_msg = result.unwrap_err();
    assert!(error_msg.contains("Pokemon"));
    assert!(error_msg.contains("banned"));
}

#[test]
fn test_config_based_validator_banned_moves() {
    let mut state = State::default();
    let format = BattleFormat::Singles;
    
    // Set up a Pokemon with banned move
    state.side_one.pokemon.p0.id = PokemonName::BLASTOISE;
    state.side_one.pokemon.p0.moves.m0.id = Choices::BATONPASS;
    
    let result = ConfigBasedValidator::validate_with_config(
        &state.side_one,
        &format,
        "singles_ou"
    );
    assert!(result.is_err());
    let error_msg = result.unwrap_err();
    assert!(error_msg.contains("Move"));
    assert!(error_msg.contains("banned"));
}

#[test]
fn test_config_based_validator_restricted_pokemon() {
    let mut state = State::default();
    let format = BattleFormat::VGC;
    
    // Set up too many restricted Pokemon (3 when limit is 2)
    state.side_one.pokemon.p0.id = PokemonName::MEWTWO;
    state.side_one.pokemon.p1.id = PokemonName::KYOGRE;
    state.side_one.pokemon.p2.id = PokemonName::RAYQUAZA;
    
    let result = ConfigBasedValidator::validate_with_config(
        &state.side_one,
        &format,
        "vgc_2024_reg_g"
    );
    assert!(result.is_err());
    let error_msg = result.unwrap_err();
    assert!(error_msg.contains("restricted Pokemon"));
    assert!(error_msg.contains("3"));
    assert!(error_msg.contains("maximum 2"));
}

#[test]
fn test_config_based_validator_valid_vgc_team() {
    let mut state = State::default();
    let format = BattleFormat::VGC;
    
    // Set up valid VGC team with 2 restricted Pokemon
    state.side_one.pokemon.p0.id = PokemonName::MEWTWO;
    state.side_one.pokemon.p0.item = Items::LIFEORB;
    state.side_one.pokemon.p1.id = PokemonName::KYOGRE;
    state.side_one.pokemon.p1.item = Items::LEFTOVERS;
    state.side_one.pokemon.p2.id = PokemonName::PIKACHU;
    state.side_one.pokemon.p2.item = Items::LIGHTBALL;
    state.side_one.pokemon.p3.id = PokemonName::CHARIZARD;
    state.side_one.pokemon.p3.item = Items::CHARCOAL;
    
    let result = ConfigBasedValidator::validate_with_config(
        &state.side_one,
        &format,
        "vgc_2024_reg_g"
    );
    assert!(result.is_ok());
}

#[test]
fn test_config_based_validator_mythical_ban() {
    let mut state = State::default();
    let format = BattleFormat::VGC;
    
    // Set up a mythical Pokemon (banned in VGC)
    state.side_one.pokemon.p0.id = PokemonName::MEW;
    
    let result = ConfigBasedValidator::validate_with_config(
        &state.side_one,
        &format,
        "vgc_2024_reg_g"
    );
    assert!(result.is_err());
    let error_msg = result.unwrap_err();
    assert!(error_msg.contains("MEW"));
    assert!(error_msg.contains("banned"));
}