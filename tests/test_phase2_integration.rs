/// Phase 2 Integration Tests
/// 
/// This file consolidates the essential tests for all Phase 2 Multi-Format Battle System components:
/// - Battle Format System (battle_format.rs)
/// - Format Registry (format_registry.rs) 
/// - Format Enforcement (format_enforcement.rs)
/// - Format Initialization (format_init.rs)
/// 
/// This single test file can be run independently to validate Phase 2 functionality
/// without triggering errors from incomplete components.

use poke_engine::battle_format::*;
use poke_engine::format_registry::*;
use poke_engine::format_enforcement::*;
use poke_engine::format_init::*;
use poke_engine::state::{State, SideReference, PokemonStatus, PokemonIndex};
use poke_engine::instruction::{Instruction, ChangeStatusInstruction};
use poke_engine::data::types::MoveTarget;
use poke_engine::pokemon::PokemonName;
use poke_engine::engine::items::Items;
use poke_engine::choices::Choices;
use poke_engine::format_config::BattleFormatFactory;

// ============================================================================
// BATTLE FORMAT TESTS
// ============================================================================

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

// ============================================================================
// FORMAT REGISTRY TESTS
// ============================================================================

#[test]
fn test_format_registry_basic_access() {
    // Test getting format definitions
    let singles_ou = FormatRegistry::get_format("singles_ou");
    assert!(singles_ou.is_some());
    
    let format = singles_ou.unwrap();
    assert_eq!(format.name, "singles_ou");
    assert_eq!(format.display_name, "Singles OU");
    assert_eq!(format.active_pokemon, 1);
    assert!(format.clauses.contains(&FormatClause::SleepClause));
    assert!(format.clauses.contains(&FormatClause::OHKOClause));
    assert!(format.clauses.contains(&FormatClause::EvasionClause));
}

#[test]
fn test_format_registry_vgc() {
    let vgc = FormatRegistry::get_format("vgc_2024_reg_g");
    assert!(vgc.is_some());
    
    let format = vgc.unwrap();
    assert_eq!(format.team_size, 4);  // VGC brings 4 Pokemon
    assert_eq!(format.active_pokemon, 2);  // 2 active at once
    assert!(format.clauses.contains(&FormatClause::ItemClause));
    assert!(!format.clauses.contains(&FormatClause::SleepClause)); // VGC doesn't have sleep clause
}

#[test]
fn test_format_registry_battle_rules_conversion() {
    let rules = FormatRegistry::get_battle_rules("doubles_ou");
    assert!(rules.is_some());
    
    let rules = rules.unwrap();
    assert_eq!(rules.active_pokemon, 2);
    assert!(rules.format_clauses.contains(&FormatClause::SpeciesClause));
    assert!(rules.format_clauses.contains(&FormatClause::OHKOClause));
}

#[test]
fn test_format_registry_categories() {
    let singles = FormatRegistry::get_singles_formats();
    let doubles = FormatRegistry::get_doubles_formats();
    
    assert!(!singles.is_empty());
    assert!(!doubles.is_empty());
    
    // Check that categorization works
    for format in singles {
        assert_eq!(format.active_pokemon, 1);
    }
    
    for format in doubles {
        assert_eq!(format.active_pokemon, 2);
    }
}

#[test]
fn test_battle_format_integration() {
    // Test that BattleFormat::Singles now uses registry
    let singles = BattleFormat::Singles;
    let rules = singles.get_rules();
    
    // Should now have OHKO and Evasion clauses from registry
    assert!(rules.format_clauses.contains(&FormatClause::OHKOClause));
    assert!(rules.format_clauses.contains(&FormatClause::EvasionClause));
    assert!(rules.format_clauses.contains(&FormatClause::SleepClause));
    assert!(rules.format_clauses.contains(&FormatClause::SpeciesClause));
}

#[test]
fn test_format_registry_special_formats() {
    // Test no clause format
    let no_clause = FormatRegistry::get_format("singles_no_clause");
    assert!(no_clause.is_some());
    
    let format = no_clause.unwrap();
    assert_eq!(format.clauses.len(), 1);  // Only species clause
    assert!(format.clauses.contains(&FormatClause::SpeciesClause));
    assert!(!format.clauses.contains(&FormatClause::SleepClause));
    
    // Test little cup
    let little_cup = FormatRegistry::get_format("little_cup");
    assert!(little_cup.is_some());
    assert!(little_cup.unwrap().description.contains("unevolved"));
    
    // Test draft league (no species clause)
    let draft = FormatRegistry::get_format("draft_league");
    assert!(draft.is_some());
    
    let draft_format = draft.unwrap();
    assert!(!draft_format.clauses.contains(&FormatClause::SpeciesClause));
    assert!(draft_format.clauses.contains(&FormatClause::SleepClause));
}

