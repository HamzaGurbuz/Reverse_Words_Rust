use std::io::{self, Write};

fn main() {
    // Prompt the user for input
    print!("Enter a sentence: ");
    io::stdout().flush().unwrap(); // Flush the prompt to ensure it prints

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // Split the input into words, reverse the words and their order, and join them back into a string
    let reversed: String = input
        .trim()
        .split_whitespace()
        .rev()
        .map(|s| s.chars().rev().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ");

    // Print the reversed sentence
    println!("Reversed sentence: {}", reversed);
}
