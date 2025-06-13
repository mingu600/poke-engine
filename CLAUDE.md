# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 🚨 CURRENT PROJECT: Multi-Format Pokemon Battle Engine Overhaul 🚨

### Project Overview
Our objectives are twofold.
1. We are implementing a major architectural overhaul to support multiple battle formats (singles, doubles, VGC).
2. We want to utilize rustemon (Rust wrapper for PokeAPI) as much as possible when referencing data. If it is more efficient to create our own custom structs like the way the current engine works, then it would be nice to base the struct creation on this well-maintained, pokemon data source. This is a critical project as downstream applications depend on this engine's accuracy and reliability.

Rustemon and PokeAPI source code is available to read in the parent folder. Refer to me as Mingu.

Before working, you must build to clear errors:
```bash
# Focus exclusively on gen9 for now
cargo build --release --features gen9,terastallization --no-default-features
```

### 🎯 CURRENT STATUS: Phase 2 Week 9-10 Complete - Ready for Week 11-12

**Phase 1: Data Layer Foundation** ✅ COMPLETED
- ✅ Rustemon dependency integration (v4.2.0 with async support)
- ✅ Data client wrapper (`src/data/rustemon_client.rs`)
- ✅ Engine data structures (`src/data/types.rs`)
- ✅ Type conversion utilities (`src/data/conversion.rs`)
- ✅ Integration testing and validation
- 🔄 Build-time data generation pipeline (deferred to optimization phase)
- 🔄 Static data file generation (deferred - currently using live HTTP requests)

**Phase 2: Multi-Format Architecture** ✅ COMPLETED

**Week 7-8: Battle Format System** ✅ COMPLETED
- ✅ Complete battle format system (`src/battle_format.rs`)
- ✅ Unified format registry (`src/format_registry.rs`) with 8 predefined formats
- ✅ Engine-focused clause system (Sleep, OHKO, Evasion, Species, Item, Freeze)
- ✅ Format enforcement system (`src/format_enforcement.rs`)
- ✅ Format initialization and detection (`src/format_init.rs`)
- ✅ Move targeting system supporting all 16 rustemon/PokeAPI targets
- ✅ Comprehensive test suite (43 tests total)

**Week 9-10: Battle State Overhaul** ✅ COMPLETED
- ✅ Battle state overhaul for multiple active Pokemon per side
- ✅ Enhanced BattleSide for variable active Pokemon counts  
- ✅ Position-based targeting system integration
- ✅ Format-aware state construction and management
- ✅ Backward compatibility maintained for existing engine

See `docs/HLD_Multi_Format_Engine_Overhaul.md` for complete roadmap. 

## Working with Python

Whenever working in Python, always run commands in conda env py312. Whenever you need to install a package in Python, always do it in the environment as well.

## Design Philosophy Principles

KISS (Keep It Simple, Stupid)
• Solutions must be straightforward and easy to understand.
• Avoid over-engineering or unnecessary abstraction.
• Prioritise code readability and maintainability.

YAGNI (You Aren’t Gonna Need It)
• Do not add speculative features or future-proofing unless explicitly required.
• Focus only on immediate requirements and deliverables.
• Minimise code bloat and long-term technical debt. 

Never make code changes that affect the design without first discussing the design and getting a confirmation to proceed.
Never include references to AI or Claude in commit messages.

 Communication Style:

    Skip affirmations and compliments. No “great question!” or “you’re absolutely right!” - just respond directly

    Challenge flawed ideas openly when you spot issues

    Ask clarifying questions whenever my request is ambiguous or unclear

## Development Workflow Protocol

### Phase Integration Testing Workflow 🚨 CRITICAL
When working on ANY component that might affect completed phases:

1. **Pre-Development Check:**
   ```bash
   # ALWAYS run this before starting work
   cargo test --test test_phase2_integration --release --features gen9,terastallization --no-default-features
   ```
   - All 45 tests MUST pass before you start

2. **During Development:**
   - Make incremental changes
   - Test frequently during development
   - If unsure about impact, run phase integration tests

3. **Post-Development Validation:**
   ```bash
   # ALWAYS run this after making changes
   cargo test --test test_phase2_integration --release --features gen9,terastallization --no-default-features
   ```
   - All 45 tests MUST still pass
   - If ANY test fails, you MUST discuss why before proceeding

4. **Failure Protocol:**
   - **STOP immediately** if phase integration tests fail
   - **Analyze** which specific test(s) are failing and why
   - **Discuss** the failure with Mingu before making any "fixes"
   - **Document** why the failure occurred and how it was resolved
   - **Never** comment out or skip failing tests without explicit approval

## Common Development Commands

### Building
```bash
# Focus exclusively on gen9 for now
cargo build --release --features gen9,terastallization --no-default-features
```

### Testing

#### Phase Integration Testing
Before making any changes to Phase 2 components, you MUST run the Phase 2 integration tests to ensure existing functionality is preserved:

```bash
# Run Phase 2 integration tests (45 tests covering all Phase 2 components)
cargo test --test test_phase2_integration --release --features gen9,terastallization --no-default-features
```

**Critical Testing Protocol:**
1. **BEFORE making any changes:** Run Phase 2 integration tests and ensure all pass
2. **AFTER implementing changes:** Re-run Phase 2 integration tests
3. **IF tests fail:** You MUST discuss why they're failing before proceeding with changes
4. **NEVER commit code** that breaks existing Phase 2 integration tests without explicit discussion

#### Full Test Suite
```bash
# Run all tests (may have errors from incomplete components)
cargo test --release --features gen9,terastallization --no-default-features

# Run specific test file
cargo test --test test_rustemon_integration --release --features gen9,terastallization --no-default-features
```

