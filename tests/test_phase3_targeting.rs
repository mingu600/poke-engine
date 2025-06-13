// Phase 3 Week 11-12: Move Targeting Implementation Tests

use poke_engine::battle_format::{BattleFormat, BattlePosition, FormatTargetResolver, TargetResolver};
use poke_engine::data::types::MoveTarget;
use poke_engine::state::{SideReference, State};

#[test]
fn test_user_targeting_consistent_across_formats() {
    // Test that MoveTarget::User always targets the user regardless of format
    let singles_state = create_test_state(BattleFormat::Singles);
    let doubles_state = create_test_state(BattleFormat::Doubles);
    
    let user_position = BattlePosition::new(SideReference::SideOne, 0);
    
    // Test in singles
    let singles_resolver = FormatTargetResolver::new(BattleFormat::Singles);
    let singles_targets = singles_resolver.resolve_targets(
        user_position,
        MoveTarget::User,
        &singles_state,
    );
    assert_eq!(singles_targets.len(), 1);
    assert_eq!(singles_targets[0], user_position);
    
    // Test in doubles
    let doubles_resolver = FormatTargetResolver::new(BattleFormat::Doubles);
    let doubles_targets = doubles_resolver.resolve_targets(
        user_position,
        MoveTarget::User,
        &doubles_state,
    );
    assert_eq!(doubles_targets.len(), 1);
    assert_eq!(doubles_targets[0], user_position);
}

#[test]
fn test_selected_pokemon_targeting_format_differences() {
    // Test that MoveTarget::SelectedPokemon behaves differently in singles vs doubles
    let singles_state = create_test_state(BattleFormat::Singles);
    let doubles_state = create_test_state(BattleFormat::Doubles);
    
    let user_position = BattlePosition::new(SideReference::SideOne, 0);
    
    // Test in singles - should target the single opponent
    let singles_resolver = FormatTargetResolver::new(BattleFormat::Singles);
    let singles_targets = singles_resolver.resolve_targets(
        user_position,
        MoveTarget::SelectedPokemon,
        &singles_state,
    );
    assert_eq!(singles_targets.len(), 1);
    assert_eq!(singles_targets[0].side, SideReference::SideTwo);
    assert_eq!(singles_targets[0].slot, 0);
    
    // Test in doubles - should target the first opponent (in actual implementation,
    // this would need player input to select which opponent)
    let doubles_resolver = FormatTargetResolver::new(BattleFormat::Doubles);
    let doubles_targets = doubles_resolver.resolve_targets(
        user_position,
        MoveTarget::SelectedPokemon,
        &doubles_state,
    );
    assert_eq!(doubles_targets.len(), 1);
    assert_eq!(doubles_targets[0].side, SideReference::SideTwo);
    assert_eq!(doubles_targets[0].slot, 0);
}

#[test]
fn test_all_opponents_targeting_format_differences() {
    // Test that MoveTarget::AllOpponents behaves differently in singles vs doubles
    let singles_state = create_test_state(BattleFormat::Singles);
    let doubles_state = create_test_state(BattleFormat::Doubles);
    
    let user_position = BattlePosition::new(SideReference::SideOne, 0);
    
    // Test in singles - should target 1 opponent
    let singles_resolver = FormatTargetResolver::new(BattleFormat::Singles);
    let singles_targets = singles_resolver.resolve_targets(
        user_position,
        MoveTarget::AllOpponents,
        &singles_state,
    );
    assert_eq!(singles_targets.len(), 1);
    assert_eq!(singles_targets[0].side, SideReference::SideTwo);
    assert_eq!(singles_targets[0].slot, 0);
    
    // Test in doubles - should target 2 opponents
    let doubles_resolver = FormatTargetResolver::new(BattleFormat::Doubles);
    let doubles_targets = doubles_resolver.resolve_targets(
        user_position,
        MoveTarget::AllOpponents,
        &doubles_state,
    );
    assert_eq!(doubles_targets.len(), 2);
    assert_eq!(doubles_targets[0].side, SideReference::SideTwo);
    assert_eq!(doubles_targets[0].slot, 0);
    assert_eq!(doubles_targets[1].side, SideReference::SideTwo);
    assert_eq!(doubles_targets[1].slot, 1);
}

#[test]
fn test_ally_targeting_format_differences() {
    // Test that MoveTarget::Ally behaves differently in singles vs doubles
    let singles_state = create_test_state(BattleFormat::Singles);
    let doubles_state = create_test_state(BattleFormat::Doubles);
    
    let user_position = BattlePosition::new(SideReference::SideOne, 0);
    
    // Test in singles - should target no one (no allies)
    let singles_resolver = FormatTargetResolver::new(BattleFormat::Singles);
    let singles_targets = singles_resolver.resolve_targets(
        user_position,
        MoveTarget::Ally,
        &singles_state,
    );
    assert_eq!(singles_targets.len(), 0);
    
    // Test in doubles - should target ally in slot 1
    let doubles_resolver = FormatTargetResolver::new(BattleFormat::Doubles);
    let doubles_targets = doubles_resolver.resolve_targets(
        user_position,
        MoveTarget::Ally,
        &doubles_state,
    );
    assert_eq!(doubles_targets.len(), 1);
    assert_eq!(doubles_targets[0].side, SideReference::SideOne);
    assert_eq!(doubles_targets[0].slot, 1);
}

#[test]
fn test_vgc_format_targeting() {
    // Test that VGC format (which uses doubles rules) works correctly
    let vgc_state = create_test_state(BattleFormat::VGC);
    let user_position = BattlePosition::new(SideReference::SideOne, 0);
    
    let vgc_resolver = FormatTargetResolver::new(BattleFormat::VGC);
    
    // VGC should behave like doubles for targeting
    let targets = vgc_resolver.resolve_targets(
        user_position,
        MoveTarget::AllOpponents,
        &vgc_state,
    );
    assert_eq!(targets.len(), 2); // Should target both opponent slots
}

#[test]
fn test_target_validation() {
    let doubles_state = create_test_state(BattleFormat::Doubles);
    let resolver = FormatTargetResolver::new(BattleFormat::Doubles);
    
    // Valid positions
    assert!(resolver.is_valid_target(
        BattlePosition::new(SideReference::SideOne, 0),
        &doubles_state
    ));
    assert!(resolver.is_valid_target(
        BattlePosition::new(SideReference::SideOne, 1),
        &doubles_state
    ));
    assert!(resolver.is_valid_target(
        BattlePosition::new(SideReference::SideTwo, 0),
        &doubles_state
    ));
    assert!(resolver.is_valid_target(
        BattlePosition::new(SideReference::SideTwo, 1),
        &doubles_state
    ));
    
    // Invalid position (slot 2 doesn't exist in doubles)
    assert!(!resolver.is_valid_target(
        BattlePosition::new(SideReference::SideOne, 2),
        &doubles_state
    ));
}

// Helper function to create a test state with the specified format
fn create_test_state(format: BattleFormat) -> State {
    let mut state = State::default();
    state.format = format;
    state
}