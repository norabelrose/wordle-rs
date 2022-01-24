use rayon::{prelude::*};
use super::{constraint::Constraint, word::Word};


// Type for representing the probability that we will win a game
#[derive(Debug, PartialEq, PartialOrd)]
pub struct WinProb {
    value: f64,
}

impl WinProb {
    // Consumes an iterator over WinProbs, returning the expectation / average
    pub fn expectation<T>(iter: T) -> Option<WinProb>
    where T: Iterator<Item = Self>
    {
        let mut count: usize = 0;
        let mut sum: f64 = 0.0;
        
        for prob in iter {
            sum += prob.value;
            count += 1;
        }
        if count == 0 {
            None
        } else {
            Some(WinProb { value: sum / count as f64 })
        }
    }
    // Because we generate the probability from a usize, the compiler will ensure this is never negative,
    // but it might end up being zero, which would yield a NaN.
    pub fn from_count(candidate_count: usize) -> WinProb {
        assert_ne!(candidate_count, 0);
        WinProb {
            value: 1.0 / (candidate_count as f64),
        }
    }
}
// We guarantee that WinProbs are finite, non-negative floats in [0, 1], so we can do a total ordering
impl Eq for WinProb {}
impl Ord for WinProb {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).expect("Somehow a WinProb struct was created with a NaN value")
    }
}


pub fn evaluate_move(guess: &Word, candidates: &Vec<Word>, depth: usize) -> WinProb {
    // Simulate the consequences of making this move across all possible candidates for the true word
    let value_iter = candidates.iter().map(|candidate| {
        // What feedback would I get if I made this guess, and the true word were this candidate?
        let constraint = Constraint::from_words(candidate, guess);

        // Given that feedback, what words would be left over?
        let next_pool_iter = candidates.iter().filter(|&w| constraint.test(w));

        // We're at maximum depth, so don't bother allocating a Vec to store the next pool, just
        // count how many words would be in the pool and return a probability based on that
        if depth == 0 {
            WinProb::from_count(next_pool_iter.count())
        }
        // We're not at maximum depth, so prepare to recurse
        else {
            let next_pool: Vec<_> = next_pool_iter.cloned().collect();
            let pool_size = next_pool.len();

            // If there's only one word left, we win, so this branch has a win probability of 1.
            // Also, if we've reached the maximum depth, we'll just return 1 / pool_size.
            if pool_size == 1 {
                WinProb::from_count(pool_size)
            } else {
                // Otherwise, we need to simulate the consequences of making this move on the next pool
                let branch_values = next_pool
                    .iter()
                    .map(|w| evaluate_move(w, &next_pool, depth - 1));
                
                WinProb::expectation(branch_values).unwrap()
            }
        }
    });
    WinProb::expectation(value_iter).unwrap()
}

pub fn best_move_recursive(candidates: &Vec<Word>, depth: usize) -> (&Word, WinProb) {
    candidates.par_iter().map(|candidate| {
        let value = evaluate_move(candidate, candidates, depth);
        (candidate, value)
    }).max().unwrap()
}
