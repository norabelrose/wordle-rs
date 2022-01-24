use super::word::Word;


type AsciiChar = u8;

#[derive(Eq, Hash, PartialEq)]
pub struct Constraint {
    forbidden: u32,
    required: u32,
    
    positions: Vec<i8>
}

impl Constraint {
    pub fn from_colors(colors: &[AsciiChar], guess: &[AsciiChar]) -> Result<Constraint, std::io::Error> {
        let num_colors = colors.len();
        if num_colors != guess.len() {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Colors and guess must be the same length"));
        }

        let mut forbidden = 0;
        let mut required = 0;
        let mut positions = vec![0i8; num_colors];

        for (i, (color, g)) in colors.iter().cloned().zip(guess.iter().cloned()).enumerate() {
            let bit = 1 << (g - b'a');
            match color {
                b'b' => {
                    forbidden |= bit;
                },
                b'g' => {
                    positions[i] = g as i8;
                    required |= bit;
                },
                b'y' => {
                    positions[i] = -(g as i8);
                    required |= bit;
                },
                _ => {
                    return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid color"));
                }
            }
        }

        // Remove forbidden letters that are also required
        forbidden &= !required;

        Ok(Constraint {
            forbidden,
            required,
            positions
        })
    }

    pub fn from_words(true_word: &Word, guess: &Word) -> Constraint {
        // We assert here instead of returning an error because this should never happen if there aren't any bugs
        let word_len = true_word.bytes.len();
        assert_eq!(word_len, guess.bytes.len());

        // First find the intersection of the true word and the guess words' bitfields,
        // so we can figure out if we need to yield a "yellow" or a "black" color where
        // the guess word doesn't match up with the true one
        let required = true_word.bitfield & guess.bitfield;

        let mut positions = vec![0i8; word_len];
        for (i, (&true_char, &guess_char)) in true_word.bytes.iter().zip(guess.bytes.iter()).enumerate() {
            // The guess word and the true word match up at this position, so this
            // so we need to yield a "green" color
            if true_char == guess_char {
                positions[i] = true_char as i8;
            }
            // The guess word and the true word don't match up at this position,
            // but this letter does exist somewhere else in the true word, so we
            // need to yield a "yellow" color
            else if required & (1 << (guess_char - b'a')) != 0 {
                positions[i] = -(guess_char as i8);
            }
        }

        Constraint {
            forbidden: !true_word.bitfield & guess.bitfield,
            required: required,
            positions
        }
    }

    pub fn test(&self, guess: &Word) -> bool {
        for (&pos, &letter) in self.positions.iter().zip(guess.bytes.iter()) {
            if pos == 0 {
                continue;
            }

            // Positive positions are equality constraints (green letters)
            if pos > 0 {
                if letter != pos as AsciiChar {
                    return false;
                }
            }
            // Negative positions are inequality constraints (yellow letters)
            else {
                if letter == -pos as AsciiChar {
                    return false;
                }
            }
        }

        (self.forbidden & guess.bitfield) == 0 && (self.required & guess.bitfield) == self.required
    }
}


#[cfg(test)]
mod tests {
    use crate::utils::load_word_list;
    use super::*;
    
    #[test]
    fn test_correctness() {
        let words = load_word_list("wordlegame-org_words.txt", 5).unwrap();

        for true_word in words.iter() {
            for guess_word in words.iter() {
                let constraint = Constraint::from_words(&true_word, &guess_word);
                let are_equal = true_word == guess_word;

                assert_eq!(constraint.test(&guess_word), are_equal);
                assert!(constraint.test(&true_word));
            }
        }
    }
}