#[test]
fn test_named_format_creation() {
    // Test creating named formats
    let little_cup = BattleFormat::Named("little_cup".to_string());
    let rules = little_cup.get_rules();
    
    assert_eq!(rules.active_pokemon, 1);
    assert!(rules.format_clauses.contains(&FormatClause::OHKOClause));
    
    // Test registry name getter
    assert_eq!(little_cup.get_registry_name(), Some("little_cup".to_string()));
}

#[test]
fn test_factory_with_registry() {
    // Test factory can create named formats
    let triples = BattleFormatFactory::named("triples");
    assert!(triples.is_some());
    
    let triples_format = triples.unwrap();
    let rules = triples_format.get_rules();
    assert_eq!(rules.active_pokemon, 3);
    
    // Test factory listing
    let available = BattleFormatFactory::list_available_formats();
    assert!(available.contains(&"singles_ou".to_string()));
    assert!(available.contains(&"triples".to_string()));
    assert!(available.contains(&"draft_league".to_string()));
}

#[test]
fn test_format_registry_unknown_format() {
    // Test unknown format handling
    let unknown = FormatRegistry::get_format("unknown_format");
    assert!(unknown.is_none());
    
    let unknown_rules = FormatRegistry::get_battle_rules("unknown_format");
    assert!(unknown_rules.is_none());
    
    assert!(!FormatRegistry::format_exists("unknown_format"));
}

#[test]
fn test_format_info() {
    let info = FormatRegistry::get_format_info("vgc_2024_reg_g");
    assert!(info.is_some());
    
    let (display_name, description) = info.unwrap();
    assert_eq!(display_name, "VGC 2024 Regulation G");
    assert!(description.contains("Official VGC"));
}

// ============================================================================
// FORMAT ENFORCEMENT TESTS
// ============================================================================

#[test]
fn test_format_enforcer_sleep_clause() {
    let format = BattleFormat::Singles;
    let enforcer = FormatEnforcer::new(format);
    let mut state = State::default();
    
    // Put first Pokemon to sleep - should be allowed
    state.side_one.pokemon.p0.status = PokemonStatus::SLEEP;
    state.side_one.pokemon.p0.hp = 100;
    
    // Try to put second Pokemon to sleep - should be blocked
    let instruction = Instruction::ChangeStatus(ChangeStatusInstruction {
        side_ref: SideReference::SideOne,
        pokemon_index: PokemonIndex::P1,
        old_status: PokemonStatus::NONE,
        new_status: PokemonStatus::SLEEP,
    });
    
    let result = enforcer.validate_instruction(&instruction, &state);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Sleep Clause"));
}

#[test]
fn test_format_enforcer_freeze_clause() {
    let format = BattleFormat::Custom(BattleRules {
        party_size: 6,
        team_size: 6,
        active_pokemon: 1,
        format_clauses: vec![FormatClause::FreezeClause],
    });
    let enforcer = FormatEnforcer::new(format);
    let mut state = State::default();
    
    // Freeze first Pokemon - should be allowed
    state.side_two.pokemon.p0.status = PokemonStatus::FREEZE;
    state.side_two.pokemon.p0.hp = 100;
    
    // Try to freeze second Pokemon - should be blocked
    let instruction = Instruction::ChangeStatus(ChangeStatusInstruction {
        side_ref: SideReference::SideTwo,
        pokemon_index: PokemonIndex::P1,
        old_status: PokemonStatus::NONE,
        new_status: PokemonStatus::FREEZE,
    });
    
    let result = enforcer.validate_instruction(&instruction, &state);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Freeze Clause"));
}

#[test]
fn test_format_enforcer_ohko_clause() {
    let format = BattleFormat::Custom(BattleRules {
        party_size: 6,
        team_size: 6,
        active_pokemon: 1,
        format_clauses: vec![FormatClause::OHKOClause],
    });
    let enforcer = FormatEnforcer::new(format);
    let state = State::default();
    
    // OHKO moves should be blocked
    for ohko_move in &[Choices::FISSURE, Choices::GUILLOTINE, Choices::HORNDRILL, Choices::SHEERCOLD] {
        let result = enforcer.validate_move_choice(ohko_move, SideReference::SideOne, &state);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("OHKO Clause"));
    }
    
    // Regular moves should be allowed
    let result = enforcer.validate_move_choice(&Choices::TACKLE, SideReference::SideOne, &state);
    assert!(result.is_ok());
}

