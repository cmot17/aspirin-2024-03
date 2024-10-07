//! This file implements a simple guessing game, useful for learning the Rust programming language.
#![warn(missing_docs)]

use rand::Rng;
use std::cmp::Ordering;
use std::io;

/// Asks the user for a guess, reads an i32 from stdin or panic if the user's input is not a valid i32.
fn get_input() -> i32 {
    println!("Please input your guess");

    let mut input = String::new();
    // Read a line from stdin
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Match the trimmed line into an i32 or panic
    match input.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Invalid entry."),
    }
}

/// The main function that contains the overall control flow and checks the user's guesses.
fn main() {
    println!("Guess the number!");

    // Generate the random secret number
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let guess = get_input();
        print!("You guessed: {}. ", guess);

        // Compare the user's guess to the actual secret number
        match secret_number.cmp(&guess) {
            Ordering::Equal => {
                println!("That is correct!");
                break;
            }
            Ordering::Greater => println!("You're guess is too low."),
            Ordering::Less => println!("You're guess is too high."),
        }
    }
}
