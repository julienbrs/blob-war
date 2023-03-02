//! Generic iterative deepening strategies (with variable algorithms).
use std::fmt;

use std::io;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

use super::Strategy;
use crate::configuration::{Configuration, Movement};
use crate::shmem::AtomicMove;

#[derive(Copy, Clone, Debug)]
/// All possible strategies for iterative deepening.
pub enum IterativeStrategy {
    /// MinMax algorithm
    MinMax,
    /// AlphaBeta algorithm
    AlphaBeta,
}

/// Anytime algorithms strategies. Implemented in another process.
pub struct IterativeDeepening {
    strategy: IterativeStrategy,
    duration: u64,
}

impl fmt::Display for IterativeDeepening {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?} (iterative deepening {}ms)",
            self.strategy, self.duration
        )
    }
}

impl Strategy for IterativeDeepening {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        let movement = AtomicMove::new().expect("failed creating shmem");
        let configuration = state.serialize();
        let mut child = Command::new("blobwar_iterative_deepening")
            .current_dir("./target/release/")
            .arg(configuration)
            .arg((self.strategy as usize).to_string())
            .spawn()
            .expect("failed to start child process");
        sleep(Duration::from_millis(self.duration));
        if let Err(e) = child.kill() {
            if e.kind() != io::ErrorKind::InvalidInput {
                panic!("failed to kill child")
            }
        }
        movement.load()
    }
}

impl IterativeDeepening {
    /// New iterative deepening strategy with given algorithm.
    /// default time is 2 seconds.
    pub fn new(strategy: IterativeStrategy) -> IterativeDeepening {
        IterativeDeepening {
            strategy,
            duration: 1000,
        }
    }

    /// Sets duration in milliseconds on given algorithm.
    pub fn duration(&self, duration: u64) -> Self {
        IterativeDeepening {
            strategy: self.strategy,
            duration,
        }
    }
}
