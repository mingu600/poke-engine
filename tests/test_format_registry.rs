use poke_engine::format_registry::*;
use poke_engine::battle_format::{BattleFormat, FormatClause};
use poke_engine::format_config::BattleFormatFactory;

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