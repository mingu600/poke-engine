use poke_engine::data::move_migration::MoveMigrationTool;

fn main() {
    println!("Starting move migration analysis...\n");
    
    // Generate summary report
    let summary = MoveMigrationTool::generate_summary_report();
    println!("{}", summary);
    
    // Extract special moves
    let special_moves = MoveMigrationTool::extract_special_moves();
    println!("\nFound {} moves with engine-specific data", special_moves.len());
    
    // Generate registration code
    let output_path = "generated_move_registrations.rs";
    match MoveMigrationTool::generate_registration_code(output_path) {
        Ok(_) => println!("\nSuccessfully generated registration code to: {}", output_path),
        Err(e) => eprintln!("\nError generating registration code: {}", e),
    }
    
    println!("\nMigration analysis complete!");
}