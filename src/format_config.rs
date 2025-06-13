use crate::battle_format::{BattleFormat, BattleRules};
use crate::pokemon::PokemonName;
use crate::engine::items::Items;
use std::collections::HashMap;
use lazy_static::lazy_static;

/// Configuration for external team validation (not engine mechanics)
/// This is for downstream applications to validate teams before using the engine
pub struct FormatConfig {
    pub banned_pokemon: Vec<PokemonName>,     // Pokemon banned from this format
    pub banned_items: Vec<Items>,             // Items banned from this format  
    pub restricted_pokemon: Vec<PokemonName>, // VGC restricted legendaries
    pub restricted_limit: usize,              // Max restricted Pokemon allowed
}

impl Default for FormatConfig {
    fn default() -> Self {
        FormatConfig {
            banned_pokemon: Vec::new(),
            banned_items: Vec::new(),
            restricted_pokemon: Vec::new(),
            restricted_limit: 0,
        }
    }
}

lazy_static! {
    /// Standard format configurations
    pub static ref FORMAT_CONFIGS: HashMap<String, FormatConfig> = {
        let mut configs = HashMap::new();
        
        // Singles OU configuration
        configs.insert("singles_ou".to_string(), FormatConfig {
            banned_pokemon: vec![
                // Uber Pokemon typically banned from OU
                PokemonName::MEWTWO,
                PokemonName::MEW,
                PokemonName::LUGIA,
                PokemonName::HOOH,
                PokemonName::RAYQUAZA,
                PokemonName::RAYQUAZAMEGA,
                PokemonName::DIALGA,
                PokemonName::DIALGAORIGIN,
                PokemonName::PALKIA,
                PokemonName::PALKIAORIGIN,
                PokemonName::GIRATINA,
                PokemonName::GIRATINAORIGIN,
                PokemonName::ARCEUS,
                PokemonName::RESHIRAM,
                PokemonName::ZEKROM,
                PokemonName::KYUREM,
                PokemonName::KYUREMWHITE,
                PokemonName::XERNEAS,
                PokemonName::YVELTAL,
                PokemonName::ZYGARDECOMPLETE,
                PokemonName::SOLGALEO,
                PokemonName::LUNALA,
                PokemonName::NECROZMADUSKMANE,
                PokemonName::NECROZMADAWNWINGS,
                PokemonName::ZACIAN,
                PokemonName::ZACIANCROWNED,
                PokemonName::ZAMAZENTA,
                PokemonName::ZAMAZENTACROWNED,
                PokemonName::ETERNATUS,
                PokemonName::CALYREXICE,
                PokemonName::CALYREXSHADOW,
            ],
            banned_items: vec![
                // Items typically banned in OU
                Items::SOULDEW,  // In older gens
            ],
            restricted_pokemon: Vec::new(),
            restricted_limit: 0,
        });
        
        // VGC 2024 Regulation G configuration
        configs.insert("vgc_2024_reg_g".to_string(), FormatConfig {
            banned_pokemon: vec![
                // Mythical Pokemon are typically banned
                PokemonName::MEW,
                PokemonName::CELEBI,
                PokemonName::JIRACHI,
                PokemonName::DEOXYS,
                PokemonName::DEOXYSATTACK,
                PokemonName::DEOXYSDEFENSE,
                PokemonName::DEOXYSSPEED,
                PokemonName::PHIONE,
                PokemonName::MANAPHY,
                PokemonName::DARKRAI,
                PokemonName::SHAYMIN,
                PokemonName::SHAYMINSKY,
                PokemonName::ARCEUS,
                PokemonName::VICTINI,
                PokemonName::KELDEO,
                PokemonName::MELOETTA,
                PokemonName::GENESECT,
                PokemonName::DIANCIE,
                PokemonName::HOOPA,
                PokemonName::VOLCANION,
                PokemonName::MAGEARNA,
                PokemonName::MARSHADOW,
                PokemonName::ZERAORA,
                PokemonName::MELTAN,
                PokemonName::MELMETAL,
                PokemonName::ZARUDE,
            ],
            banned_items: Vec::new(),
            restricted_pokemon: vec![
                // Restricted legendaries (can only use 2)
                PokemonName::MEWTWO,
                PokemonName::LUGIA,
                PokemonName::HOOH,
                PokemonName::KYOGRE,
                PokemonName::KYOGREPRIMAL,
                PokemonName::GROUDON,
                PokemonName::GROUDONPRIMAL,
                PokemonName::RAYQUAZA,
                PokemonName::RAYQUAZAMEGA,
                PokemonName::DIALGA,
                PokemonName::DIALGAORIGIN,
                PokemonName::PALKIA,
                PokemonName::PALKIAORIGIN,
                PokemonName::GIRATINA,
                PokemonName::GIRATINAORIGIN,
                PokemonName::RESHIRAM,
                PokemonName::ZEKROM,
                PokemonName::KYUREM,
                PokemonName::KYUREMWHITE,
                PokemonName::KYUREMBLACK,
                PokemonName::XERNEAS,
                PokemonName::YVELTAL,
                PokemonName::ZYGARDE,
                PokemonName::ZYGARDECOMPLETE,
                PokemonName::COSMOG,
                PokemonName::COSMOEM,
                PokemonName::SOLGALEO,
                PokemonName::LUNALA,
                PokemonName::NECROZMA,
                PokemonName::NECROZMADUSKMANE,
                PokemonName::NECROZMADAWNWINGS,
                PokemonName::ZACIAN,
                PokemonName::ZACIANCROWNED,
                PokemonName::ZAMAZENTA,
                PokemonName::ZAMAZENTACROWNED,
                PokemonName::ETERNATUS,
                PokemonName::CALYREX,
                PokemonName::CALYREXICE,
                PokemonName::CALYREXSHADOW,
                PokemonName::KORAIDON,
                PokemonName::MIRAIDON,
            ],
            restricted_limit: 2,
        });
        
        // Standard doubles configuration
        configs.insert("doubles_ou".to_string(), FormatConfig {
            banned_pokemon: vec![
                // Similar to singles but adjusted for doubles
                PokemonName::MEWTWO,
                PokemonName::LUGIA,
                PokemonName::HOOH,
                PokemonName::KYOGRE,
                PokemonName::KYOGREPRIMAL,
                PokemonName::GROUDON,
                PokemonName::GROUDONPRIMAL,
                PokemonName::RAYQUAZA,
                PokemonName::RAYQUAZAMEGA,
                PokemonName::DIALGA,
                PokemonName::DIALGAORIGIN,
                PokemonName::PALKIA,
                PokemonName::PALKIAORIGIN,
                PokemonName::GIRATINA,
                PokemonName::GIRATINAORIGIN,
                PokemonName::ARCEUS,
                PokemonName::RESHIRAM,
                PokemonName::ZEKROM,
                PokemonName::KYUREM,
                PokemonName::KYUREMWHITE,
                PokemonName::XERNEAS,
                PokemonName::YVELTAL,
                PokemonName::ZYGARDECOMPLETE,
                PokemonName::SOLGALEO,
                PokemonName::LUNALA,
                PokemonName::NECROZMADUSKMANE,
                PokemonName::NECROZMADAWNWINGS,
                PokemonName::ZACIAN,
                PokemonName::ZACIANCROWNED,
                PokemonName::ZAMAZENTA,
                PokemonName::ZAMAZENTACROWNED,
                PokemonName::ETERNATUS,
                PokemonName::CALYREXICE,
                PokemonName::CALYREXSHADOW,
            ],
            banned_items: Vec::new(),
            restricted_pokemon: Vec::new(),
            restricted_limit: 0,
        });
        
        configs
    };
}