#### Test File Organization
- `tests/test_phase2_integration.rs` - **CRITICAL**: Complete Phase 2 validation (45 tests)
- `tests/test_rustemon_integration.rs` - Data layer validation tests
- `tests/test_*.rs` - Legacy tests for other components (may have errors from incomplete system)

## Architecture Overview

States should never be assumed to be Showdown packed format, or any Showdown related structure. Always assume internal engine constructs.

### Generation-Specific Code Structure
The engine uses Rust features to conditionally compile code for different Pokémon generations:
- `src/gen1/` - Generation 1 specific implementation
- `src/gen2/` - Generation 2 specific implementation  
- `src/gen3/` - Generation 3 specific implementation
- `src/genx/` - Default implementation for generations 4-9

Each generation module contains:
- `abilities.rs` - Pokémon abilities
- `base_stats.rs` - Base statistics for Pokémon
- `choice_effects.rs` - Move and choice effects
- `damage_calc.rs` - Damage calculation logic
- `evaluate.rs` - State evaluation functions
- `generate_instructions.rs` - Instruction generation from moves
- `items.rs` - Item definitions and effects
- `state.rs` - Battle state representation

### Core Components

#### Legacy Components (Existing)
- `src/state.rs` - Core battle state representation and serialization
- `src/instruction.rs` - Battle instructions that modify state
- `src/search.rs` - Search algorithms (expectiminimax, iterative deepening)
- `src/mcts.rs` - Monte Carlo Tree Search implementation
- `src/choices.rs` - Move choices and move data structures
- `src/pokemon.rs` - Pokémon data structures
- `src/io.rs` - CLI interface and subcommands
- `src/genx/battle_environment.rs` - Battle environment for testing algorithms
- `data/` - Data folder containing important moves, pokedex, and random_team data information

#### New Data Layer (Phase 1 Complete)
- `src/data/rustemon_client.rs` - Rustemon/PokeAPI HTTP client wrapper
- `src/data/types.rs` - Engine-optimized data structures that compose rustemon data
- `src/data/conversion.rs` - Type conversion utilities between rustemon and engine types
- `tests/test_rustemon_integration.rs` - Data layer validation tests

### State Representation
The battle state is serialized as a string format. State deserialization is documented in `State::deserialize` in `src/state.rs`. This is the source of truth for state string parsing.

### Python Bindings
Python bindings are in the `poke-engine-py/` directory using maturin for building.

## Feature Flags
The codebase uses feature flags to enable generation-specific code:
- `gen1` through `gen9` - Enable specific generation
- `terastallization` - Enable terastallization mechanics (requires gen9)
- `remove_low_chance_instructions` - Optimization flag

Only one generation feature should be enabled at a time.

## Battle Environment

### Overview
The battle environment (`src/genx/battle_environment.rs`) provides a comprehensive testing framework for Pokemon battle algorithms. It allows different AI players to compete against each other with full observability and parallel execution support.

### Player Types
The system supports multiple player implementations:

- **RandomPlayer** - Chooses moves randomly from available options
- **FirstMovePlayer** - Always selects the first available move
- **DamageMaximizer** - Picks moves based on base power to maximize damage
- **MctsPlayer** - Uses Monte Carlo Tree Search with configurable search time

### Battle CLI Command
```bash
# Basic battle between two players
./poke-engine battle -p mcts -q random

# Verbose battle with detailed state logging to file
./poke-engine battle -p damage -q mcts -v -l battle.log

# Parallel execution for performance testing
./poke-engine battle -p mcts -q random -r 100 -j 8

# Asymmetric MCTS search times for handicapping
./poke-engine battle -p mcts -q mcts --p1-mcts-time 50 --p2-mcts-time 500
```

### CLI Options
- `-p/--player-one` - Player 1 type (random, firstmove, damage, mcts)
- `-q/--player-two` - Player 2 type (random, firstmove, damage, mcts)
- `-m/--max-turns` - Maximum battle turns (default: 100)
- `-r/--runs` - Number of battles to run (default: 1)
- `-v/--verbose` - Enable detailed turn-by-turn output
- `-l/--log-file` - Write verbose output to file instead of terminal
- `-j/--threads` - Number of parallel threads for multiple battles
- `-t/--mcts-time` - Default MCTS search time in milliseconds
- `--p1-mcts-time` - Player 1 specific MCTS search time
- `--p2-mcts-time` - Player 2 specific MCTS search time

### Verbose Output Features
When verbose mode is enabled, the system outputs:
- Complete battle state using `State::pprint()`
- Move selections for each turn
- Pokemon stats, abilities, items, and conditions
- Weather, terrain, and battlefield conditions
- Available move choices for each side

### Parallel Execution
The battle environment supports parallel execution for performance testing:
- Multiple battles run simultaneously across threads
- Each thread manages independent battle instances
- Results are aggregated with timing statistics
- Significant speedup for bulk algorithm testing

### Architecture Notes
- **Player Trait**: All players implement `Player: Send + Sync + 'static`
- **State Management**: Each battle gets a fresh state from random teams
- **Move Selection**: Players choose from available `MoveChoice` options
- **Instruction Processing**: Moves generate probabilistic `StateInstructions`
- **Result Tracking**: Complete turn history and final outcomes recorded

### Testing and Examples
- Unit tests in `tests/test_battle_environment.rs`
- Example usage in `examples/battle_test.rs`
- Performance benchmarks via parallel execution
- Algorithm comparison through win/loss statistics
