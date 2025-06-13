use crate::choices::{Choices, MOVES, Flags};
use crate::data::move_service::{EngineSpecificMoveData, EngineDataBuilder};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

/// Automated migration tool to extract engine-specific move data from the legacy MOVES HashMap
pub struct MoveMigrationTool;

impl MoveMigrationTool {
    /// Extract all moves that have engine-specific properties
    pub fn extract_special_moves() -> HashMap<Choices, EngineSpecificMoveData> {
        let mut special_moves = HashMap::new();
        
        for (move_id, choice) in MOVES.iter() {
            let mut has_special_data = false;
            let mut builder = EngineDataBuilder::new();
            
            // Check for drain
            if let Some(drain) = choice.drain {
                builder = builder.drain(drain);
                has_special_data = true;
            }
            
            // Check for recoil
            if let Some(recoil) = choice.recoil {
                builder = builder.recoil(recoil);
                has_special_data = true;
            }
            
            // Check for crash
            if let Some(crash) = choice.crash {
                builder = builder.crash(crash);
                has_special_data = true;
            }
            
            // Check for heal
            if let Some(heal) = choice.heal.clone() {
                builder = builder.heal(heal);
                has_special_data = true;
            }
            
            // Check for boost
            if let Some(boost) = choice.boost.clone() {
                builder = builder.boost(boost);
                has_special_data = true;
            }
            
            // Check for secondaries
            if let Some(secondaries) = choice.secondaries.clone() {
                builder = builder.secondaries(secondaries);
                has_special_data = true;
            }
            
            // Check for status
            if let Some(status) = choice.status.clone() {
                builder = builder.status(status);
                has_special_data = true;
            }
            
            // Check for volatile_status
            if let Some(volatile_status) = choice.volatile_status.clone() {
                builder = builder.volatile_status(volatile_status);
                has_special_data = true;
            }
            
            // Check for side_condition
            if let Some(side_condition) = choice.side_condition.clone() {
                builder = builder.side_condition(side_condition);
                has_special_data = true;
            }
            
            // Always extract flags as they might contain important battle mechanics
            let flags = extract_flags_from_choice(choice);
            builder = builder.flags(flags);
            
            if has_special_data {
                let engine_data = builder.build();
                special_moves.insert(*move_id, engine_data);
            }
        }
        
        special_moves
    }
    
    /// Generate registration code for MoveFactory
    pub fn generate_registration_code(output_path: &str) -> std::io::Result<()> {
        let special_moves = Self::extract_special_moves();
        let mut file = File::create(output_path)?;
        
        writeln!(file, "// Auto-generated move registrations for MoveFactory")?;
        writeln!(file, "// Generated from legacy MOVES HashMap\n")?;
        writeln!(file, "use crate::choices::{{Choices, Flags, Secondary, Effect, Boost, StatBoosts, VolatileStatus, Heal, Status, SideCondition}};")?;
        writeln!(file, "use crate::data::move_service::EngineDataBuilder;")?;
        writeln!(file, "use crate::data::types::MoveTarget;")?;
        writeln!(file, "use crate::state::{{PokemonStatus, PokemonType, PokemonSideCondition}};")?;
        writeln!(file, "use crate::engine::state::PokemonVolatileStatus;")?;
        writeln!(file, "use super::move_factory::MoveFactory;\n")?;
        
        writeln!(file, "impl MoveFactory {{")?;
        writeln!(file, "    /// Register all engine-specific move data")?;
        writeln!(file, "    pub async fn register_all_engine_data(&self) {{")?;
        
        // Group moves by category for better organization
        let mut drain_moves = Vec::new();
        let mut recoil_moves = Vec::new();
        let mut boost_moves = Vec::new();
        let mut status_moves = Vec::new();
        let mut secondary_moves = Vec::new();
        let mut other_moves = Vec::new();
        
        for (move_id, engine_data) in special_moves.iter() {
            if engine_data.drain.is_some() {
                drain_moves.push((move_id, engine_data));
            } else if engine_data.recoil.is_some() {
                recoil_moves.push((move_id, engine_data));
            } else if engine_data.boost.is_some() {
                boost_moves.push((move_id, engine_data));
            } else if engine_data.status.is_some() || engine_data.volatile_status.is_some() {
                status_moves.push((move_id, engine_data));
            } else if engine_data.secondaries.is_some() {
                secondary_moves.push((move_id, engine_data));
            } else {
                other_moves.push((move_id, engine_data));
            }
        }
        
        // Generate drain moves
        if !drain_moves.is_empty() {
            writeln!(file, "\n        // Draining moves")?;
            for (move_id, engine_data) in drain_moves {
                Self::write_registration(&mut file, move_id, engine_data)?;
            }
        }
        
        // Generate recoil moves
        if !recoil_moves.is_empty() {
            writeln!(file, "\n        // Recoil moves")?;
            for (move_id, engine_data) in recoil_moves {
                Self::write_registration(&mut file, move_id, engine_data)?;
            }
        }
        
        // Generate boost moves
        if !boost_moves.is_empty() {
            writeln!(file, "\n        // Stat-boosting moves")?;
            for (move_id, engine_data) in boost_moves {
                Self::write_registration(&mut file, move_id, engine_data)?;
            }
        }
        
        // Generate status moves
        if !status_moves.is_empty() {
            writeln!(file, "\n        // Status-inflicting moves")?;
            for (move_id, engine_data) in status_moves {
                Self::write_registration(&mut file, move_id, engine_data)?;
            }
        }
        
        // Generate secondary effect moves
        if !secondary_moves.is_empty() {
            writeln!(file, "\n        // Moves with secondary effects")?;
            for (move_id, engine_data) in secondary_moves {
                Self::write_registration(&mut file, move_id, engine_data)?;
            }
        }
        
        // Generate other moves
        if !other_moves.is_empty() {
            writeln!(file, "\n        // Other special moves")?;
            for (move_id, engine_data) in other_moves {
                Self::write_registration(&mut file, move_id, engine_data)?;
            }
        }
        
        writeln!(file, "    }}")?;
        writeln!(file, "}}")?;
        
        Ok(())
    }
    
