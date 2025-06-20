use std::env;
use std::io::{self, Read};
use std::process::Command;
use atty::Stream;
// use leetspeak::Level;
// Import the leetspeak library

fn translate_without_library(input: &str, max_replacements: usize) -> String {
    input
        .split_whitespace()
        .map(|word| {
            let mut replacements = 0;
            let transformed_word: String = word
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
                .collect();
            transformed_word
        })
        .collect::<Vec<String>>()
        .join("-")
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "--help" {
        println!("Usage: leetpw [--max <number>] [--words <number>] [<text>]");
        println!("Options:");
        println!("  --max <number>    Set the maximum number of replacements (default: 2)");
        println!("  --words <number>  Set the number of words to generate (default: 4)");
        println!("  --help            Display this help message");
        println!("\nIf no text is provided, input will be read from standard input or a default command.");
        return;
    }

    let mut max_replacements = 2; // Default value for max_replacements
    let mut num_words = 4; // Default value for number of words
    let mut input_start_index = 1;
    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "--max" => {
                if i + 1 < args.len() {
                    max_replacements = args[i + 1].parse::<usize>().unwrap_or(2);
                    i += 2;
                    input_start_index = i;
                } else {
                    eprintln!("Error: Missing value for --max");
                    return;
                }
            }
            "--words" => {
                if i + 1 < args.len() {
                    num_words = args[i + 1].parse::<usize>().unwrap_or(4);
                    i += 2;
                    input_start_index = i;
                } else {
                    eprintln!("Error: Missing value for --words");
                    return;
                }
            }
            _ => {
                input_start_index = i;
                break;
            }
        }
    }

    if args.len() > input_start_index {
        // If arguments are provided, use them as input
        let input = args[input_start_index..].join(" ");
        println!("{}", translate_without_library(&input, max_replacements));
    } else {
        if atty::is(Stream::Stdin) {
            // If no pipe input, use the result of the default command
            let command = format!("diceware -w fr -n {} -d ' '", num_words);
            let output = Command::new("sh")
                .arg("-c")
                .arg(&command)
                .output()
                .expect("Failed to execute default command. please install diceware and french dictionnary\n curl -s https://raw.githubusercontent.com/mbelivo/diceware-wordlists-fr/refs/heads/master/wordlist_fr_5d.txt -o /usr/lib/python3/dist-packages/diceware/wordlists/wordlist_fr.txt");

            if output.status.success() {
                let default_input = String::from_utf8_lossy(&output.stdout);
                println!("{}", translate_without_library(default_input.trim(), max_replacements));
                return;
            } else {
                eprintln!("Failed to get input from default command.");
            }
        }
        // Otherwise, check for standard input (pipe)
        let mut buffer = String::new();
        if io::stdin().read_to_string(&mut buffer).is_ok() && !buffer.trim().is_empty() {
            println!("{}", translate_without_library(buffer.trim(), max_replacements));
        }
    }
}