#[test]
fn test_format_enforcer_evasion_clause() {
    let format = BattleFormat::Custom(BattleRules {
        party_size: 6,
        team_size: 6,
        active_pokemon: 1,
        format_clauses: vec![FormatClause::EvasionClause],
    });
    let enforcer = FormatEnforcer::new(format);
    let state = State::default();
    
    // Evasion moves should be blocked
    let result = enforcer.validate_move_choice(&Choices::DOUBLETEAM, SideReference::SideOne, &state);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Evasion Clause"));
    
    let result = enforcer.validate_move_choice(&Choices::MINIMIZE, SideReference::SideOne, &state);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Evasion Clause"));
}

#[test]
fn test_format_enforcer_allows_multiple_statuses_without_clause() {
    let format = BattleFormat::Custom(BattleRules {
        party_size: 6,
        team_size: 6,
        active_pokemon: 1,
        format_clauses: vec![], // No sleep or freeze clause
    });
    let enforcer = FormatEnforcer::new(format);
    let mut state = State::default();
    
    // Put multiple Pokemon to sleep - should be allowed
    state.side_one.pokemon.p0.status = PokemonStatus::SLEEP;
    state.side_one.pokemon.p0.hp = 100;
    
    let instruction = Instruction::ChangeStatus(ChangeStatusInstruction {
        side_ref: SideReference::SideOne,
        pokemon_index: PokemonIndex::P1,
        old_status: PokemonStatus::NONE,
        new_status: PokemonStatus::SLEEP,
    });
    
    let result = enforcer.validate_instruction(&instruction, &state);
    assert!(result.is_ok());
}

#[test]
fn test_format_enforcer_active_pokemon_limit() {
    let singles = FormatEnforcer::new(BattleFormat::Singles);
    assert_eq!(singles.get_active_pokemon_limit(), 1);
    
    let doubles = FormatEnforcer::new(BattleFormat::Doubles);
    assert_eq!(doubles.get_active_pokemon_limit(), 2);
    
    let vgc = FormatEnforcer::new(BattleFormat::VGC);
    assert_eq!(vgc.get_active_pokemon_limit(), 2);
}

#[test]
fn test_format_state_tracker_sleep_count() {
    let format = BattleFormat::Singles;
    let mut tracker = FormatStateTracker::new(format);
    let mut state = State::default();
    
    // Initially no sleeping Pokemon
    tracker.update_from_state(&state);
    assert!(tracker.can_inflict_status(PokemonStatus::SLEEP, SideReference::SideOne));
    assert!(tracker.can_inflict_status(PokemonStatus::SLEEP, SideReference::SideTwo));
    
    // Add one sleeping Pokemon on side one
    state.side_one.pokemon.p0.status = PokemonStatus::SLEEP;
    state.side_one.pokemon.p0.hp = 100;
    tracker.update_from_state(&state);
    
    // Should not allow more sleep on side one, but side two is OK
    assert!(!tracker.can_inflict_status(PokemonStatus::SLEEP, SideReference::SideOne));
    assert!(tracker.can_inflict_status(PokemonStatus::SLEEP, SideReference::SideTwo));
}

#[test]
fn test_format_state_tracker_freeze_count() {
    let format = BattleFormat::Custom(BattleRules {
        party_size: 6,
        team_size: 6,
        active_pokemon: 1,
        format_clauses: vec![FormatClause::FreezeClause],
    });
    let mut tracker = FormatStateTracker::new(format);
    let mut state = State::default();
    
    // Initially no frozen Pokemon
    tracker.update_from_state(&state);
    assert!(tracker.can_inflict_status(PokemonStatus::FREEZE, SideReference::SideOne));
    
    // Add one frozen Pokemon
    state.side_one.pokemon.p0.status = PokemonStatus::FREEZE;
    state.side_one.pokemon.p0.hp = 100;
    tracker.update_from_state(&state);
    
    // Should not allow more freeze
    assert!(!tracker.can_inflict_status(PokemonStatus::FREEZE, SideReference::SideOne));
}

#[test]
fn test_format_state_tracker_fainted_pokemon_not_counted() {
    let format = BattleFormat::Singles;
    let mut tracker = FormatStateTracker::new(format);
    let mut state = State::default();
    
    // Add sleeping Pokemon that is fainted (0 HP)
    state.side_one.pokemon.p0.status = PokemonStatus::SLEEP;
    state.side_one.pokemon.p0.hp = 0;
    tracker.update_from_state(&state);
    
    // Should still allow sleep since fainted Pokemon don't count
    assert!(tracker.can_inflict_status(PokemonStatus::SLEEP, SideReference::SideOne));
}

