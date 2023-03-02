//! Implementation of the min max algorithm.
use super::Strategy;
use crate::configuration::{Configuration, Movement};
use crate::shmem::AtomicMove;
use std::fmt;

/// Min-Max algorithm with a given recursion depth.ch
pub struct MinMax(pub u8);

impl Strategy for MinMax {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        //unimplemented!("TODO: implementer min max");
        let (movement, _) = MinMax::min_max(self, state, self.0, state.current_player);
        movement
    }
}
impl MinMax{
    fn min_max(&mut self, state: &Configuration, depth: u8, is_maximizing_player: bool) -> (Option<Movement>, i8) {
        if depth == 0 {
            return (None, if state.current_player {state.value()} else {-state.value()});
        }
        //println!("{depth}");
        let mut value;
        let mut best_movement: Option<Movement> = None;

        if !is_maximizing_player {
            //maximizing player
            value = i8::MIN;

            for movement in state.movements(){

                // We play the current move
                // let new_state = state.clone();
                // new_state.play(&movement);
                let (_ , new_state_val) = self.min_max(&state.play(&movement), depth - 1, !is_maximizing_player);
                if value < new_state_val {
                    value = new_state_val;
                    best_movement = Some(movement);
                }
            }
            
        } else {
            //minimizing player
            value = i8::MAX;

            for movement in state.movements(){

                // let new_state = state.clone();
                // new_state.play(& movement);

                let (_ , new_state_val) = self.min_max(&state.play(&movement), depth - 1, !is_maximizing_player);

                if value > new_state_val {
                    value = new_state_val;
                    best_movement = Some(movement);
                }
            }  
        }
        return (best_movement, value) ;
    }
}
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Min - Max (max level: {})", self.0)
    }
}

/// Anytime min max algorithm.
/// Any time algorithms will compute until a deadline is hit and the process is killed.
/// They are therefore run in another process and communicate through shared memory.
/// This function is intended to be called from blobwar_iterative_deepening.
pub fn min_max_anytime(state: &Configuration) {
    let mut movement = AtomicMove::connect().expect("failed connecting to shmem");
    for depth in 1..100 {
        movement.store(MinMax(depth).compute_next_move(state));
    }
}