    fn write_registration(file: &mut File, move_id: &Choices, engine_data: &EngineSpecificMoveData) -> std::io::Result<()> {
        writeln!(file, "        self.service.register_engine_data(")?;
        writeln!(file, "            Choices::{:?},", move_id)?;
        writeln!(file, "            EngineDataBuilder::new()")?;
        
        if let Some(drain) = engine_data.drain {
            writeln!(file, "                .drain({})", drain)?;
        }
        if let Some(recoil) = engine_data.recoil {
            writeln!(file, "                .recoil({})", recoil)?;
        }
        if let Some(crash) = engine_data.crash {
            writeln!(file, "                .crash({})", crash)?;
        }
        if let Some(ref heal) = engine_data.heal {
            Self::write_heal_field(file, heal)?;
        }
        if let Some(ref boost) = engine_data.boost {
            Self::write_boost_field(file, boost)?;
        }
        if let Some(ref secondaries) = engine_data.secondaries {
            Self::write_secondaries_field(file, secondaries)?;
        }
        if let Some(ref status) = engine_data.status {
            Self::write_status_field(file, status)?;
        }
        if let Some(ref volatile_status) = engine_data.volatile_status {
            Self::write_volatile_status_field(file, volatile_status)?;
        }
        if let Some(ref side_condition) = engine_data.side_condition {
            Self::write_side_condition_field(file, side_condition)?;
        }
        
        // Write flags if any are set
        let flags = &engine_data.flags;
        if flags.bite || flags.bullet || flags.charge || flags.contact || flags.drag ||
           flags.heal || flags.powder || flags.protect || flags.pulse || flags.punch ||
           flags.recharge || flags.reflectable || flags.slicing || flags.sound || 
           flags.pivot || flags.wind {
            writeln!(file, "                .flags(Flags {{")?;
            if flags.bite { writeln!(file, "                    bite: true,")?; }
            if flags.bullet { writeln!(file, "                    bullet: true,")?; }
            if flags.charge { writeln!(file, "                    charge: true,")?; }
            if flags.contact { writeln!(file, "                    contact: true,")?; }
            if flags.drag { writeln!(file, "                    drag: true,")?; }
            if flags.heal { writeln!(file, "                    heal: true,")?; }
            if flags.powder { writeln!(file, "                    powder: true,")?; }
            if flags.protect { writeln!(file, "                    protect: true,")?; }
            if flags.pulse { writeln!(file, "                    pulse: true,")?; }
            if flags.punch { writeln!(file, "                    punch: true,")?; }
            if flags.recharge { writeln!(file, "                    recharge: true,")?; }
            if flags.reflectable { writeln!(file, "                    reflectable: true,")?; }
            if flags.slicing { writeln!(file, "                    slicing: true,")?; }
            if flags.sound { writeln!(file, "                    sound: true,")?; }
            if flags.pivot { writeln!(file, "                    pivot: true,")?; }
            if flags.wind { writeln!(file, "                    wind: true,")?; }
            writeln!(file, "                    ..Default::default()")?;
            writeln!(file, "                }})")?;
        }
        
        writeln!(file, "                .build()")?;
        writeln!(file, "        ).await;")?;
        
        Ok(())
    }
    
