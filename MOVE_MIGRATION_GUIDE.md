# Move Migration Guide

This guide explains how to use the automated move migration system to extract engine-specific data from the legacy `MOVES` HashMap and integrate it with the new rustemon-based move system.

## Overview

The migration system automatically extracts special move properties from the existing `choices.rs` file and generates registration code for the new `MoveFactory` system. This ensures a seamless transition from the legacy hardcoded move data to the new flexible rustemon integration.

## Migration Results

The automated analysis extracted **428 moves with engine-specific data** from **885 total moves** in the system:

- **Draining moves**: 13 (e.g., Absorb, Drain Punch, Giga Drain)
- **Recoil moves**: 12 (e.g., Brave Bird, Flare Blitz, Head Smash)
- **Stat-boosting moves**: 90 (e.g., Agility, Swords Dance, Calm Mind)
- **Status moves**: 93 (e.g., Thunder Wave, Will-O-Wisp, Toxic)
- **Moves with secondary effects**: 197 (e.g., Thunderbolt, Ice Beam, Psychic)

## Using the Migration System

### 1. Run the Migration Analysis

```bash
# Build the migration tool
cargo build --release --features gen9,terastallization --no-default-features --bin migrate_moves

# Run the analysis
./target/release/migrate_moves
```

This will:
- Analyze all moves in the `MOVES` HashMap
- Extract moves with special engine-specific properties
- Generate a summary report
- Create `generated_move_registrations.rs` with registration code

### 2. Generated Registration Code

The migration tool generates registration code like this:

```rust
// Draining moves
self.service.register_engine_data(
    Choices::ABSORB,
    EngineDataBuilder::new()
        .drain(0.5)
        .flags(Flags {
            heal: true,
            protect: true,
            ..Default::default()
        })
        .build()
).await;

// Recoil moves
self.service.register_engine_data(
    Choices::BRAVEBIRD,
    EngineDataBuilder::new()
        .recoil(0.33)
        .flags(Flags {
            contact: true,
            protect: true,
            ..Default::default()
        })
        .build()
).await;
```

### 3. Integration with MoveFactory

The generated code can be integrated into the existing `MoveFactory` system:

1. Copy the generated registration calls into `MoveFactory::register_all_engine_data()`
2. Update the imports in the generated file to match your module structure
3. Test that all moves work correctly with the new system

## Engine-Specific Properties Extracted

The migration system extracts these special move properties:

### Damage Modifiers
- **`drain`**: Percentage of damage restored as HP (e.g., 0.5 = 50%)
- **`recoil`**: Percentage of damage taken as recoil (e.g., 0.33 = 33%)
- **`crash`**: Damage taken on miss (e.g., High Jump Kick)

### Status Effects
- **`status`**: Primary status conditions (burn, freeze, paralysis, poison, sleep)
- **`volatile_status`**: Temporary battle conditions (confusion, flinch, etc.)
- **`side_condition`**: Field effects (Stealth Rock, Spikes, etc.)

### Stat Changes
- **`boost`**: Stat modifications (attack +2, speed +1, etc.)
- **`heal`**: Fixed healing amounts

### Secondary Effects
- **`secondaries`**: Additional effects with probability (paralysis chance, stat drops, etc.)

### Move Flags
- **Contact flags**: `contact`, `bite`, `punch`, `slicing`
- **Protection flags**: `protect`, `reflectable`
- **Targeting flags**: `bullet`, `pulse`, `sound`
- **Mechanics flags**: `charge`, `recharge`, `heal`, `powder`, `drag`, `pivot`, `wind`

## Key Design Principles

### Separation of Concerns
- **Base move data** (power, accuracy, type, PP) comes from rustemon/PokeAPI
- **Engine-specific data** (drain, recoil, boosts) is registered separately
- **Battle mechanics** (spread damage, immunities) are handled in instruction generation

### What NOT to Register
Don't over-specify move data. Only register moves with **intrinsic special properties**:

✅ **Register these**:
- Absorb (drain: 0.5)
- Thunderbolt (secondaries: 10% paralysis)
- Agility (boost: speed +2)

❌ **Don't register these**:
- Earthquake (spread mechanics are positional, not intrinsic)
- Tackle (basic attack with no special properties)

### Testing the Migration

Run the built-in tests to verify migration completeness:

```bash
# Test the extraction logic
cargo test --lib test_extract_special_moves --release --features gen9,terastallization --no-default-features

# Test the summary generation
cargo test --lib test_generate_summary --release --features gen9,terastallization --no-default-features
```

## Benefits of This Approach

1. **Automated**: No manual copying of hundreds of move definitions
2. **Comprehensive**: Extracts all special properties systematically
3. **Organized**: Groups moves by category for easier review
4. **Testable**: Includes verification tests for completeness
5. **Maintainable**: Clear separation between data source and engine mechanics

## Future Maintenance

As new moves are added:
1. Add them to the legacy `MOVES` HashMap with proper engine-specific data
2. Re-run the migration tool to update registration code
3. The rustemon integration will automatically fetch base data from PokeAPI

This migration system provides a smooth transition path while maintaining the accuracy and reliability that downstream applications depend on.