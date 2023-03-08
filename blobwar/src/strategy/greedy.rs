//! Dumb greedy algorithm.
use super::Strategy;
use crate::configuration::{Configuration, Movement};
use rayon::prelude::*;
use std::fmt;

/// Dumb algorithm.
/// Amongst all possible movements return the one which yields the configuration with the best
/// immediate value.
pub struct Greedy();

impl fmt::Display for Greedy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Greedy")
    }
}

impl Strategy for Greedy {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        state.movements().max_by_key(|&movement| state.play(&movement).value())
    }
}

impl Greedy {
    fn greedy_par(&mut self, state: &Configuration) -> Option<Movement> {
        state.movements().par_bridge().max_by_key(|&movement| state.play(&movement).value())
    }
}