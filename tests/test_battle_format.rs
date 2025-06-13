use poke_engine::battle_format::*;
use poke_engine::state::{State, SideReference};
use poke_engine::data::types::MoveTarget;
use poke_engine::pokemon::PokemonName;
use poke_engine::engine::items::Items;
use poke_engine::choices::Choices;

#[test]
fn test_battle_format_rules() {
    // Test Singles format rules
    let singles = BattleFormat::Singles;
    let singles_rules = singles.get_rules();
    assert_eq!(singles_rules.party_size, 6);
    assert_eq!(singles_rules.team_size, 6);
    assert_eq!(singles_rules.active_pokemon, 1);
    assert!(singles_rules.format_clauses.contains(&FormatClause::SpeciesClause));
    assert!(singles_rules.format_clauses.contains(&FormatClause::SleepClause));
    
    // Test Doubles format rules
    let doubles = BattleFormat::Doubles;
    let doubles_rules = doubles.get_rules();
    assert_eq!(doubles_rules.party_size, 6);
    assert_eq!(doubles_rules.team_size, 6);
    assert_eq!(doubles_rules.active_pokemon, 2);
    
    // Test VGC format rules
    let vgc = BattleFormat::VGC;
    let vgc_rules = vgc.get_rules();
    assert_eq!(vgc_rules.party_size, 6);
    assert_eq!(vgc_rules.team_size, 4);
    assert_eq!(vgc_rules.active_pokemon, 2);
    assert!(vgc_rules.format_clauses.contains(&FormatClause::ItemClause));
}

#[test]
fn test_custom_format() {
    let custom_rules = BattleRules {
        party_size: 3,
        team_size: 3,
        active_pokemon: 1,
        format_clauses: vec![FormatClause::OHKOClause],
    };
    
    let custom_format = BattleFormat::Custom(custom_rules.clone());
    let retrieved_rules = custom_format.get_rules();
    assert_eq!(retrieved_rules.party_size, 3);
    assert_eq!(retrieved_rules.team_size, 3);
    assert_eq!(retrieved_rules.active_pokemon, 1);
    assert!(retrieved_rules.format_clauses.contains(&FormatClause::OHKOClause));
}

#[test]
fn test_battle_position() {
    let pos1 = BattlePosition::new(SideReference::SideOne, 0);
    assert_eq!(pos1.side, SideReference::SideOne);
    assert_eq!(pos1.slot, 0);
    
    let pos2 = BattlePosition::new(SideReference::SideTwo, 1);
    assert_eq!(pos2.side, SideReference::SideTwo);
    assert_eq!(pos2.slot, 1);
}

#[test]
fn test_target_resolver_singles() {
    let format = BattleFormat::Singles;
    let resolver = FormatTargetResolver::new(format);
    let state = State::default();
    let user_pos = BattlePosition::new(SideReference::SideOne, 0);
    
    // Test User target
    let targets = resolver.resolve_targets(user_pos, MoveTarget::User, &state);
    assert_eq!(targets.len(), 1);
    assert_eq!(targets[0].side, SideReference::SideOne);
    assert_eq!(targets[0].slot, 0);
    
    // Test SelectedPokemon target (should target opponent in singles)
    let targets = resolver.resolve_targets(user_pos, MoveTarget::SelectedPokemon, &state);
    assert_eq!(targets.len(), 1);
    assert_eq!(targets[0].side, SideReference::SideTwo);
    assert_eq!(targets[0].slot, 0);
    
    // Test AllOpponents target
    let targets = resolver.resolve_targets(user_pos, MoveTarget::AllOpponents, &state);
    assert_eq!(targets.len(), 1);
    assert_eq!(targets[0].side, SideReference::SideTwo);
    
    // Test EntireField target
    let targets = resolver.resolve_targets(user_pos, MoveTarget::EntireField, &state);
    assert_eq!(targets.len(), 1);
}

#[test]
fn test_target_resolver_doubles() {
    let format = BattleFormat::Doubles;
    let resolver = FormatTargetResolver::new(format);
    let state = State::default();
    let user_pos = BattlePosition::new(SideReference::SideOne, 0);
    
    // Test AllOpponents target (should target both opponents in doubles)
    let targets = resolver.resolve_targets(user_pos, MoveTarget::AllOpponents, &state);
    assert_eq!(targets.len(), 2);
    assert!(targets.iter().all(|t| t.side == SideReference::SideTwo));
    
    // Test Ally target
    let targets = resolver.resolve_targets(user_pos, MoveTarget::Ally, &state);
    assert_eq!(targets.len(), 1);
    assert_eq!(targets[0].side, SideReference::SideOne);
    assert_eq!(targets[0].slot, 1);
    
    // Test AllOtherPokemon target
    let targets = resolver.resolve_targets(user_pos, MoveTarget::AllOtherPokemon, &state);
    assert_eq!(targets.len(), 3); // Ally + 2 opponents
    
    // Test UserAndAllies target
    let targets = resolver.resolve_targets(user_pos, MoveTarget::UserAndAllies, &state);
    assert_eq!(targets.len(), 2);
    assert!(targets.iter().all(|t| t.side == SideReference::SideOne));
}

