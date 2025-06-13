use poke_engine::format_init::*;
use poke_engine::battle_format::BattleFormat;
use poke_engine::state::{State, SideReference};
use poke_engine::pokemon::PokemonName;

#[test]
fn test_format_detection_singles() {
    let state = State::default();
    
    // Default state should be detected as singles
    let format = BattleInitializer::detect_format(&state);
    assert_eq!(format, BattleFormat::Singles);
}

#[test]
fn test_format_detection_doubles() {
    let mut state = State::default();
    
    // Add a Pokemon to P1 slot to indicate doubles
    state.side_one.pokemon.p1.id = PokemonName::PIKACHU;
    state.side_one.pokemon.p1.hp = 100;
    
    let format = BattleInitializer::detect_format(&state);
    assert_eq!(format, BattleFormat::Doubles);
}

#[test]
fn test_format_detection_doubles_side_two() {
    let mut state = State::default();
    
    // Add a Pokemon to side two's P1 slot
    state.side_two.pokemon.p1.id = PokemonName::CHARIZARD;
    state.side_two.pokemon.p1.hp = 100;
    
    let format = BattleInitializer::detect_format(&state);
    assert_eq!(format, BattleFormat::Doubles);
}

#[test]
fn test_format_from_string() {
    // Test valid format strings
    assert_eq!(
        BattleInitializer::format_from_string("singles").unwrap(),
        BattleFormat::Singles
    );
    assert_eq!(
        BattleInitializer::format_from_string("SINGLES").unwrap(),
        BattleFormat::Singles
    );
    assert_eq!(
        BattleInitializer::format_from_string("doubles").unwrap(),
        BattleFormat::Doubles
    );
    assert_eq!(
        BattleInitializer::format_from_string("DOUBLES").unwrap(),
        BattleFormat::Doubles
    );
    assert_eq!(
        BattleInitializer::format_from_string("vgc").unwrap(),
        BattleFormat::VGC
    );
    assert_eq!(
        BattleInitializer::format_from_string("VGC").unwrap(),
        BattleFormat::VGC
    );
    
    // Test config-based format strings
    assert_eq!(
        BattleInitializer::format_from_string("singles_ou").unwrap(),
        BattleFormat::Singles
    );
    assert_eq!(
        BattleInitializer::format_from_string("doubles_ou").unwrap(),
        BattleFormat::Doubles
    );
    assert_eq!(
        BattleInitializer::format_from_string("vgc_2024_reg_g").unwrap(),
        BattleFormat::VGC
    );
    
    // Test invalid format string
    assert!(BattleInitializer::format_from_string("invalid_format").is_err());
}

#[test]
fn test_battle_context_creation() {
    let mut state = State::default();
    let format = BattleFormat::Singles;
    
    // Set up valid team
    state.side_one.pokemon.p0.id = PokemonName::PIKACHU;
    state.side_two.pokemon.p0.id = PokemonName::CHARIZARD;
    
    let context = BattleInitializer::initialize_battle(&mut state, format.clone(), None);
    assert!(context.is_ok());
    
    let context = context.unwrap();
    assert_eq!(context.format, format);
    assert!(context.config_name.is_none());
}

#[test]
fn test_battle_context_with_config() {
    let mut state = State::default();
    let format = BattleFormat::Singles;
    
    // Set up valid team
    state.side_one.pokemon.p0.id = PokemonName::PIKACHU;
    state.side_two.pokemon.p0.id = PokemonName::CHARIZARD;
    
    let context = BattleInitializer::initialize_battle(
        &mut state, 
        format.clone(), 
        Some("singles_ou")
    );
    assert!(context.is_ok());
    
    let context = context.unwrap();
    assert_eq!(context.format, format);
    assert_eq!(context.config_name, Some("singles_ou".to_string()));
}

#[test]
fn test_battle_context_invalid_team() {
    let mut state = State::default();
    let format = BattleFormat::Singles;
    
    // Set up team with duplicate species
    state.side_one.pokemon.p0.id = PokemonName::PIKACHU;
    state.side_one.pokemon.p1.id = PokemonName::PIKACHU;
    state.side_two.pokemon.p0.id = PokemonName::CHARIZARD;
    
    let result = BattleInitializer::initialize_battle(&mut state, format, None);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Duplicate species"));
}

#[test]
fn test_battle_context_banned_pokemon_config() {
    let mut state = State::default();
    let format = BattleFormat::Singles;
    
    // Set up team with banned Pokemon
    state.side_one.pokemon.p0.id = PokemonName::MEWTWO; // Banned in singles OU
    state.side_two.pokemon.p0.id = PokemonName::CHARIZARD;
    
    let result = BattleInitializer::initialize_battle(
        &mut state, 
        format, 
        Some("singles_ou")
    );
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("banned"));
}

#[test]
fn test_battle_context_user_positions() {
    let format = BattleFormat::Singles;
    let context = BattleContext {
        format: format.clone(),
        config_name: None,
        enforcer: poke_engine::format_enforcement::FormatEnforcer::new(format.clone()),
        tracker: poke_engine::format_enforcement::FormatStateTracker::new(format),
    };
    
    // Singles should only allow slot 0
    assert!(context.get_user_position(SideReference::SideOne, 0).is_some());
    assert!(context.get_user_position(SideReference::SideOne, 1).is_none());
    
    let pos = context.get_user_position(SideReference::SideOne, 0).unwrap();
    assert_eq!(pos.side, SideReference::SideOne);
    assert_eq!(pos.slot, 0);
}

#[test]
fn test_battle_context_user_positions_doubles() {
    let format = BattleFormat::Doubles;
    let context = BattleContext {
        format: format.clone(),
        config_name: None,
        enforcer: poke_engine::format_enforcement::FormatEnforcer::new(format.clone()),
        tracker: poke_engine::format_enforcement::FormatStateTracker::new(format),
    };
    
    // Doubles should allow slots 0 and 1
    assert!(context.get_user_position(SideReference::SideOne, 0).is_some());
    assert!(context.get_user_position(SideReference::SideOne, 1).is_some());
    assert!(context.get_user_position(SideReference::SideOne, 2).is_none());
    
    let pos0 = context.get_user_position(SideReference::SideTwo, 0).unwrap();
    assert_eq!(pos0.side, SideReference::SideTwo);
    assert_eq!(pos0.slot, 0);
    
    let pos1 = context.get_user_position(SideReference::SideTwo, 1).unwrap();
    assert_eq!(pos1.side, SideReference::SideTwo);
    assert_eq!(pos1.slot, 1);
}

#[test]
fn test_battle_context_update() {
    let format = BattleFormat::Singles;
    let mut context = BattleContext {
        format: format.clone(),
        config_name: None,
        enforcer: poke_engine::format_enforcement::FormatEnforcer::new(format.clone()),
        tracker: poke_engine::format_enforcement::FormatStateTracker::new(format),
    };
    
    let mut state = State::default();
    state.side_one.pokemon.p0.status = poke_engine::state::PokemonStatus::SLEEP;
    state.side_one.pokemon.p0.hp = 100;
    
    // Update context with new state
    context.update(&state);
    
    // Should not allow more sleep on side one
    assert!(!context.tracker.can_inflict_status(
        poke_engine::state::PokemonStatus::SLEEP,
        SideReference::SideOne
    ));
}