    fn write_heal_field(file: &mut File, heal: &crate::choices::Heal) -> std::io::Result<()> {
        writeln!(file, "                .heal(Heal {{")?;
        writeln!(file, "                    target: MoveTarget::{:?},", heal.target)?;
        writeln!(file, "                    amount: {},", if heal.amount.fract() == 0.0 { format!("{:.1}", heal.amount) } else { heal.amount.to_string() })?;
        writeln!(file, "                }})")
    }
    
    fn write_boost_field(file: &mut File, boost: &crate::choices::Boost) -> std::io::Result<()> {
        writeln!(file, "                .boost(Boost {{")?;
        writeln!(file, "                    target: MoveTarget::{:?},", boost.target)?;
        writeln!(file, "                    boosts: StatBoosts {{")?;
        writeln!(file, "                        attack: {},", boost.boosts.attack)?;
        writeln!(file, "                        defense: {},", boost.boosts.defense)?;
        writeln!(file, "                        special_attack: {},", boost.boosts.special_attack)?;
        writeln!(file, "                        special_defense: {},", boost.boosts.special_defense)?;
        writeln!(file, "                        speed: {},", boost.boosts.speed)?;
        writeln!(file, "                        accuracy: {},", boost.boosts.accuracy)?;
        writeln!(file, "                    }},")?;
        writeln!(file, "                }})")
    }
    
    fn write_secondaries_field(file: &mut File, secondaries: &Vec<crate::choices::Secondary>) -> std::io::Result<()> {
        writeln!(file, "                .secondaries(vec![")?;
        for secondary in secondaries {
            writeln!(file, "                    Secondary {{")?;
            writeln!(file, "                        chance: {},", if secondary.chance.fract() == 0.0 { format!("{:.1}", secondary.chance) } else { secondary.chance.to_string() })?;
            writeln!(file, "                        target: MoveTarget::{:?},", secondary.target)?;
            write!(file, "                        effect: ")?;
            Self::write_effect_field(file, &secondary.effect)?;
            writeln!(file, ",")?;
            writeln!(file, "                    }},")?;
        }
        writeln!(file, "                ])")
    }
    
    fn write_effect_field(file: &mut File, effect: &crate::choices::Effect) -> std::io::Result<()> {
        match effect {
            crate::choices::Effect::Status(status) => {
                writeln!(file, "Effect::Status(PokemonStatus::{:?})", status)
            }
            crate::choices::Effect::VolatileStatus(volatile_status) => {
                writeln!(file, "Effect::VolatileStatus(PokemonVolatileStatus::{:?})", volatile_status)
            }
            crate::choices::Effect::Boost(stat_boosts) => {
                writeln!(file, "Effect::Boost(StatBoosts {{")?;
                writeln!(file, "                            attack: {},", stat_boosts.attack)?;
                writeln!(file, "                            defense: {},", stat_boosts.defense)?;
                writeln!(file, "                            special_attack: {},", stat_boosts.special_attack)?;
                writeln!(file, "                            special_defense: {},", stat_boosts.special_defense)?;
                writeln!(file, "                            speed: {},", stat_boosts.speed)?;
                writeln!(file, "                            accuracy: {},", stat_boosts.accuracy)?;
                writeln!(file, "                        }})")
            }
            crate::choices::Effect::Heal(heal_amount) => {
                writeln!(file, "Effect::Heal({})", heal_amount)
            }
            crate::choices::Effect::RemoveItem => {
                writeln!(file, "Effect::RemoveItem")
            }
        }
    }
    