/// Factory for creating battle formats using the registry system
pub struct BattleFormatFactory;

impl BattleFormatFactory {
    /// Create a standard singles OU format
    pub fn singles_ou() -> BattleFormat {
        BattleFormat::Singles
    }
    
    /// Create a standard doubles OU format
    pub fn doubles_ou() -> BattleFormat {
        BattleFormat::Doubles
    }
    
    /// Create a VGC format with specific regulations
    pub fn vgc_2024_reg_g() -> BattleFormat {
        BattleFormat::VGC
    }
    
    /// Create a custom format with specific rules
    pub fn custom(rules: BattleRules) -> BattleFormat {
        BattleFormat::Custom(rules)
    }
    
    /// Create a named format from the registry
    pub fn named(name: &str) -> Option<BattleFormat> {
        if crate::format_registry::FormatRegistry::format_exists(name) {
            Some(BattleFormat::Named(name.to_string()))
        } else {
            None
        }
    }
    
    /// Create a format from a configuration string (backwards compatibility)
    pub fn from_config_name(name: &str) -> Option<BattleFormat> {
        match name {
            "singles_ou" => Some(Self::singles_ou()),
            "doubles_ou" => Some(Self::doubles_ou()),
            "vgc_2024_reg_g" => Some(Self::vgc_2024_reg_g()),
            _ => Self::named(name), // Try registry
        }
    }
    
    /// List all available formats from the registry
    pub fn list_available_formats() -> Vec<String> {
        crate::format_registry::FormatRegistry::list_formats()
    }
}

/// Extended format validation using configurations
pub struct ConfigBasedValidator;

impl ConfigBasedValidator {
    pub fn validate_with_config(
        side: &crate::state::Side,
        format: &BattleFormat,
        config_name: &str,
    ) -> Result<(), String> {
        // First do basic format validation
        crate::battle_format::FormatValidator::validate_team(side, format)?;
        
        // Then apply configuration-based validation
        if let Some(config) = FORMAT_CONFIGS.get(config_name) {
            // Check banned Pokemon
            for pokemon in side.pokemon.into_iter() {
                if config.banned_pokemon.contains(&pokemon.id) {
                    return Err(format!("Pokemon {:?} is banned in format {}", pokemon.id, config_name));
                }
            }
            
            // Check banned items
            for pokemon in side.pokemon.into_iter() {
                if config.banned_items.contains(&pokemon.item) {
                    return Err(format!("Item {:?} is banned in format {}", pokemon.item, config_name));
                }
            }
            
            // Move validation is now handled by engine clauses, not config bans
            
            // Check restricted Pokemon limit
            if config.restricted_limit > 0 {
                let restricted_count = side.pokemon.into_iter()
                    .filter(|p| config.restricted_pokemon.contains(&p.id))
                    .count();
                    
                if restricted_count > config.restricted_limit {
                    return Err(format!(
                        "Team has {} restricted Pokemon, but format allows maximum {}", 
                        restricted_count, 
                        config.restricted_limit
                    ));
                }
            }
        }
        
        Ok(())
    }
}