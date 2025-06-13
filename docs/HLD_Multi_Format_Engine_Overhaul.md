# High-Level Design: Multi-Format Pokemon Battle Engine Overhaul

## Table of Contents
1. [Project Overview](#project-overview)
2. [Current Architecture Analysis](#current-architecture-analysis)
3. [Target Architecture](#target-architecture)
4. [Implementation Roadmap](#implementation-roadmap)
5. [Technical Considerations](#technical-considerations)
6. [Success Criteria](#success-criteria)

## Project Overview

### Objectives
We are implementing a major architectural overhaul of the Pokemon battle engine with two primary goals:

1. **Multi-Format Support**: Extend the engine to support multiple battle formats (singles, doubles, VGC) with format-specific rules and mechanics
2. **Rustemon Integration**: Utilize rustemon (Rust wrapper for PokeAPI) as the primary data source for Pokemon, moves, abilities, and items

This is a **version 2 rewrite** that prioritizes flexibility, accuracy, and maintainability over strict backwards compatibility. Substantial rewrites are expected and acceptable.

### Design Philosophy
- **KISS (Keep It Simple, Stupid)**: Solutions must be straightforward and easy to understand
- **YAGNI (You Aren't Gonna Need It)**: Focus only on immediate requirements, avoid speculative features
- **Data Accuracy**: Leverage rustemon/PokeAPI as the canonical source of Pokemon data
- **Performance**: Maintain or improve performance for existing single battle scenarios

## Current Architecture Analysis

### Existing Engine Structure

#### Core Components
- **State Management** (`src/state.rs`): Battle state serialization via `State::deserialize`
- **Instruction System** (`src/instruction.rs`): `StateInstructions` containing probabilistic game state modifications
- **Choice System** (`src/choices.rs`): Move and switch choices with battle mechanics
- **Pokemon Data** (`src/pokemon.rs`): Custom Pokemon name enums and structures
- **Generation Modules**: Feature-flagged code for generation-specific mechanics
  - `src/gen1/`, `src/gen2/`, `src/gen3/`, `src/genx/`
- **Battle Environment** (`src/genx/battle_environment.rs`): Testing framework for AI battles

#### Data Sources
- **Static JSON Files**:
  - `data/moves.json`: Move data with power, accuracy, effects
  - `data/pokedex.json`: Pokemon base stats, types, abilities
  - `data/random_teams.json`: Pre-generated teams for testing
- **Hardcoded Enums**: `PokemonName` with 1400+ variants

#### Current Battle Flow
1. State deserialization from string format
2. Move choice generation based on available moves (assumes any move can be used)
3. Instruction generation from move pairs
4. Probabilistic instruction execution
5. State updates and serialization

### Key Limitations

1. **Singles-Only Architecture**: 
   - Hardcoded assumption of 1v1 battles
   - No support for multiple active Pokemon per side
   - Move targeting assumes single opponent

2. **Static Data Management**:
   - Manual maintenance of JSON data files
   - Risk of data inconsistencies with official sources
   - No automated updates from canonical Pokemon data

3. **Tight Coupling**:
   - Battle mechanics tightly coupled with data structures
   - Difficult to extend for new formats
   - Generation-specific code scattered across modules

4. **Limited Extensibility**:
   - Hard to add new battle formats
   - Complex to implement format-specific rules
   - No abstraction for different targeting mechanics

## Target Architecture

### 1. Data Layer Overhaul

#### Rustemon Integration Strategy

**Primary Benefits of Rustemon**:
- Comprehensive Pokemon data (1000+ Pokemon, 900+ moves, 300+ abilities, 2000+ items)
- Type-safe Rust integration with official PokeAPI data
- Generation-specific historical data and changes
- Built-in HTTP caching and async support

**Integration Approach**:
```rust
// Rustemon as data source layer
rustemon::pokemon::Pokemon -> engine::data::BattlePokemonData
rustemon::moves::Move -> engine::data::BattleMoveData  
rustemon::items::Item -> engine::data::BattleItemData
```

**Data Generation Pipeline**:
- Build-time data extraction from rustemon
- Static file generation for offline functionality
- Conversion layer between rustemon types and engine battle types
- Preservation of `random_teams.json` for testing

#### Proposed Data Architecture
```rust
// Engine-specific data structures that compose rustemon data
// Based on current v1 Pokemon struct fields from src/state.rs
pub struct BattlePokemonData {
    pub id: PokemonName,                             // Engine Pokemon name enum
    pub types: (PokemonType, PokemonType),           // Engine type enum
    pub base_stats: BaseStatBlock,                   // HP, Atk, Def, SpA, SpD, Spe
    pub abilities: Vec<Abilities>,                   // Engine ability enum
    pub weight_kg: f32,                              // For weight-based moves
    // Rustemon source data for validation/reference
    pub rustemon_data: rustemon::model::pokemon::Pokemon,
}

pub struct BattleMoveData {
    pub id: Choices,                                 // Engine move enum
    pub power: Option<u8>,                           // Base power
    pub accuracy: Option<u8>,                        // Accuracy percentage
    pub pp: u8,                                      // Power Points
    pub category: MoveCategory,                      // Physical/Special/Status
    pub move_type: PokemonType,                      // Move type
    pub target: MoveTarget,                          // Engine target enum (1:1 with rustemon)
    // Rustemon source data for validation/reference
    pub rustemon_data: rustemon::model::moves::Move,
}
```

### 2. Multi-Format Battle System

#### Battle Format Architecture
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum BattleFormat {
    Singles,
    Doubles,
    VGC,
    Named(String),      // Named format from the format registry
    Custom(BattleRules)
}

#[derive(Debug, Clone, PartialEq)]
pub struct BattleRules {
    pub party_size: usize,                   // Total Pokemon in party (6 for most formats)
    pub team_size: usize,                    // Pokemon brought to battle (6 for singles/doubles, 4 for VGC)
    pub active_pokemon: usize,               // Active Pokemon per side (1 for singles, 2 for doubles)
    pub format_clauses: Vec<FormatClause>,   // Species, Sleep, etc.
}

#[derive(Debug, Clone, PartialEq)]
pub enum FormatClause {
    SpeciesClause,           // No duplicate species
    SleepClause,             // Limit sleeping opponents
    FreezeClause,            // Limit freezing opponents  
    OHKOClause,              // Ban OHKO moves
    EvasionClause,           // Ban evasion moves
    ItemClause,              // No duplicate items
    Custom(String),          // Custom format-specific rules
}
```

#### Enhanced Battle State

**Implementation Note**: Our implementation maintains existing struct names (`State`, `Side`) and includes backward compatibility fields to ensure the existing singles engine continues to work without modification. This incremental approach allows for gradual migration while preserving engine accuracy.

```rust
// Current implementation maintains existing field names for compatibility
pub struct State {
    pub format: BattleFormat,           // NEW: Format awareness
    pub side_one: Side,                 // Enhanced with multi-format support
    pub side_two: Side,                 // Enhanced with multi-format support
    pub weather: StateWeather,          // Existing field
    pub terrain: StateTerrain,          // Existing field
    pub trick_room: StateTrickRoom,     // Existing field
    pub team_preview: bool,             // Existing field
    pub use_last_used_move: bool,       // Existing field
    pub use_damage_dealt: bool,         // Existing field
}

pub struct Side {
    pub active_slots: Vec<Option<ActivePokemon>>,  // NEW: Variable size based on format
    pub reserve: SidePokemon,                      // NEW: Non-active team members
    pub side_conditions: SideConditions,          // Existing field
    pub wish: (i8, i16),                         // Existing field  
    pub future_sight: (i8, PokemonIndex),        // Existing field
    
    // Backward compatibility fields (delegate to first active Pokemon)
    pub active_index: PokemonIndex,
    pub pokemon: SidePokemon,           // Alias for reserve
    pub volatile_statuses: HashSet<PokemonVolatileStatus>,
    pub substitute_health: i16,
    pub attack_boost: i8,
    pub defense_boost: i8,
    pub special_attack_boost: i8,
    pub special_defense_boost: i8,
    pub speed_boost: i8,
    pub accuracy_boost: i8,
    pub evasion_boost: i8,
    pub last_used_move: LastUsedMove,
    pub damage_dealt: DamageDealt,
    pub baton_passing: bool,
    pub shed_tailing: bool,
    pub force_switch: bool,
    pub force_trapped: bool,
    pub slow_uturn_move: bool,
    pub volatile_status_durations: VolatileStatusDurations,
    pub switch_out_move_second_saved_move: Choices,
}

pub struct ActivePokemon {
    pub position: BattlePosition,
    pub pokemon_index: PokemonIndex,        // Index into reserve
    pub volatile_statuses: HashSet<PokemonVolatileStatus>,
    pub substitute_health: i16,
    pub attack_boost: i8,
    pub defense_boost: i8,
    pub special_attack_boost: i8,
    pub special_defense_boost: i8,
    pub speed_boost: i8,
    pub accuracy_boost: i8,
    pub evasion_boost: i8,
    pub last_used_move: LastUsedMove,
    pub damage_dealt: DamageDealt,
    pub baton_passing: bool,
    pub shed_tailing: bool,
    pub force_switch: bool,
    pub force_trapped: bool,
    pub slow_uturn_move: bool,
    pub volatile_status_durations: VolatileStatusDurations,
    pub switch_out_move_second_saved_move: Choices,
}
```

### 3. Move Targeting System

#### Move Target Enum (1:1 with Rustemon/PokeAPI)
```rust
// Custom enum that maps 1:1 with rustemon move targets
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MoveTarget {
    SpecificMove,           // specific-move (1)
    SelectedPokemonMeFirst, // selected-pokemon-me-first (2)
    Ally,                   // ally (3)
    UsersField,             // users-field (4)
    UserOrAlly,             // user-or-ally (5)
    OpponentsField,         // opponents-field (6)
    User,                   // user (7)
    RandomOpponent,         // random-opponent (8)
    AllOtherPokemon,        // all-other-pokemon (9)
    SelectedPokemon,        // selected-pokemon (10)
    AllOpponents,           // all-opponents (11)
    EntireField,            // entire-field (12)
    UserAndAllies,          // user-and-allies (13)
    AllPokemon,             // all-pokemon (14)
    AllAllies,              // all-allies (15)
    FaintingPokemon,        // fainting-pokemon (16)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BattlePosition {
    pub side: SideReference,
    pub slot: usize,  // 0 for singles, 0-1 for doubles
}
```

#### Targeting Resolution
```rust
pub trait TargetResolver {
    fn resolve_targets(
        &self,
        user_position: BattlePosition,
        move_target: MoveTarget,
        state: &State,
    ) -> Vec<BattlePosition>;
    
    fn is_valid_target(
        &self,
        target: BattlePosition,
        state: &State,
    ) -> bool;
}
```

### 4. Enhanced Instruction System

#### Multi-Target Instructions
```rust
#[derive(PartialEq, Clone)]
pub struct StateInstructions {
    pub percentage: f32,
    pub instruction_list: Vec<Instruction>,
    pub affected_positions: Vec<BattlePosition>,  // Simple list of affected positions
}

// Enhanced instructions for multi-target scenarios
#[derive(PartialEq, Clone)]
pub enum Instruction {
    // Existing instructions...
    Damage(DamageInstruction),
    Heal(HealInstruction),
    
    // New multi-target instructions
    MultiTargetDamage(MultiTargetDamageInstruction),
    PositionSwap(PositionSwapInstruction),
    FormatSpecificEffect(FormatEffectInstruction),
}

#[derive(PartialEq, Clone)]
pub struct DamageInstruction {
    pub target_position: BattlePosition,  // Now includes position
    pub damage_amount: i16,
}
```

### 5. Generation System Enhancement

#### Unified Generation Architecture
```rust
pub trait GenerationRules {
    fn supports_format(&self, format: &BattleFormat) -> bool;
    fn apply_generation_mechanics(&self, state: &mut State);
    fn calculate_damage(&self, context: &DamageContext) -> DamageResult;
    fn resolve_move_effects(&self, context: &MoveContext) -> Vec<Instruction>;
}

// Maintain existing feature flag system while enhancing with rustemon data
#[cfg(feature = "gen1")]
pub struct Gen1Rules;

#[cfg(feature = "gen2")]  
pub struct Gen2Rules;

// etc.
```

## Implementation Roadmap

### 🎯 Progress Summary
- **Phase 1: Data Layer Foundation** ✅ **COMPLETED** 
- **Phase 2: Multi-Format Architecture** ✅ **COMPLETED**
  - **Week 7-8: Battle Format System** ✅ **COMPLETED**
  - **Week 9-10: Battle State Overhaul** ✅ **COMPLETED**
- **Phase 3: Core Battle Mechanics** 🚧 **READY TO START**
- **Phase 4: Testing & Validation** ⏳ **PENDING**

### Phase 1: Data Layer Foundation (4-6 weeks) ✅ COMPLETED

#### Week 1-2: Rustemon Integration ✅ COMPLETED
- [x] Add rustemon dependency to `Cargo.toml`
- [x] Create rustemon client wrapper (`src/data/rustemon_client.rs`)
- [x] Implement basic data fetching for Pokemon, moves, items
- [ ] Add build script for data generation (`build.rs`) - *Deferred to optimization phase*

#### Week 3-4: Data Structure Redesign ✅ COMPLETED
- [x] Design new Pokemon/Move/Item structs in `src/data/` module based on current v1 Pokemon fields
- [x] Implement conversion from rustemon types to engine types
- [x] Create serialization/deserialization for new structures
- [ ] Generate static data files from rustemon (no learnable moves validation needed) - *Deferred to optimization phase*

#### Week 5-6: Migration & Testing ✅ COMPLETED
- [x] Create migration utilities for existing test data
- [x] Update existing tests to use new data structures
- [x] Validate data accuracy against current JSON files
- [x] Performance testing for data loading

**Phase 1 Deliverables Completed:**
- Rustemon v4.2.0 integrated with async HTTP client
- `PokeEngineDataClient` wrapper for easy PokeAPI access
- `BattlePokemonData`, `BattleMoveData`, `BattleItemData` structures based on v1 Pokemon fields
- `MoveTarget` enum with 1:1 rustemon/PokeAPI mapping (16 target types)
- Type-safe conversion utilities with comprehensive error handling
- Unit tests validating data conversion and API integration
- Architecture ready for multi-format battle system development

**Phase 1 Deferred Items:**
- Build-time data generation pipeline (`build.rs`) - Can be added later for optimization
- Static data file generation - Currently using live HTTP requests to rustemon/PokeAPI

### Phase 2: Multi-Format Architecture (6-8 weeks) ✅ COMPLETED

#### Week 7-8: Battle Format System ✅ COMPLETED
- [x] Implement `BattleFormat` enum and `BattleRules` struct
- [x] Create format-specific configuration system
- [x] Design format validation and enforcement mechanisms  
- [x] Add format detection and initialization
- [x] **NEW:** Implement unified format registry system
- [x] **NEW:** Create engine-focused clause system (Sleep, OHKO, Evasion, etc.)
- [x] **NEW:** Add 8 predefined competitive formats

**Phase 2 Week 7-8 Deliverables Completed:**
- Complete battle format system with Singles, Doubles, VGC, Named, and Custom formats
- Unified format registry (`src/format_registry.rs`) with 8 predefined competitive formats
- Engine-focused clause system: Species, Sleep, Freeze, OHKO, Evasion, Item clauses
- Format enforcement system with runtime validation during battle
- Move targeting system supporting all 16 rustemon/PokeAPI target types
- Format initialization and detection system with battle context management
- Comprehensive test suite: 43 tests across battle format, config, enforcement, and registry
- Clean separation: engine handles battle mechanics, external apps handle team validation

#### Week 9-10: Battle State Overhaul ✅ COMPLETED
- [x] Extend `State` struct for multi-Pokemon battles
- [x] Redesign `BattleSide` for variable active Pokemon
- [x] Implement new `BattlePosition` and targeting systems  
- [x] Update state serialization for new format
- [x] Maintain backward compatibility with existing singles engine
- [x] Add format-aware constructors and initialization

**Phase 2 Completion Summary:**
The multi-format battle system foundation is now complete with full support for singles, doubles, and VGC formats. The enhanced battle state architecture supports multiple active Pokemon per side while maintaining backward compatibility with the existing singles engine. All Phase 2 format tests (43 tests) are passing, validating the robustness of the implementation.

### Phase 3: Core Battle Mechanics (8-10 weeks)

#### Week 11-12: Move Targeting Implementation
- [ ] Integrate rustemon move targets directly into engine
- [ ] Implement format-specific target resolution (singles vs doubles)
- [ ] Add target validation and auto-targeting logic
- [ ] Create targeting resolution engine for rustemon MoveTarget enum

#### Week 13-14: Instruction System Enhancement
- [ ] Extend instructions to handle multi-target scenarios
- [ ] Implement format-aware instruction generation
- [ ] Add support for doubles-specific mechanics
- [ ] Update instruction serialization

### Phase 3: Core Battle Mechanics (8-10 weeks)

#### Week 15-16: Damage Calculation Updates
- [ ] Extend damage calc for multi-target scenarios
- [ ] Implement format-specific damage modifiers  
- [ ] Add support for doubles-specific interactions
- [ ] Handle spread move damage reduction

#### Week 17-18: Choice Generation Enhancement
- [ ] Update move choice generation for multi-format
- [ ] Implement format-aware switch mechanics
- [ ] Add support for doubles switching rules
- [ ] Create choice validation system

#### Week 19-20: Battle Flow Integration
- [ ] Integrate new systems into battle flow
- [ ] Implement turn order calculation for doubles
- [ ] Add format-specific priority resolution
- [ ] Handle simultaneous move execution

#### Week 21-22: AI/Search Algorithm Updates
- [ ] Update MCTS for multi-format support
- [ ] Implement format-aware move choice generation
- [ ] Optimize search performance for doubles complexity
- [ ] Add format-specific evaluation functions

### Phase 4: Testing & Validation (4-6 weeks)

#### Week 23-24: Test Suite Migration & Phase 3 Integration Testing
- [ ] Create `tests/test_phase3_integration.rs` consolidating all Phase 3 tests
- [ ] Port remaining legacy tests to new architecture
- [ ] Maintain test coverage and intent consistency  
- [ ] Add comprehensive multi-format test scenarios
- [ ] Validate Phase 2 integration tests continue passing with Phase 3 changes

#### Week 25-26: Battle Environment Enhancement & Complete System Testing
- [ ] Update battle CLI for format selection
- [ ] Add format-specific battle initialization
- [ ] Implement format validation in battle setup
- [ ] Create format-specific AI players
- [ ] Create `tests/test_phase4_integration.rs` for complete system validation

#### Week 27-28: Performance & Optimization
- [ ] Profile multi-format performance impact using phase integration tests
- [ ] Optimize critical paths for doubles/VGC scenarios
- [ ] Ensure all phase integration tests continue passing after optimizations
- [ ] Create performance benchmarks integrated with phase testing

**Phase 4 Deliverables**:
- Complete test suite migration with phase integration testing
- `test_phase3_integration.rs` and `test_phase4_integration.rs` 
- Performance-optimized multi-format engine
- Comprehensive validation that all phases work together correctly

## Technical Considerations

### Data Consistency & Management

**Rustemon Integration**:
- **Pros**: Official PokeAPI accuracy, type safety, comprehensive data
- **Cons**: Network dependency, potential rate limiting, large data size
- **Solution**: Build-time data generation with static file caching

**Version Management**:
- Pin rustemon version for reproducible builds
- Version data files with generation markers
- Automated testing against known data snapshots

### Performance Impact

**Complexity Analysis**:
- Singles: O(n) move choices per Pokemon
- Doubles: O(n²) move combinations, O(m²) targeting combinations  
- State space increases exponentially with active Pokemon count

**Optimization Strategies**:
- Lazy evaluation of move combinations
- Cached move target resolution
- Format-specific pruning in search algorithms
- Memory pooling for frequent allocations

### Memory Management

**State Size Considerations**:
- Current singles state: ~1KB serialized
- Estimated doubles state: ~2-4KB serialized
- Battle history grows linearly with turn count

**Memory Optimizations**:
- Copy-on-write for Pokemon data
- Interned strings for common data (move names, etc.)
- Compressed state representation for MCTS

### Error Handling & Validation

**Data Validation**:
- Rustemon data validation at build time
- Runtime format rule validation
- Team composition validation

**Error Recovery**:
- Graceful degradation for data inconsistencies
- Format fallback mechanisms
- Comprehensive error reporting

### Testing Strategy

**Test Categories**:
1. **Unit Tests**: Individual component functionality
2. **Integration Tests**: Cross-component interactions  
3. **Format Tests**: Format-specific rule validation
4. **Performance Tests**: Benchmark comparisons
5. **Data Tests**: Rustemon integration validation
6. **🆕 Phase Integration Tests**: Consolidated validation of complete phases

**Phase Integration Testing Protocol** 🚨 **CRITICAL**:

Each completed phase now has a consolidated integration test file that validates ALL components working together:

- **Phase 2**: `tests/test_phase2_integration.rs` (45 tests)
  - Covers: Battle Format System, Format Registry, Format Enforcement, Format Initialization
  - Command: `cargo test --test test_phase2_integration --release --features gen9,terastallization --no-default-features`

**MANDATORY Testing Workflow**:
1. **Before ANY changes to completed phases**: Run phase integration tests and ensure ALL pass
2. **After implementing changes**: Re-run phase integration tests  
3. **If tests fail**: MUST discuss why they're failing before proceeding with changes
4. **NEVER commit**: Code that breaks existing phase integration tests without explicit approval

**Future Phase Integration Tests**:
- **Phase 3**: `tests/test_phase3_integration.rs` (Core Battle Mechanics validation)
- **Phase 4**: `tests/test_phase4_integration.rs` (Complete system validation)

**Test Migration**:
- Preserve existing test intent and coverage
- Create format-specific test suites
- Add comprehensive multi-format scenarios
- Maintain benchmark performance tests
- **🆕 Consolidate tests into phase integration files** for easy validation

## Success Criteria

### Functional Requirements
1. **Complete Format Support**: 
   - [ ] Singles format maintains existing functionality
   - [ ] Doubles format supports all standard doubles mechanics
   - [ ] VGC format implements official VGC rules and restrictions

2. **Data Accuracy**:
   - [ ] All Pokemon data sourced from rustemon/PokeAPI
   - [ ] Move data matches official sources
   - [ ] Type effectiveness accurate across all generations

3. **Performance**:
   - [ ] No significant performance regression for singles battles
   - [ ] Doubles performance within 3x of singles complexity
   - [ ] MCTS search maintains reasonable response times

### Technical Requirements
4. **Test Coverage**:
   - [x] **Phase 2**: All 45 integration tests pass covering complete multi-format system
   - [ ] **Phase 3**: Integration tests for core battle mechanics with format support
   - [ ] **Phase 4**: Complete system integration tests across all phases  
   - [ ] All existing test scenarios pass with equivalent behavior
   - [ ] New format-specific tests achieve >90% coverage
   - [ ] Performance tests validate optimization targets

5. **Code Quality**:
   - [ ] Clean separation between data layer and battle mechanics
   - [ ] Extensible architecture for future formats
   - [ ] Comprehensive documentation and examples

6. **Integration**:
   - [ ] Python bindings updated for new formats
   - [ ] CLI supports all battle formats
   - [ ] Battle environment works with multi-format scenarios

### Delivery Milestones
- **M1 (Week 6)**: Data layer complete with rustemon integration
- **M2 (Week 14)**: Multi-format architecture operational
- **M3 (Week 22)**: Core battle mechanics updated for all formats
- **M4 (Week 28)**: Testing complete, ready for production use

## Risk Mitigation

### Technical Risks
1. **Performance Degradation**: 
   - Mitigation: Incremental performance testing, optimization sprints
2. **Data Integration Complexity**:
   - Mitigation: Prototype rustemon integration early, validate data accuracy
3. **Architecture Complexity**:
   - Mitigation: Phased implementation, maintain working singles throughout

### Project Risks  
1. **Scope Creep**:
   - Mitigation: Strict adherence to YAGNI principles, defer nice-to-have features
2. **Timeline Pressure**:
   - Mitigation: Prioritize core functionality, implement optional features later
3. **Breaking Changes**:
   - Mitigation: Version 2.0 release, clear migration documentation

---

*This document serves as the architectural blueprint for the multi-format Pokemon battle engine overhaul. It should be updated as implementation progresses and requirements evolve.*