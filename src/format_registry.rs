use crate::battle_format::{BattleRules, FormatClause};
use std::collections::HashMap;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

/// A complete format definition focusing on engine-impacting rules
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FormatDefinition {
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub party_size: usize,
    pub team_size: usize,
    pub active_pokemon: usize,
    pub clauses: Vec<FormatClause>,
}

impl FormatDefinition {
    pub fn to_battle_rules(&self) -> BattleRules {
        BattleRules {
            party_size: self.party_size,
            team_size: self.team_size,
            active_pokemon: self.active_pokemon,
            format_clauses: self.clauses.clone(),
        }
    }
}

lazy_static! {
    /// Registry of all available battle formats
    pub static ref FORMAT_REGISTRY: HashMap<String, FormatDefinition> = {
        let mut registry = HashMap::new();
        
        // Singles OU - Standard competitive singles
        registry.insert("singles_ou".to_string(), FormatDefinition {
            name: "singles_ou".to_string(),
            display_name: "Singles OU".to_string(),
            description: "Standard competitive singles with common clauses".to_string(),
            party_size: 6,
            team_size: 6,
            active_pokemon: 1,
            clauses: vec![
                FormatClause::SpeciesClause,
                FormatClause::SleepClause,
                FormatClause::OHKOClause,
                FormatClause::EvasionClause,
            ],
        });
        
        // Singles Ubers - Legendary singles
        registry.insert("singles_ubers".to_string(), FormatDefinition {
            name: "singles_ubers".to_string(),
            display_name: "Singles Ubers".to_string(),
            description: "Singles format allowing legendary Pokemon".to_string(),
            party_size: 6,
            team_size: 6,
            active_pokemon: 1,
            clauses: vec![
                FormatClause::SpeciesClause,
                FormatClause::SleepClause,
                FormatClause::OHKOClause,
                FormatClause::EvasionClause,
            ],
        });
        
        // Doubles OU - Standard competitive doubles
        registry.insert("doubles_ou".to_string(), FormatDefinition {
            name: "doubles_ou".to_string(),
            display_name: "Doubles OU".to_string(),
            description: "Standard competitive doubles format".to_string(),
            party_size: 6,
            team_size: 6,
            active_pokemon: 2,
            clauses: vec![
                FormatClause::SpeciesClause,
                FormatClause::SleepClause,
                FormatClause::OHKOClause,
                FormatClause::EvasionClause,
            ],
        });
        
        // VGC 2024 Regulation G
        registry.insert("vgc_2024_reg_g".to_string(), FormatDefinition {
            name: "vgc_2024_reg_g".to_string(),
            display_name: "VGC 2024 Regulation G".to_string(),
            description: "Official VGC 2024 Regulation G format".to_string(),
            party_size: 6,
            team_size: 4,
            active_pokemon: 2,
            clauses: vec![
                FormatClause::SpeciesClause,
                FormatClause::ItemClause,
                FormatClause::OHKOClause,
            ],
        });
        
        // Little Cup - Level 5 Pokemon only
        registry.insert("little_cup".to_string(), FormatDefinition {
            name: "little_cup".to_string(),
            display_name: "Little Cup".to_string(),
            description: "Singles format for unevolved Pokemon".to_string(),
            party_size: 6,
            team_size: 6,
            active_pokemon: 1,
            clauses: vec![
                FormatClause::SpeciesClause,
                FormatClause::SleepClause,
                FormatClause::OHKOClause,
                FormatClause::EvasionClause,
            ],
        });
        
        // No Clause Singles - Unrestricted singles
        registry.insert("singles_no_clause".to_string(), FormatDefinition {
            name: "singles_no_clause".to_string(),
            display_name: "Singles (No Clauses)".to_string(),
            description: "Singles format with minimal restrictions".to_string(),
            party_size: 6,
            team_size: 6,
            active_pokemon: 1,
            clauses: vec![
                FormatClause::SpeciesClause,  // Only species clause
            ],
        });
        
        // Triple Battles (theoretical)
        registry.insert("triples".to_string(), FormatDefinition {
            name: "triples".to_string(),
            display_name: "Triple Battles".to_string(),
            description: "Triple battles with 3 active Pokemon per side".to_string(),
            party_size: 6,
            team_size: 6,
            active_pokemon: 3,
            clauses: vec![
                FormatClause::SpeciesClause,
                FormatClause::SleepClause,
                FormatClause::OHKOClause,
            ],
        });
        
        // Draft League
        registry.insert("draft_league".to_string(), FormatDefinition {
            name: "draft_league".to_string(),
            display_name: "Draft League".to_string(),
            description: "Draft format with species clause disabled".to_string(),
            party_size: 6,
            team_size: 6,
            active_pokemon: 1,
            clauses: vec![
                FormatClause::SleepClause,
                FormatClause::OHKOClause,
                FormatClause::EvasionClause,
            ],
        });
        
        registry
    };
}

