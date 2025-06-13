use poke_engine::format_enforcement::*;
use poke_engine::battle_format::{BattleFormat, BattleRules, FormatClause};
use poke_engine::state::{State, SideReference, PokemonStatus, PokemonIndex};
use poke_engine::instruction::{Instruction, ChangeStatusInstruction};
use poke_engine::choices::Choices;

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