#[test]
fn test_format_state_tracker_other_statuses_allowed() {
    let format = BattleFormat::Singles;
    let tracker = FormatStateTracker::new(format);
    
    // Other statuses should always be allowed
    assert!(tracker.can_inflict_status(PokemonStatus::BURN, SideReference::SideOne));
    assert!(tracker.can_inflict_status(PokemonStatus::PARALYZE, SideReference::SideOne));
    assert!(tracker.can_inflict_status(PokemonStatus::POISON, SideReference::SideOne));
    assert!(tracker.can_inflict_status(PokemonStatus::TOXIC, SideReference::SideOne));
}

// ============================================================================
// FORMAT INITIALIZATION TESTS
// ============================================================================

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
fn test_format_from_string() {
    // Test valid format strings
    assert_eq!(
        BattleInitializer::format_from_string("singles").unwrap(),
        BattleFormat::Singles
    );
    assert_eq!(
        BattleInitializer::format_from_string("doubles").unwrap(),
        BattleFormat::Doubles
    );
    assert_eq!(
        BattleInitializer::format_from_string("vgc").unwrap(),
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
fn test_format_detection_doubles_side_two() {
    let mut state = State::default();

    // Add a Pokemon to side two's P1 slot
    state.side_two.pokemon.p1.id = PokemonName::CHARIZARD;
    state.side_two.pokemon.p1.hp = 100;

    let format = BattleInitializer::detect_format(&state);
    assert_eq!(format, BattleFormat::Doubles);
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

    let context =
        BattleInitializer::initialize_battle(&mut state, format.clone(), Some("singles_ou"));
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

    let result = BattleInitializer::initialize_battle(&mut state, format, Some("singles_ou"));
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("banned"));
}

#[test]
fn test_battle_context_user_positions() {
    let format = BattleFormat::Singles;
    let context = BattleContext {
        format: format.clone(),
        config_name: None,
        enforcer: FormatEnforcer::new(format.clone()),
        tracker: FormatStateTracker::new(format),
    };

    // Singles should only allow slot 0
    assert!(context
        .get_user_position(SideReference::SideOne, 0)
        .is_some());
    assert!(context
        .get_user_position(SideReference::SideOne, 1)
        .is_none());

    let pos = context
        .get_user_position(SideReference::SideOne, 0)
        .unwrap();
    assert_eq!(pos.side, SideReference::SideOne);
    assert_eq!(pos.slot, 0);
}

#[test]
fn test_battle_context_user_positions_doubles() {
    let format = BattleFormat::Doubles;
    let context = BattleContext {
        format: format.clone(),
        config_name: None,
        enforcer: FormatEnforcer::new(format.clone()),
        tracker: FormatStateTracker::new(format),
    };

    // Doubles should allow slots 0 and 1
    assert!(context
        .get_user_position(SideReference::SideOne, 0)
        .is_some());
    assert!(context
        .get_user_position(SideReference::SideOne, 1)
        .is_some());
    assert!(context
        .get_user_position(SideReference::SideOne, 2)
        .is_none());

    let pos0 = context
        .get_user_position(SideReference::SideTwo, 0)
        .unwrap();
    assert_eq!(pos0.side, SideReference::SideTwo);
    assert_eq!(pos0.slot, 0);

    let pos1 = context
        .get_user_position(SideReference::SideTwo, 1)
        .unwrap();
    assert_eq!(pos1.side, SideReference::SideTwo);
    assert_eq!(pos1.slot, 1);
}

#[test]
fn test_battle_context_update() {
    let format = BattleFormat::Singles;
    let mut context = BattleContext {
        format: format.clone(),
        config_name: None,
        enforcer: FormatEnforcer::new(format.clone()),
        tracker: FormatStateTracker::new(format),
    };

    let mut state = State::default();
    state.side_one.pokemon.p0.status = PokemonStatus::SLEEP;
    state.side_one.pokemon.p0.hp = 100;

    // Update context with new state
    context.update(&state);

    // Should not allow more sleep on side one
    assert!(!context.tracker.can_inflict_status(
        PokemonStatus::SLEEP,
        SideReference::SideOne
    ));
}

// ============================================================================
// COMPREHENSIVE INTEGRATION TESTS
// ============================================================================

#[test]
fn test_phase2_complete_integration() {
    // Test complete workflow: Registry -> Format -> Enforcement -> Initialization
    
    // 1. Get format from registry
    let format_def = FormatRegistry::get_format("singles_ou").expect("singles_ou should exist");
    assert_eq!(format_def.active_pokemon, 1);
    assert!(format_def.clauses.contains(&FormatClause::SleepClause));
    
    // 2. Create battle format
    let format = BattleFormat::Singles;
    let rules = format.get_rules();
    assert_eq!(rules.active_pokemon, 1);
    assert!(rules.format_clauses.contains(&FormatClause::SleepClause));
    
    // 3. Set up valid battle state
    let mut state = State::default();
    state.side_one.pokemon.p0.id = PokemonName::PIKACHU;
    state.side_one.pokemon.p0.hp = 100;
    state.side_two.pokemon.p0.id = PokemonName::CHARIZARD;
    state.side_two.pokemon.p0.hp = 100;
    
    // 4. Initialize battle context
    let context = BattleInitializer::initialize_battle(&mut state, format, None);
    assert!(context.is_ok());
    
    let context = context.unwrap();
    assert_eq!(context.format, BattleFormat::Singles);
    
    // 5. Test enforcement
    assert_eq!(context.enforcer.get_active_pokemon_limit(), 1);
    
    // 6. Test targeting
    let user_pos = context.get_user_position(SideReference::SideOne, 0).unwrap();
    assert_eq!(user_pos.side, SideReference::SideOne);
    assert_eq!(user_pos.slot, 0);
}

#[test]
fn test_phase2_doubles_integration() {
    // Test doubles format end-to-end
    
    // 1. Get doubles format
    let format = BattleFormat::Doubles;
    let rules = format.get_rules();
    assert_eq!(rules.active_pokemon, 2);
    
    // 2. Set up doubles state
    let mut state = State::default();
    state.side_one.pokemon.p0.id = PokemonName::PIKACHU;
    state.side_one.pokemon.p0.hp = 100;
    state.side_one.pokemon.p1.id = PokemonName::CHARIZARD;
    state.side_one.pokemon.p1.hp = 100;
    state.side_two.pokemon.p0.id = PokemonName::BLASTOISE;
    state.side_two.pokemon.p0.hp = 100;
    state.side_two.pokemon.p1.id = PokemonName::VENUSAUR;
    state.side_two.pokemon.p1.hp = 100;
    
    // 3. Initialize context
    let context = BattleInitializer::initialize_battle(&mut state, format, None);
    assert!(context.is_ok());
    
    let context = context.unwrap();
    assert_eq!(context.enforcer.get_active_pokemon_limit(), 2);
    
    // 4. Test both positions are valid
    assert!(context.get_user_position(SideReference::SideOne, 0).is_some());
    assert!(context.get_user_position(SideReference::SideOne, 1).is_some());
    assert!(context.get_user_position(SideReference::SideOne, 2).is_none());
}

#[test] 
fn test_phase2_vgc_integration() {
    // Test VGC format integration
    
    // 1. Get VGC format from registry
    let vgc_def = FormatRegistry::get_format("vgc_2024_reg_g").expect("VGC format should exist");
    assert_eq!(vgc_def.team_size, 4);
    assert_eq!(vgc_def.active_pokemon, 2);
    assert!(vgc_def.clauses.contains(&FormatClause::ItemClause));
    
    // 2. Create VGC battle format
    let format = BattleFormat::VGC;
    let rules = format.get_rules();
    assert_eq!(rules.team_size, 4);
    assert_eq!(rules.active_pokemon, 2);
    assert!(rules.format_clauses.contains(&FormatClause::ItemClause));
    
    // 3. Set up VGC-compliant state (no duplicate items)
    let mut state = State::default();
    state.side_one.pokemon.p0.id = PokemonName::PIKACHU;
    state.side_one.pokemon.p0.hp = 100;
    state.side_one.pokemon.p0.item = Items::LIGHTBALL;
    state.side_one.pokemon.p1.id = PokemonName::CHARIZARD;
    state.side_one.pokemon.p1.hp = 100;
    state.side_one.pokemon.p1.item = Items::CHARCOAL;
    
    state.side_two.pokemon.p0.id = PokemonName::BLASTOISE;
    state.side_two.pokemon.p0.hp = 100;
    state.side_two.pokemon.p0.item = Items::MYSTICWATER;
    state.side_two.pokemon.p1.id = PokemonName::VENUSAUR;
    state.side_two.pokemon.p1.hp = 100;
    state.side_two.pokemon.p1.item = Items::MIRACLESEED;
    
    // 4. Initialize battle
    let context = BattleInitializer::initialize_battle(&mut state, format, None);
    assert!(context.is_ok());
    
    let context = context.unwrap();
    assert_eq!(context.format, BattleFormat::VGC);
    assert_eq!(context.enforcer.get_active_pokemon_limit(), 2);
}