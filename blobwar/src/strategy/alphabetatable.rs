//! Alpha - Beta algorithm with Transposition Table.
use std::collections::HashMap;
use std::fmt;

use super::{BenchmarkUnitaire, Strategy};
use crate::configuration::{Configuration, Movement};
use crate::shmem::AtomicMove;
use rand::Rng;

const BOARD_SIZE: usize = 8;
const PIECE_TYPES: usize = 2;

fn generate_zobrist_table() -> [[[u64; BOARD_SIZE]; BOARD_SIZE]; PIECE_TYPES] {
    let mut table = [[[0; BOARD_SIZE]; BOARD_SIZE]; PIECE_TYPES];
    let mut rng = rand::thread_rng();

    for i in 0..PIECE_TYPES {
        for j in 0..BOARD_SIZE {
            for k in 0..BOARD_SIZE {
                table[i][j][k] = rng.gen();
            }
        }
    }
    table
}

impl BenchmarkUnitaire for AlphaBetaTable {
    fn new(depth: u8) -> Self {
        return AlphaBetaTable(depth);
    }
}

struct TranspositionTable {
    table: HashMap<u64, i8>,
}

impl TranspositionTable {
    fn new() -> Self {
        TranspositionTable {
            table: HashMap::new(),
        }
    }

    fn get(&self, key: u64) -> Option<i8> {
        self.table.get(&key).cloned()
    }

    fn set(&mut self, key: u64, value: i8) {
        self.table.insert(key, value);
    }
}

/// Anytime alpha beta algorithm.
/// Any time algorithms will compute until a deadline is hit and the process is killed.
/// They are therefore run in another process and communicate through shared memory.
/// This function is intended to be called from blobwar_iterative_deepening.
pub fn alpha_beta_table_anytime(state: &Configuration) {
    let mut movement = AtomicMove::connect().expect("failed connecting to shmem");
    for depth in 1..100 {
        let chosen_movement = AlphaBetaTable(depth).compute_next_move(state);
        movement.store(chosen_movement);
    }
}

/// Alpha - Beta algorithm with given maximum number of recursions.
pub struct AlphaBetaTable(pub u8);

impl fmt::Display for AlphaBetaTable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Alpha - Beta TABLE (max level: {})", self.0)
    }
}

impl Strategy for AlphaBetaTable {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        let zobrist_table = generate_zobrist_table();
        let (alpha, beta) = (i8::MIN, i8::MIN);
        let (movement, _) = AlphaBetaTable::alphabeta_transposition_table(
            self,
            state,
            self.0,
            alpha,
            beta,
            false,
            &mut TranspositionTable::new(),
            &zobrist_table,
        );
        movement
    }
}

impl AlphaBetaTable {
    fn alphabeta_transposition_table(
        &mut self,
        state: &Configuration,
        depth: u8,
        mut alpha: i8,
        mut beta: i8,
        opposing_player: bool,
        transposition_table: &mut TranspositionTable,
        zobrist_table: &[[[u64; BOARD_SIZE]; BOARD_SIZE]; PIECE_TYPES],
    ) -> (Option<Movement>, i8) {
        if depth == 0 {
            return (None, state.value());
        }

        let mut best_movement: Option<Movement> = None;
        let mut best_value: i8 = i8::MIN;

        for movement in state.movements() {
            let new_value: i8;
            // Check if the current state is already in the transposition table.
            if let Some(value) = transposition_table.get(state.zobrist_key(*zobrist_table)) {
                new_value = if opposing_player { -value } else { value };
            } else {
                (_, new_value) = self.alphabeta_transposition_table(
                    &state.play(&movement),
                    depth - 1,
                    alpha,
                    beta,
                    !opposing_player,
                    transposition_table,
                    zobrist_table,
                );
            }

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
            let (_, val) = AlphaBetaTable::alphabeta_transposition_table(
                self,
                state,
                depth - 1,
                alpha,
                beta,
                true,
                transposition_table,
                zobrist_table,
            );
            (None, val)
        } else {
            return (best_movement, -best_value);
        }
    }
}
