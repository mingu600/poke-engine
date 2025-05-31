# Multi-Format Pokemon Battle Engine TODO

## Project Overview

Overhaul the existing singles-only Pokemon battle engine to support multiple battle formats (singles, doubles, and potentially others) while maintaining functional equivalence for singles battles. The goal is to preserve the engine's proven reliability while adding flexibility for multi-format support.

## Success Metrics
- ✅ All existing tests pass with new engine
- ✅ Singles performance within 10% of original
- ✅ Doubles battles fully functional
- ✅ Clean API for future format extensions

---

## **Phase 1: Architecture Design & Planning (Weeks 1-2)**

### **1.1 Battle Format Architecture Design**

**Define BattleFormat trait:**
```rust
pub trait BattleFormat {
    fn active_pokemon_count(&self) -> usize;
    fn team_size(&self) -> usize;
    fn valid_targets(&self, move_target: MoveTarget, user_position: BattlePosition) -> Vec<BattlePosition>;
    fn move_priority_groups(&self) -> Vec<Vec<BattlePosition>>;
    fn battle_end_condition(&self, state: &State) -> Option<BattleResult>;
}

pub struct SinglesFormat;
pub struct DoublesFormat;
// Future: TriplesBattleFormat, etc.
```

**Define BattlePosition for universal targeting:**
```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BattlePosition {
    pub side: SideReference,
    pub position: u8, // 0-based position on side (0 for singles, 0-1 for doubles)
}
```

- [ ] Design unified battle format architecture with format-agnostic core
- [ ] Define new state representation that supports variable active Pokemon count
- [ ] Design enhanced targeting system for multi-format support
- [ ] Plan instruction system overhaul for position-aware operations
- [ ] Design backward compatibility layer for existing tests

### **1.2 Enhanced State Representation**

**New flexible state structure:**
```rust
pub struct State<F: BattleFormat> {
    pub format: F,
    pub side_one: Side,
    pub side_two: Side,
    pub weather: Weather,
    pub terrain: Terrain,
    pub trick_room: TrickRoomState,
    pub turn_count: u16,
}

pub struct Side {
    pub pokemon: SidePokemon, // Still 6 Pokemon max
    pub active_positions: Vec<Option<PokemonIndex>>, // Variable length based on format
    pub side_conditions: SideConditions,
    // Move per-Pokemon state to Pokemon struct or position-indexed
    pub position_states: Vec<PositionState>, // Boosts, volatiles per active position
    // ... rest of side data
}

pub struct PositionState {
    pub stat_boosts: StatBoosts,
    pub volatile_statuses: HashSet<PokemonVolatileStatus>,
    pub substitute_health: i16,
    // Other position-specific state
}
```

### **1.3 Enhanced Targeting System**

**Comprehensive MoveTarget enum:**
```rust
pub enum MoveTarget {
    // Self-targeting
    User,
    UserSide,
    UserAndAllies,
    
    // Single opponent targeting
    AdjacentOpponent,
    RandomOpponent,
    
    // Multi-opponent targeting
    AllAdjacentOpponents,
    AllOpponents,
    OpponentSide,
    
    // Ally targeting (doubles+)
    AdjacentAlly,
    AllAllies,
    AllyOrUser,
    
    // Multi-target
    AllAdjacent,
    AllOthers,
    Everyone,
    
    // Field targeting
    EntireField,
    
    // Special cases
    UserOrAdjacentAlly, // Assist, etc.
    Manual, // Player chooses target
}
```

---

## **Phase 2: Core Implementation (Weeks 3-6)**

### **2.1 Battle Format Implementation**

**File: `src/battle_format.rs`**
- [ ] Implement `BattleFormat` trait
- [ ] Create `SinglesFormat` and `DoublesFormat` structs
- [ ] Implement targeting validation logic
- [ ] Add format-specific battle rules

### **2.2 State System Overhaul**

