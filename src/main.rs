use anyhow::Result;
use std::io::{self, Write};

// Mock verifier function that simulates ZK proof verification
fn verify_puzzle_solution(puzzle: &str, solution: &str) -> bool {
    // Simulate a zero-knowledge proof by checking if the solution hashes to a known value
    // For now, use a simple placeholder: puzzle + solution must be equal to "rustzk"
    puzzle == "rust" && solution == "zk"
}

fn main() -> Result<()> {
    println!("\n🧩 Welcome to the zk Puzzle Game!");
    println!("Prove your knowledge without revealing the full answer.\n");

    let puzzle = "rust"; // In real ZK, this would be some public input

    print!("Enter your secret solution: ");
    io::stdout().flush()?;

    let mut solution = String::new();
    io::stdin().read_line(&mut solution)?;
    let solution = solution.trim();

    if verify_puzzle_solution(puzzle, solution) {
        println!("✅ Zero-knowledge proof verified! You solved the puzzle!");
    } else {
        println!("❌ Verification failed. Try again.");
    }

    Ok(())
}
