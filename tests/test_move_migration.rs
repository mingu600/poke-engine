use poke_engine::data::move_migration::MoveMigrationTool;
use poke_engine::choices::{Choices, MOVES};

#[test]
fn test_migration_completeness() {
    let special_moves = MoveMigrationTool::extract_special_moves();
    
    // Print summary for verification
    let summary = MoveMigrationTool::generate_summary_report();
    println!("{}", summary);
    
    // Verify known special moves are correctly extracted
    assert!(special_moves.contains_key(&Choices::ABSORB), "ABSORB should be extracted (drain move)");
    assert!(special_moves.contains_key(&Choices::BRAVEBIRD), "BRAVEBIRD should be extracted (recoil move)");
    assert!(special_moves.contains_key(&Choices::AGILITY), "AGILITY should be extracted (stat boost move)");
    assert!(special_moves.contains_key(&Choices::THUNDERBOLT), "THUNDERBOLT should be extracted (secondary effect)");
    
    // Verify basic attacking moves without special properties are NOT extracted
    assert!(!special_moves.contains_key(&Choices::TACKLE), "TACKLE should not be extracted (basic move)");
    
    // Verify coverage - should extract a significant portion of moves
    let total_moves = MOVES.len();
    let special_count = special_moves.len();
    let coverage_percentage = (special_count as f32 / total_moves as f32) * 100.0;
    
    println!("Migration coverage: {:.1}% ({}/{})", coverage_percentage, special_count, total_moves);
    
    // At least 30% of moves should have some special property
    assert!(coverage_percentage >= 30.0, 
        "Expected at least 30% of moves to have special properties, got {:.1}%", 
        coverage_percentage);
}

#[test]
fn test_engine_data_extraction() {
    let special_moves = MoveMigrationTool::extract_special_moves();
    
    // Test drain moves
    if let Some(absorb_data) = special_moves.get(&Choices::ABSORB) {
        assert_eq!(absorb_data.drain, Some(0.5), "ABSORB should have 50% drain");
        assert!(absorb_data.flags.heal, "ABSORB should have heal flag");
        assert!(absorb_data.flags.protect, "ABSORB should have protect flag");
    } else {
        panic!("ABSORB should be in extracted moves");
    }
    
    // Test recoil moves
    if let Some(bravebird_data) = special_moves.get(&Choices::BRAVEBIRD) {
        assert!(bravebird_data.recoil.is_some(), "BRAVEBIRD should have recoil damage");
        assert!(bravebird_data.flags.contact, "BRAVEBIRD should have contact flag");
    } else {
        panic!("BRAVEBIRD should be in extracted moves");
    }
    
    // Test stat boost moves
    if let Some(agility_data) = special_moves.get(&Choices::AGILITY) {
        assert!(agility_data.boost.is_some(), "AGILITY should have stat boost");
    } else {
        panic!("AGILITY should be in extracted moves");
    }
}

#[test] 
fn test_move_categories() {
    let special_moves = MoveMigrationTool::extract_special_moves();
    
    let mut drain_count = 0;
    let mut recoil_count = 0;
    let mut boost_count = 0;
    let mut status_count = 0;
    let mut secondary_count = 0;
    
    for (_, data) in special_moves.iter() {
        if data.drain.is_some() { drain_count += 1; }
        if data.recoil.is_some() { recoil_count += 1; }
        if data.boost.is_some() { boost_count += 1; }
        if data.status.is_some() || data.volatile_status.is_some() { status_count += 1; }
        if data.secondaries.is_some() { secondary_count += 1; }
    }
    
    println!("Move categories found:");
    println!("  Draining moves: {}", drain_count);
    println!("  Recoil moves: {}", recoil_count);
    println!("  Stat-boosting moves: {}", boost_count);
    println!("  Status moves: {}", status_count);
    println!("  Secondary effect moves: {}", secondary_count);
    
    // Verify we found moves in each major category
    assert!(drain_count > 5, "Should find multiple draining moves");
    assert!(recoil_count > 5, "Should find multiple recoil moves");
    assert!(boost_count > 20, "Should find many stat-boosting moves");
    assert!(status_count > 20, "Should find many status moves");
    assert!(secondary_count > 50, "Should find many moves with secondary effects");
}