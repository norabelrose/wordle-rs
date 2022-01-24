mod constraint;
mod strategy;
mod utils;
mod word;
use clap::{Parser};
use utils::*;
// use rand::{Rng, rngs::StdRng};
use std::io::{prelude::*};
use constraint::Constraint;
use strategy::*;


// #[derive(ArgEnum, Clone)]
// enum PlayStrategy {
//     Random,
//     Greedy
// }
// 
// 
// #[derive(Subcommand)]
// enum Command {
//     Play {
//         strategy: PlayStrategy,
//         word_list: String,
//     },
//     Generate {
//         word_list: String,
//     }
// }


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
    //let mut rng = StdRng::from_entropy();

    for i in 0..args.turns {
        println!("Turn {}; {} candidates", i + 1, words.len());

        // Keep looping until the user enters a valid input
        loop {
            let (guess, value) = greedy_best_move(&words, &EvalMode::ExpectedValue);
            //let idx = rng.gen_range(0..words.len());
            // let guess = &words[idx];
            println!("Guess: {}; value: {:?}", guess.as_str(), value);

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