**File: `src/state_v2.rs` (parallel implementation)**
- [ ] Implement new flexible `State<F>` struct
- [ ] Create `PositionState` for per-position tracking
- [ ] Implement state serialization/deserialization
- [ ] Add migration utilities from old state format

### **2.3 Enhanced Instruction System**

**File: `src/instruction_v2.rs`**
```rust
pub enum Instruction {
    // Position-aware instructions
    Damage(DamageInstruction),
    Heal(HealInstruction),
    Boost(BoostInstruction),
    ApplyVolatileStatus(ApplyVolatileStatusInstruction),
    // ... all existing instructions with position targeting
    
    // New multi-position instructions
    MultiDamage(MultiDamageInstruction),
    SwapPositions(SwapPositionsInstruction),
    // ...
}

pub struct DamageInstruction {
    pub target_position: BattlePosition,
    pub damage_amount: i16,
}

pub struct MultiDamageInstruction {
    pub targets: Vec<(BattlePosition, i16)>, // Position + damage pairs
}
```

- [ ] Refactor instruction system for position-based operations
- [ ] Implement position-aware damage, heal, and boost instructions
- [ ] Add multi-target instruction variants
- [ ] Update instruction application logic for multiple positions

### **2.4 Move Database Enhancement**

**File: `src/choices_v2.rs`**
- [ ] Add targeting classification to all moves
- [ ] Implement move effect resolution for multiple targets
- [ ] Add doubles-specific move mechanics (spread damage reduction, etc.)
- [ ] Create move validation system for format compatibility

---

## **Phase 3: Compatibility & Testing (Weeks 7-8)**

### **3.1 Backward Compatibility Layer**

**File: `src/compatibility.rs`**
```rust
// Adapter to convert old state format to new
pub fn migrate_state_v1_to_v2(old_state: &state::State) -> state_v2::State<SinglesFormat>;

// Wrapper to make new engine look like old for existing tests
pub struct CompatibilityWrapper<F: BattleFormat> {
    inner: state_v2::State<F>,
}

impl CompatibilityWrapper<SinglesFormat> {
    // Implement old State interface methods
    pub fn get_active(&self, side: SideReference) -> &Pokemon { ... }
    // ... other compatibility methods
}
```

- [ ] Create compatibility layer for existing test suite
- [ ] Implement state migration utilities
- [ ] Create wrapper interface for old API compatibility
- [ ] Test migration accuracy and completeness

### **3.2 Test Migration Strategy**

**Update existing test files:**
1. **Minimal changes**: Update imports and struct initialization
2. **Add compatibility layer**: Wrap new engine in old interface
3. **Validate equivalence**: Ensure identical behavior for singles

**Test validation approach:**
```rust
#[test]
fn test_battle_equivalence() {
    let old_state = create_old_battle_state();
    let new_state = migrate_state_v1_to_v2(&old_state);
    
    // Run same sequence of moves on both
    assert_eq!(old_result, new_result);
}
```

- [ ] Validate singles functionality matches original engine
- [ ] Update test imports and initialization
- [ ] Implement battle equivalence testing
- [ ] Ensure all existing tests pass with new engine

---

## **Phase 4: Advanced Features (Weeks 9-12)**

### **4.1 Core System Updates**

- [ ] Implement format-aware move choice generation
- [ ] Refactor damage calculation for multi-target scenarios
- [ ] Update abilities system for position-aware interactions
- [ ] Update items system for multi-format support

### **4.2 Doubles-Specific Mechanics**

**Implement key doubles features:**
- [ ] Spread move damage reduction (75% in doubles)
- [ ] Position-based abilities (Storm Drain, Lightning Rod, Telepathy)
- [ ] Partner interactions (Helping Hand, Follow Me)
- [ ] Switching mechanics with multiple active Pokemon
- [ ] Doubles-specific move behaviors (Beat Up, etc.)

### **4.3 Enhanced Choice Generation**

