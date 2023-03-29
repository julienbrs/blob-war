//! Alpha - Beta algorithm with Pass heuristic
use core::panic;
use std::fmt;

use super::Strategy;
use crate::configuration::{Configuration, Movement};
use crate::shmem::AtomicMove;

/// Anytime alpha beta algorithm.
/// Any time algorithms will compute until a deadline is hit and the process is killed.
/// They are therefore run in another process and communicate through shared memory.
/// This function is intended to be called from blobwar_iterative_deepening.
pub fn alpha_beta_pass_anytime(state: &Configuration) {
    let mut movement = AtomicMove::connect().expect("failed connecting to shmem");
    for depth in 5..100 {
        println!("{:}{:}{:}{:}", depth, depth, depth, depth);
        let chosen_movement = AlphaBetaPass(depth).compute_next_move(state);
        movement.store(chosen_movement);
    }
}

/// Alpha - Beta algorithm with given maximum number of recursions.
pub struct AlphaBetaPass(pub u8);

impl fmt::Display for AlphaBetaPass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Alpha - Beta PASS (max level: {})", self.0)
    }
}

impl Strategy for AlphaBetaPass {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        let (alpha, beta) = (i8::MIN, i8::MIN);
        let (movement, _) = AlphaBetaPass::alphabetaPass(self, state, self.0, alpha, beta, false);
        movement
    }
}

impl AlphaBetaPass {
    fn alphabetaPass(
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
            let (_, new_value) = self.alphabetaPass(
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

                // If a position seems good enough at depth 2, the computer can see what would happen if it
                // did not play (it passes) at depth k + 1. The idea is that in general this choice is bad.
                // Thus, if the result obtained remains good, the position must be really good, otherwise,
                // a normal search must be done.
                if depth == 3 {
                    // let best_state: &Configuration = state;
                    // Play a skip turn and then check if the value is still good
                    if best_value <= -7 {
                        match best_movement {
                            Some(m) => {
                                let best_state = state.play(&m);
                                let (_, val) = self.alphabetaPass(
                                    &best_state.skip_play(),
                                    depth - 1,
                                    alpha,
                                    beta,
                                    !opposing_player,
                                );
                                if val >= best_value && !opposing_player {
                                    print!("A");
                                    return (best_movement, best_value);
                                }
                            }
                            None => {
                                let best_state = state.skip_play();
                                let (_, val) = self.alphabetaPass(
                                    &best_state.skip_play(),
                                    depth - 1,
                                    alpha,
                                    beta,
                                    !opposing_player,
                                );
                                if val >= best_value && !opposing_player {
                                    print!("A");
                                    return (best_movement, best_value);
                                }
                            }
                        }
                    }

                    // let (_, val) = self.alphabetaPass(
                    //     &best_state.skip_play(),
                    //     depth - 1,
                    //     alpha,
                    //     beta,
                    //     !opposing_player,
                    // );
                    // if val >= best_value {
                    //     return (best_movement, best_value);
                    // }
                }
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
            let (_, val) = AlphaBetaPass::alphabetaPass(self, state, depth - 1, alpha, beta, true);
            (None, val)
        } else {
            return (best_movement, -best_value);
        }
    }
}
