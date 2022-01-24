mod constraint;
mod strategy;
mod utils;
mod word;
use clap::{Parser};
use utils::*;
use std::io::prelude::*;
use constraint::Constraint;
use strategy::*;


#[derive(Parser, Debug)]
#[clap(author = "Nora Belrose", about = "A bot that plays Wordle")]
struct Args {
    #[clap(long, default_value = "6")]
    turns: usize,

    #[clap(long = "word-length", default_value = "5")]
    word_length: usize,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let mut words = load_word_list("wordlegame-org_words.txt", args.word_length)?;
    
    println!("Possible starting words: {}", words.len());
    println!("Number of turns: {}", args.turns);

    let mut input_buf = String::new();

    for i in 0..args.turns {
        let num_words = words.len();
        println!("Turn {}; {} candidates", i + 1, num_words);

        let turns_left = args.turns - (i + 1);
        let depth = if num_words < 100 {
            turns_left
        } else if num_words < 500 {
            1
        } else {
            0
        };
        let (guess, value) = best_move_recursive(&words, depth);
        println!("Guess: {}; value: {:?}", guess.as_str(), value);

        // Keep looping until the user enters a valid input
        loop {
            print!("Wordle colors: ");
            std::io::stdout().flush()?;
            input_buf.clear();
            std::io::stdin().read_line(&mut input_buf)?;

            let color_bytes: Vec<_> = input_buf.trim().bytes().collect();
            if color_bytes.iter().all(|c| *c == b'g') {
                println!("You win!");
                return Ok(());
            }

            match Constraint::from_colors(&color_bytes, &guess.bytes) {
                Ok(constraint) => {
                    words = words.into_iter().filter(|w| constraint.test(&w)).collect();

                    if words.is_empty() {
                        println!("You lose!");
                        return Ok(());
                    }
                    break;
                },
                Err(_) => {
                    //words.swap_remove(idx);
                }
            }
        }
    }

    Ok(())
}