#[test]
fn test_format_validator_species_clause() {
    let mut state = State::default();
    let format = BattleFormat::Singles;
    
    // Set up duplicate species
    state.side_one.pokemon.p0.id = PokemonName::PIKACHU;
    state.side_one.pokemon.p1.id = PokemonName::PIKACHU;
    
    let result = FormatValidator::validate_team(&state.side_one, &format);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Duplicate species"));
}

#[test]
fn test_format_validator_item_clause() {
    let mut state = State::default();
    let format = BattleFormat::VGC;
    
    // Set up duplicate items
    state.side_one.pokemon.p0.id = PokemonName::PIKACHU;
    state.side_one.pokemon.p0.item = Items::LEFTOVERS;
    state.side_one.pokemon.p1.id = PokemonName::CHARIZARD;
    state.side_one.pokemon.p1.item = Items::LEFTOVERS;
    
    let result = FormatValidator::validate_team(&state.side_one, &format);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Duplicate item"));
}

#[test]
fn test_format_validator_ohko_clause() {
    let mut state = State::default();
    let format = BattleFormat::Custom(BattleRules {
        party_size: 6,
        team_size: 6,
        active_pokemon: 1,
        format_clauses: vec![FormatClause::OHKOClause],
    });
    
    // Set up OHKO move
    state.side_one.pokemon.p0.id = PokemonName::LAPRAS;
    state.side_one.pokemon.p0.moves.m0.id = Choices::SHEERCOLD;
    
    let result = FormatValidator::validate_team(&state.side_one, &format);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("OHKO move"));
}

#[test]
fn test_format_validator_evasion_clause() {
    let mut state = State::default();
    let format = BattleFormat::Custom(BattleRules {
        party_size: 6,
        team_size: 6,
        active_pokemon: 1,
        format_clauses: vec![FormatClause::EvasionClause],
    });
    
    // Set up evasion move
    state.side_one.pokemon.p0.id = PokemonName::PIKACHU;
    state.side_one.pokemon.p0.moves.m0.id = Choices::DOUBLETEAM;
    
    let result = FormatValidator::validate_team(&state.side_one, &format);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Evasion move"));
}

#[test]
fn test_format_validator_valid_team() {
    let mut state = State::default();
    let format = BattleFormat::Singles;
    
    // Set up valid team
    state.side_one.pokemon.p0.id = PokemonName::PIKACHU;
    state.side_one.pokemon.p0.item = Items::LIGHTBALL;
    state.side_one.pokemon.p0.moves.m0.id = Choices::THUNDERBOLT;
    
    state.side_one.pokemon.p1.id = PokemonName::CHARIZARD;
    state.side_one.pokemon.p1.item = Items::CHARCOAL;
    state.side_one.pokemon.p1.moves.m0.id = Choices::FLAMETHROWER;
    
    let result = FormatValidator::validate_team(&state.side_one, &format);
    assert!(result.is_ok());
}

#[test]
fn test_is_valid_target() {
    let singles_format = BattleFormat::Singles;
    let doubles_format = BattleFormat::Doubles;
    
    let singles_resolver = FormatTargetResolver::new(singles_format);
    let doubles_resolver = FormatTargetResolver::new(doubles_format);
    
    let state = State::default();
    
    // Singles: slot 0 is valid, slot 1 is not
    assert!(singles_resolver.is_valid_target(
        BattlePosition::new(SideReference::SideOne, 0),
        &state
    ));
    assert!(!singles_resolver.is_valid_target(
        BattlePosition::new(SideReference::SideOne, 1),
        &state
    ));
    
    // Doubles: slots 0 and 1 are valid, slot 2 is not
    assert!(doubles_resolver.is_valid_target(
        BattlePosition::new(SideReference::SideOne, 0),
        &state
    ));
    assert!(doubles_resolver.is_valid_target(
        BattlePosition::new(SideReference::SideOne, 1),
        &state
    ));
    assert!(!doubles_resolver.is_valid_target(
        BattlePosition::new(SideReference::SideOne, 2),
        &state
    ));
}