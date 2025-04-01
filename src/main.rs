use std::env;
use std::io::{self, Read};
use leetspeak::Level;
// Import the leetspeak library

fn translate_without_library(input: &str, max_replacements: usize) -> String {
    let mut replacements = 0;
    input
        .chars()
        .map(|c| {
            if replacements >= max_replacements {
                return c;
            }
            match c {
                'a' | 'A' => {
                    replacements += 1;
                    '4'
                }
                'e' | 'E' => {
                    replacements += 1;
                    '3'
                }
                'i' | 'I' => {
                    replacements += 1;
                    '1'
                }
                'o' | 'O' => {
                    replacements += 1;
                    '0'
                }
                's' | 'S' => {
                    replacements += 1;
                    '5'
                }
                't' | 'T' => {
                    replacements += 1;
                    '7'
                }
                _ => c,
            }
        })
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "--help" {
        println!("Usage: leetpw [--max <number>] [<text>]");
        println!("Options:");
        println!("  --max <number>   Set the maximum number of replacements (default: 2)");
        println!("  --help           Display this help message");
        println!("\nIf no text is provided, input will be read from standard input.");
        return;
    }

    let mut max_replacements = 2; // Default value for max_replacements
    let mut input_start_index = 1;

    if args.len() > 1 && args[1] == "--max" {
        if args.len() > 2 {
            max_replacements = args[2].parse::<usize>().unwrap_or(2); // Parse the value after --max
            input_start_index = 3; // Adjust input start index
        } else {
            eprintln!("Error: Missing value for --max");
            return;
        }
    }

    if args.len() > input_start_index {
        // If arguments are provided, use them as input
        let input = args[input_start_index..].join(" ");
        println!("{}", leetspeak::translate_with_level(&input, &Level::One));
        println!("{}", translate_without_library(&input, max_replacements)); // Use max_replacements
    } else {
        // Otherwise, read from standard input (pipe)
        let mut buffer = String::new();
        if io::stdin().read_to_string(&mut buffer).is_ok() {
            println!("{}", leetspeak::translate_with_level(buffer.trim(), &Level::One));
            println!("{}", translate_without_library(buffer.trim(), max_replacements)); // Use max_replacements
        } else {
            eprintln!("Failed to read input.");
        }
    }
}
