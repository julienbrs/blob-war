//! Implementation of the min max algorithm.
use super::Strategy;
use crate::configuration::{ Configuration, Movement };
use crate::shmem::AtomicMove;
use rayon::prelude::*;
use std::fmt;

/// Min-Max algorithm with a given recursion depth.ch
pub struct MinMax(pub u8);

impl Strategy for MinMax {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        //let (movement, _) = MinMax::min_max(self, state, self.0, state.current_player);

        let (movement, _) = MinMax::min_max_par(self, state, self.0, false);
        movement
    }
}
impl MinMax {
    /// Classic min-max algorithm
    fn min_max(
        &mut self,
        state: &Configuration,
        depth: u8,
        is_maximizing_player: bool
    ) -> (Option<Movement>, i8) {
        if depth == 0 {
            return (None, state.value());
        }
        //println!("{depth}");
        let mut value;
        let mut best_movement: Option<Movement> = None;
        let mut empty = true;
        if !is_maximizing_player {
            //maximizing player
            value = i8::MIN;

            for movement in state.movements() {
                // We play the current move
                // let new_state = state.clone();
                // new_state.play(&movement);
                empty = false;
                let (_, new_state_val) = self.min_max(
                    &state.play(&movement),
                    depth - 1,
                    !is_maximizing_player
                );
                if value < new_state_val {
                    value = new_state_val;
                    best_movement = Some(movement);
                }
            }
        } else {
            //minimizing player
            value = i8::MAX;

            for movement in state.movements() {
                empty = false;
                // let new_state = state.clone();
                // new_state.play(& movement);

                let (_, new_state_val) = self.min_max(
                    &state.play(&movement),
                    depth - 1,
                    !is_maximizing_player
                );

                if value > new_state_val {
                    value = new_state_val;
                    best_movement = Some(movement);
                }
            }
        }
        if empty {value = state.value();}
        return (best_movement, value);
    }

    /// Shorter, smarter min-max algorithm (only minimize)
    fn min_max_two(&mut self, state: &Configuration, depth: u8, opposing_player:bool) -> (Option<Movement>, i8){
        if depth == 0 {
            // state.value() indicates the loss of the current player
            return (None, state.value());
        }

        let mut value;
        let mut best_movement: Option<Movement> = None;
        let mut empty = true;
        // Minimize loss for our player
        value = i8::MAX;

        for movement in state.movements() {
            empty = false;
            // We play the current move

            let (_, mut new_state_val) = self.min_max_two(
                &state.play(&movement),
                depth - 1,
                !opposing_player
            );
            // If opposing player, we want to maximize the loss
            // => minimizing the gain
            if opposing_player {new_state_val = -new_state_val;}
            if value > new_state_val {
                value = new_state_val;
                best_movement = Some(movement);
            }
        }
        // And restore original value
        if opposing_player{
            value = - value;
        }
        if empty {value = state.value();}
        (best_movement, value)
    }
    
    /// Parallel min-max algorithm
    fn min_max_par(&mut self, state: &Configuration, depth: u8, opposing_player:bool) -> (Option<Movement>, i8){
        if depth == 0 {
            // state.value() indicates the loss of the current player
            return (None, state.value());
        }

        let value = i8::MIN;
        let best_movement: Option<Movement> = None;

        // Minimize loss for our player

        let values= state.movements().map(|movement| {
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