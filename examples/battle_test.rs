use poke_engine::engine::battle_environment::{run_random_battle, MctsPlayer, RandomPlayer};
use std::fs;

fn main() {
    // Load the JSON data
    let random_teams_json =
        fs::read_to_string("data/random_teams.json").expect("Failed to read random_teams.json");
    let pokedex_json =
        fs::read_to_string("data/pokedex.json").expect("Failed to read pokedex.json");
    let movedex_json = fs::read_to_string("data/moves.json").expect("Failed to read moves.json");

    // Create players - demonstrate MCTS player
    let player_one = Box::new(MctsPlayer::new("MCTSBot".to_string(), 200)); // 200ms search time
    let player_two = Box::new(RandomPlayer::new("RandomBot".to_string()));

    // Run a battle
    let result = run_random_battle(
        &random_teams_json,
        &pokedex_json,
        &movedex_json,
        player_one,
        player_two,
        100,  // max turns
        true, // verbose
    );

    // Print summary
    println!("\n=== Battle Summary ===");
    match result.winner {
        Some(poke_engine::state::SideReference::SideOne) => println!("Winner: Player 1"),
        Some(poke_engine::state::SideReference::SideTwo) => println!("Winner: Player 2"),
        None => println!("Draw"),
    }
    println!("Total turns: {}", result.turn_count);

    // Print some turn details
    println!("\n=== Turn History Sample ===");
    for turn in result.turn_history.iter().take(3) {
        println!("Turn {}: ", turn.turn_number);
        println!(
            "  Side 1 chose: {:?}",
            turn.side_one_choice.to_string(&turn.state_before.side_one)
        );
        println!(
            "  Side 2 chose: {:?}",
            turn.side_two_choice.to_string(&turn.state_before.side_two)
        );
        println!(
            "  {} instruction sets generated",
            turn.instructions_generated.len()
        );
    }
}
