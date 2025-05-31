# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

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

## Common Development Commands

### Building
```bash
# Focus exclusively on gen9 for now
cargo build --release --features gen9,terastallization --no-default-features
```

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
- `src/state.rs` - Core battle state representation and serialization
- `src/instruction.rs` - Battle instructions that modify state
- `src/search.rs` - Search algorithms (expectiminimax, iterative deepening)
- `src/mcts.rs` - Monte Carlo Tree Search implementation
- `src/choices.rs` - Move choices and move data structures
- `src/pokemon.rs` - Pokémon data structures
- `src/io.rs` - CLI interface and subcommands
- `src/genx/battle_environment.rs` - Battle environment for testing algorithms
- `data/` - Data folder containing important moves, pokedex, and random_team data information.

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

### Important Implementation Details

#### Move Choice Data Population
When creating Pokemon from packed team data, it's critical to populate the `choice` field in the `Move` struct with data from `MOVES` constant. Without this, `generate_instructions_from_move_pair` returns empty instruction lists, causing battles to stall with no progress.

```rust
// Correct implementation in create_pokemon():
let choice = if let Some(move_info) = crate::choices::MOVES.get(&move_choice) {
    move_info.clone()
} else {
    Default::default()
};
```

#### State Progress Verification
The engine tracks battle progress through:
- `State::battle_is_over()` - Returns 0.0 for ongoing, 1.0/-1.0 for winners
- HP changes in Pokemon teams
- Turn count vs maximum turn limit
- Instruction generation and application success

