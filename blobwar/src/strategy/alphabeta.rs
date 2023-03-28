//! Alpha - Beta algorithm.
use std::fmt;

use super::Strategy;
use crate::configuration::{Configuration, Movement};
use crate::shmem::AtomicMove;

/// Anytime alpha beta algorithm.
/// Any time algorithms will compute until a deadline is hit and the process is killed.
/// They are therefore run in another process and communicate through shared memory.
/// This function is intended to be called from blobwar_iterative_deepening.
pub fn alpha_beta_anytime(state: &Configuration) {
    let mut movement = AtomicMove::connect().expect("failed connecting to shmem");
    for depth in 1..100 {
        let chosen_movement = AlphaBeta(depth).compute_next_move(state);
        movement.store(chosen_movement);
    }
}

/// Alpha - Beta algorithm with given maximum number of recursions.
pub struct AlphaBeta(pub u8);

impl fmt::Display for AlphaBeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Alpha - Beta (max level: {})", self.0)
    }
}

impl Strategy for AlphaBeta {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        let (mut alpha, mut beta) = (i8::MIN, i8::MAX);
        let m = self.elagage_no_par(state, self.0, &alpha, &beta, false).0;
        match m {
            Some(m) => {
                println!("{:?}", m);
                None
            }
            None => None,
        }
    }
}

impl AlphaBeta {
    fn elagage_no_par(
        &mut self,
        state: &Configuration,
        depth: u8,
        alpha: &i8,
        beta: &i8,
        minimize_player: bool,
    ) -> (Option<Movement>, i8) {
        println!("alpha {:} beta {:}", alpha, beta);
        if depth == 0 {
            return (None, state.value());
        }
        let mut a = *alpha;
        let mut b = *beta;
        if minimize_player {
            let mut v = i8::MAX;
            for movement in state.movements() {
                v = std::cmp::min(
                    v,
                    self.elagage_no_par(
                        &state.play(&movement),
                        depth - 1,
                        &a,
                        &b,
                        !minimize_player,
                    )
                    .1,
                );
                if v >= b {
                    return (Some(movement), v);
                }
                b = std::cmp::min(b, v);
            }
        } else {
            let mut v = i8::MIN;
            for movement in state.movements() {
                println!("here");
                v = std::cmp::max(
                    v,
                    self.elagage_no_par(
                        &state.play(&movement),
                        depth - 1,
                        &a,
                        &b,
                        !minimize_player,
                    )
                    .1,
                );
                if a >= v {
                    return (Some(movement), v);
                }
                println!("alpha treated {:}", a);
                a = std::cmp::max(a, v);
            }
        }
        return (None, state.value());
    }
}
