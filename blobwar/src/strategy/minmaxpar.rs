//! Implementation of the min max algorithm.
use super::{BenchmarkUnitaire, Strategy};
use crate::configuration::{ Configuration, Movement };
use crate::shmem::AtomicMove;
use rayon::prelude::*;
use std::fmt;

/// Min-Max algorithm with a given recursion depth.ch
pub struct MinMaxPar(pub u8);

impl BenchmarkUnitaire for MinMaxPar {
    fn new(depth: u8) -> Self {
        return MinMaxPar(depth);
    }
}

impl Strategy for MinMaxPar {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {

        let (movement, _) = MinMaxPar::min_max_par(self, state, self.0, false);
        movement
    }
}
impl MinMaxPar {
    /// Parallel min-max algorithm
    fn min_max_par(&self, state: &Configuration, depth: u8, opposing_player:bool) -> (Option<Movement>, i8){
        if depth == 0 {
            // state.value() indicates the loss of the current player
            return (None, state.value());
        }

        let value = i8::MIN;
        let best_movement: Option<Movement> = None;

        // Minimize loss for our player

        let values= state.movements().par_bridge().map(|movement| {
            // We play the current move

            let (_, new_state_val) = self.min_max_par(
                &state.play(&movement),
                depth - 1,
                !opposing_player
            );
            // If opposing player, we want to maximize the loss
            // => minimizing the gain
            //if opposing_player {new_state_val = -new_state_val;}
            (Some(movement), new_state_val)
        });

        let result;
        if opposing_player {
            result = values.max_by_key(|(_, value)| *value);
        }else{
            result = values.min_by_key(|(_, value)| *value);
        }
        
        match result {
            Some(some) => some,
            None => {if opposing_player {(best_movement, -value)} else {(best_movement, value)}},
        }
    }
}
impl fmt::Display for MinMaxPar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Min - Max Par (max level: {})", self.0)
    }
}

/// Anytime min max parallel algorithm.
/// Any time algorithms will compute until a deadline is hit and the process is killed.
/// They are therefore run in another process and communicate through shared memory.
/// This function is intended to be called from blobwar_iterative_deepening.
pub fn min_max_par_anytime(state: &Configuration) {
    let mut movement = AtomicMove::connect().expect("failed connecting to shmem");
    for depth in 1..100 {
        movement.store(MinMaxPar(depth).compute_next_move(state));
    }
}