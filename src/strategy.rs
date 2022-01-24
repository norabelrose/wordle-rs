use rayon::{prelude::*};
use std::collections::HashMap;
use super::{constraint::Constraint, word::Word};


pub enum EvalMode {
    ExpectedValue,
    Minimax,
}

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


pub fn greedy_move_evaluate(guess: &Word, candidates: &Vec<Word>, mode: &EvalMode) -> WinProb {
    // We need to somehow aggregate over all candidate words; either averaging or taking the min.
    let value_iter = candidates.iter().map(|candidate| {
        let constraint = Constraint::from_words(candidate, guess);

        // Compute what the size of the candidate pool would be at the next turn in the world where
        // the true word is `candidate` and we make the guess `guess`
        let candidate_count = candidates.iter().filter(|w| constraint.test(&w)).count();
        WinProb::from_count(candidate_count)
    });

    match mode {
        EvalMode::ExpectedValue => WinProb::expectation(value_iter),
        EvalMode::Minimax => value_iter.min(),
    }.unwrap()
}

pub fn greedy_best_move<'w>(candidates: &'w Vec<Word>, mode: &EvalMode) -> (&'w Word, WinProb) {
    candidates.par_iter().map(|candidate| {
        let value = greedy_move_evaluate(candidate, candidates, mode);
        (candidate, value)
    }).max().unwrap()
}