**File: `src/choice_generation_v2.rs`**
```rust
pub fn generate_move_choices<F: BattleFormat>(
    state: &State<F>,
    side: SideReference,
    position: Option<u8>, // None = all positions
) -> Vec<MoveChoice> {
    // Generate format-appropriate choices
    // Include target selection for applicable moves
}

pub struct MoveChoice {
    pub user_position: BattlePosition,
    pub move_choice: Choices,
    pub target_position: Option<BattlePosition>, // Some if manual targeting required
}
```

- [ ] Implement format-aware choice generation
- [ ] Add target selection for manual targeting moves
- [ ] Handle position-based move availability
- [ ] Implement switching with multiple active Pokemon

### **4.4 Performance Optimization**

- [ ] Profile new vs old engine performance
- [ ] Optimize critical paths for state operations
- [ ] Implement efficient position indexing
- [ ] Add benchmarks for regression testing

---

## **Phase 5: Integration & Validation (Weeks 13-14)**

### **5.1 Full Engine Integration**

**Replace old engine:**
1. Update `src/lib.rs` to export new modules
2. Update CLI interface for format selection
3. Update Python bindings
4. Update battle environment

- [ ] Update main library exports
- [ ] Add format selection to CLI interface
- [ ] Update battle environment for multi-format support
- [ ] Update Python bindings with new API

### **5.2 Comprehensive Testing**

**Singles validation:**
- [ ] All existing tests pass with new engine
- [ ] Battle outcomes identical to original
- [ ] Performance within 10% of original

**Doubles testing:**
- [ ] Create comprehensive doubles test suite
- [ ] Test all targeting patterns
- [ ] Validate doubles-specific mechanics
- [ ] Test edge cases and interactions

---

## **Phase 6: Documentation & Examples (Week 15)**

### **6.1 Documentation Updates**

- [ ] Update CLAUDE.md with new architecture
- [ ] Document migration process
- [ ] Add format-specific usage examples
- [ ] Update API documentation

### **6.2 Example Implementation**

```rust
// src/examples/multi_format_battle.rs
use poke_engine::{SinglesFormat, DoublesFormat, State, BattleEnvironment};

fn main() {
    // Singles battle (backward compatible)
    let singles_state = State::new(SinglesFormat);
    
    // Doubles battle (new functionality)
    let doubles_state = State::new(DoublesFormat);
    
    // Battle environment supports both
    let env = BattleEnvironment::new(doubles_state);
}
```

- [ ] Create multi-format battle examples
- [ ] Add format comparison demonstrations
- [ ] Document best practices for new format implementations

---

## **Implementation Priority & Risk Mitigation**

### **Critical Path:**
1. **State representation** → **Instruction system** → **Choice generation** → **Testing**

### **Risk Mitigation:**
- **Parallel development**: Keep old engine working during development
- **Incremental testing**: Validate each component individually
- **Performance monitoring**: Continuous benchmarking
- **Rollback plan**: Maintain old engine as fallback

### **Key Files to Create/Modify:**

**New Files:**
- `src/battle_format.rs` - Format trait and implementations
- `src/state_v2.rs` - New state representation
- `src/instruction_v2.rs` - Enhanced instruction system
- `src/choices_v2.rs` - Enhanced move database
- `src/compatibility.rs` - Backward compatibility layer
- `src/choice_generation_v2.rs` - Format-aware choice generation

**Modified Files:**
- `src/lib.rs` - Updated exports
- `src/io.rs` - CLI format selection
- `src/genx/battle_environment.rs` - Multi-format support
- All test files - Updated for new API
- `CLAUDE.md` - Architecture documentation

---

## **Notes & Considerations**

### **Functional Equivalence Requirements:**
- All existing test cases must pass
- Battle outcomes must be identical for singles
- Performance degradation < 10%
- API compatibility layer for smooth migration

### **Future Extensibility:**
- Easy addition of new formats (triples, etc.)
- Modular targeting system
- Format-specific rule overrides
- Plugin architecture for custom mechanics

### **Development Guidelines:**
- Maintain engine's proven reliability
- Preserve performance characteristics
- Keep clean, readable code structure
- Comprehensive testing at each phase