    fn write_status_field(file: &mut File, status: &crate::choices::Status) -> std::io::Result<()> {
        writeln!(file, "                .status(Status {{")?;
        writeln!(file, "                    target: MoveTarget::{:?},", status.target)?;
        writeln!(file, "                    status: PokemonStatus::{:?},", status.status)?;
        writeln!(file, "                }})")
    }
    
    fn write_volatile_status_field(file: &mut File, volatile_status: &crate::choices::VolatileStatus) -> std::io::Result<()> {
        writeln!(file, "                .volatile_status(VolatileStatus {{")?;
        writeln!(file, "                    target: MoveTarget::{:?},", volatile_status.target)?;
        writeln!(file, "                    volatile_status: PokemonVolatileStatus::{:?},", volatile_status.volatile_status)?;
        writeln!(file, "                }})")
    }
    
    fn write_side_condition_field(file: &mut File, side_condition: &crate::choices::SideCondition) -> std::io::Result<()> {
        writeln!(file, "                .side_condition(SideCondition {{")?;
        writeln!(file, "                    target: MoveTarget::{:?},", side_condition.target)?;
        writeln!(file, "                    condition: PokemonSideCondition::{:?},", side_condition.condition)?;
        writeln!(file, "                }})")
    }
    
    /// Generate a summary report of the migration
    pub fn generate_summary_report() -> String {
        let special_moves = Self::extract_special_moves();
        let total_moves = MOVES.len();
        let special_count = special_moves.len();
        
        let mut drain_count = 0;
        let mut recoil_count = 0;
        let mut boost_count = 0;
        let mut status_count = 0;
        let mut secondary_count = 0;
        
        for (_, engine_data) in special_moves.iter() {
            if engine_data.drain.is_some() { drain_count += 1; }
            if engine_data.recoil.is_some() { recoil_count += 1; }
            if engine_data.boost.is_some() { boost_count += 1; }
            if engine_data.status.is_some() || engine_data.volatile_status.is_some() { status_count += 1; }
            if engine_data.secondaries.is_some() { secondary_count += 1; }
        }
        
        format!(
            "Move Migration Summary:\n\
             Total moves in MOVES HashMap: {}\n\
             Moves with engine-specific data: {}\n\
             - Draining moves: {}\n\
             - Recoil moves: {}\n\
             - Stat-boosting moves: {}\n\
             - Status moves: {}\n\
             - Moves with secondary effects: {}\n",
            total_moves, special_count, drain_count, recoil_count, boost_count, status_count, secondary_count
        )
    }
}

/// Extract flags from legacy Choice struct
fn extract_flags_from_choice(choice: &crate::choices::Choice) -> Flags {
    Flags {
        bite: choice.flags.bite,
        bullet: choice.flags.bullet,
        charge: choice.flags.charge,
        contact: choice.flags.contact,
        drag: choice.flags.drag,
        heal: choice.flags.heal,
        powder: choice.flags.powder,
        protect: choice.flags.protect,
        pulse: choice.flags.pulse,
        punch: choice.flags.punch,
        recharge: choice.flags.recharge,
        reflectable: choice.flags.reflectable,
        slicing: choice.flags.slicing,
        sound: choice.flags.sound,
        pivot: choice.flags.pivot,
        wind: choice.flags.wind,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_extract_special_moves() {
        let special_moves = MoveMigrationTool::extract_special_moves();
        
        // Verify some known special moves are extracted
        assert!(special_moves.contains_key(&Choices::ABSORB), "Should extract ABSORB (drain move)");
        assert!(special_moves.contains_key(&Choices::BRAVEBIRD), "Should extract BRAVEBIRD (recoil move)");
        assert!(special_moves.contains_key(&Choices::AGILITY), "Should extract AGILITY (boost move)");
        assert!(special_moves.contains_key(&Choices::THUNDERBOLT), "Should extract THUNDERBOLT (secondary effect)");
        
        // Verify basic moves without special properties are not extracted
        assert!(!special_moves.contains_key(&Choices::TACKLE), "Should not extract TACKLE (basic move)");
    }
    
    #[test]
    fn test_generate_summary() {
        let summary = MoveMigrationTool::generate_summary_report();
        println!("{}", summary);
        assert!(summary.contains("Total moves"));
        assert!(summary.contains("Moves with engine-specific data"));
    }
}