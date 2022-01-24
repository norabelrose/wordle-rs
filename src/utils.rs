use std::{fs::File, io::{BufReader, prelude::*}};
use super::word::Word;


pub fn load_word_list(filename: &str, word_len: usize) -> std::io::Result<Vec<Word>> {
    // We read in the word list as a vector of ASCII bytes instead of dealing
    // with Unicode strings, since we know that the words are all ASCII.
    let reader = BufReader::new(File::open(filename)?);
    Ok(
        reader
        .split(b'\n')
        .map(|w| w.unwrap())
        .filter(|w| w.len() == word_len)
        .map(|w| Word::new(w))
        .collect()
    )
}