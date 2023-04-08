//! Alpha - Beta algorithm
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
        let (alpha, beta) = (i8::MIN, i8::MIN);
        let (movement, _) = AlphaBeta::alphabeta(self, state, self.0, alpha, beta, false);
        movement
    }
}

impl AlphaBeta {
    fn alphabeta(
        &mut self,
        state: &Configuration,
        depth: u8,
        mut alpha: i8,
        mut beta: i8,
        opposing_player: bool,
    ) -> (Option<Movement>, i8) {
        if depth == 0 {
            return (None, state.value());
        }

        let mut best_movement: Option<Movement> = None;
        let mut best_value: i8 = i8::MIN;

        for movement in state.movements() {
            let (_, new_value) = self.alphabeta(
                &state.play(&movement),
                depth - 1,
                alpha,
                beta,
                !opposing_player,
            );

            // Compute the evaluation of the current move and store it along with the movement in a tuple to avoid redundant computation.
            let (value, movement) = (new_value, Some(movement));

            if value > best_value {
                best_value = value;
                best_movement = movement;
            }

            // Update alpha and beta
            if opposing_player && value > beta {
                beta = value;
            } else if !opposing_player && value > alpha {
                alpha = value;
            }

            // Alpha - Beta cut off
            if opposing_player && -beta <= alpha {
                return (best_movement, -best_value);
            } else if !opposing_player && -alpha <= beta {
                return (best_movement, -best_value);
            }
        }

        if best_movement.is_none() {
            let (_, val) = AlphaBeta::alphabeta(self, state, depth - 1, alpha, beta, true);
            (None, val)
        } else {
            return (best_movement, -best_value);
        }
    }
}