/// Format registry management
pub struct FormatRegistry;

impl FormatRegistry {
    /// Get a format definition by name
    pub fn get_format(name: &str) -> Option<&'static FormatDefinition> {
        FORMAT_REGISTRY.get(name)
    }
    
    /// Get battle rules for a format
    pub fn get_battle_rules(name: &str) -> Option<BattleRules> {
        Self::get_format(name).map(|def| def.to_battle_rules())
    }
    
    /// List all available format names
    pub fn list_formats() -> Vec<String> {
        FORMAT_REGISTRY.keys().cloned().collect()
    }
    
    /// Get formats by category
    pub fn get_singles_formats() -> Vec<&'static FormatDefinition> {
        FORMAT_REGISTRY.values()
            .filter(|def| def.active_pokemon == 1)
            .collect()
    }
    
    pub fn get_doubles_formats() -> Vec<&'static FormatDefinition> {
        FORMAT_REGISTRY.values()
            .filter(|def| def.active_pokemon == 2)
            .collect()
    }
    
    /// Check if a format exists
    pub fn format_exists(name: &str) -> bool {
        FORMAT_REGISTRY.contains_key(name)
    }
    
    /// Get format display information
    pub fn get_format_info(name: &str) -> Option<(String, String)> {
        Self::get_format(name).map(|def| (def.display_name.clone(), def.description.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_format_registry_basic() {
        // Test getting a format
        let singles_ou = FormatRegistry::get_format("singles_ou");
        assert!(singles_ou.is_some());
        
        let format = singles_ou.unwrap();
        assert_eq!(format.name, "singles_ou");
        assert_eq!(format.active_pokemon, 1);
        assert!(format.clauses.contains(&FormatClause::SleepClause));
        assert!(format.clauses.contains(&FormatClause::OHKOClause));
    }
    
    #[test]
    fn test_format_registry_rules_conversion() {
        let rules = FormatRegistry::get_battle_rules("vgc_2024_reg_g");
        assert!(rules.is_some());
        
        let rules = rules.unwrap();
        assert_eq!(rules.party_size, 6);
        assert_eq!(rules.team_size, 4);
        assert_eq!(rules.active_pokemon, 2);
        assert!(rules.format_clauses.contains(&FormatClause::ItemClause));
        assert!(!rules.format_clauses.contains(&FormatClause::SleepClause));
    }
    
    #[test]
    fn test_format_registry_categories() {
        let singles = FormatRegistry::get_singles_formats();
        let doubles = FormatRegistry::get_doubles_formats();
        
        assert!(!singles.is_empty());
        assert!(!doubles.is_empty());
        
        // All singles should have 1 active Pokemon
        assert!(singles.iter().all(|f| f.active_pokemon == 1));
        
        // All doubles should have 2 active Pokemon  
        assert!(doubles.iter().all(|f| f.active_pokemon == 2));
    }
    
    #[test]
    fn test_no_clause_format() {
        let no_clause = FormatRegistry::get_format("singles_no_clause");
        assert!(no_clause.is_some());
        
        let format = no_clause.unwrap();
        assert_eq!(format.clauses.len(), 1);
        assert!(format.clauses.contains(&FormatClause::SpeciesClause));
        assert!(!format.clauses.contains(&FormatClause::SleepClause));
    }
    
    #[test]
    fn test_format_listing() {
        let formats = FormatRegistry::list_formats();
        assert!(formats.contains(&"singles_ou".to_string()));
        assert!(formats.contains(&"vgc_2024_reg_g".to_string()));
        assert!(formats.contains(&"doubles_ou".to_string()));
        assert!(formats.len() >= 7); // We defined at least 7 formats
    }
    
    #[test]
    fn test_format_info() {
        let info = FormatRegistry::get_format_info("little_cup");
        assert!(info.is_some());
        
        let (display_name, description) = info.unwrap();
        assert_eq!(display_name, "Little Cup");
        assert!(description.contains("unevolved"));
    }
}