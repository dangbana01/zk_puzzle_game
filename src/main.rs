use anyhow::Result;
use std::io::{self, Write};

fn verify_puzzle_solution(puzzle: &str, solution: &str) -> bool {
    match puzzle {
        "rust" => solution == "zk",
        "succinct" => solution == "rocks",
        _ => false,
    }
}

fn main() -> Result<()> {
    println!("🧩 Welcome to the zk Puzzle Game with Levels!\n");

    let puzzles = vec!["rust", "succinct"];
    let mut score = 0;

    for puzzle in &puzzles {
        println!("Puzzle: {}", puzzle);
        print!("Enter your secret solution: ");
        io::stdout().flush()?;

        let mut solution = String::new();
        io::stdin().read_line(&mut solution)?;
        let solution = solution.trim();

        if verify_puzzle_solution(puzzle, solution) {
            println!("✅ Correct!\n");
            score += 1;
        } else {
            println!("❌ Incorrect. Try next puzzle.\n");
        }
    }

    println!("Game Over! Your score: {}/{}", score, puzzles.len());
    Ok(())
}

