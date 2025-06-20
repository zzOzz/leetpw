use std::io::{self, Read};
use std::process::Command;
use atty::Stream;
use clap::{Parser, ValueEnum};
use clap_complete::{generate, shells::{Fish, Zsh}};

#[derive(Debug, Copy, Clone, ValueEnum)]
enum Shell {
    Fish,
    Zsh,
}

#[derive(Parser, Debug)]
#[command(
    name = "leetpw",
    about = "A tool for generating leetspeak passwords",
    long_about = "If no text is provided, input will be read from standard input or a default command."
)]
struct Cli {
    /// Set the maximum number of replacements
    #[arg(long, default_value = "2")]
    max: usize,

    /// Set the number of words to generate
    #[arg(long, default_value = "4")]
    words: usize,

    /// Output shell completion script
    #[arg(long, value_enum)]
    completion: Option<Shell>,

    /// Text to transform (optional)
    text: Vec<String>,
}

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

fn print_completions(shell: Shell) {
    use clap::CommandFactory;
    let mut cmd = Cli::command();
    match shell {
        Shell::Fish => {
            generate(Fish, &mut cmd, "leetpw", &mut io::stdout());
        }
        Shell::Zsh => {
            generate(Zsh, &mut cmd, "leetpw", &mut io::stdout());
        }
    }
}

fn main() {
    let cli = Cli::parse();

    if let Some(shell) = cli.completion {
        print_completions(shell);
        return;
    }

    let input = if cli.text.is_empty() {
        if atty::is(Stream::Stdin) {
            // If no pipe input, use the result of the default command
            let command = format!("diceware -w fr -n {} -d ' '", cli.words);
            let output = Command::new("sh")
                .arg("-c")
                .arg(&command)
                .output()
                .expect("Failed to execute default command. please install diceware and french dictionnary\n curl -s https://raw.githubusercontent.com/mbelivo/diceware-wordlists-fr/refs/heads/master/wordlist_fr_5d.txt -o /usr/lib/python3/dist-packages/diceware/wordlists/wordlist_fr.txt");

            if output.status.success() {
                String::from_utf8_lossy(&output.stdout).to_string()
            } else {
                eprintln!("Failed to get input from default command.");
                return;
            }
        } else {
            // Check for standard input (pipe)
            let mut buffer = String::new();
            if io::stdin().read_to_string(&mut buffer).is_ok() && !buffer.trim().is_empty() {
                buffer
            } else {
                return;
            }
        }
    } else {
        cli.text.join(" ")
    };

    println!("{}", translate_without_library(input.trim(), cli.max));
}
