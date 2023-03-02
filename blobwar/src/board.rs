//! Board related features. Provides the `Board` structure storing holes.
use std::cmp::{max, min};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

use super::positions::{BoardPosition, Position, Positions};

/// Board representation.
pub struct Board {
    /// `Positions` of the holes.
    pub holes: Positions,
    /// Associate to each position an uncompressed set of neighbouring positions (at distance 1 and at distance 2)
    /// (prefiltered with holes).
    /// For example individual_neighbours[0][5] is a vector of all positions which are not holes
    /// and are neighbours of position 5 at distance 1.
    pub individual_neighbours: [Vec<Vec<Position>>; 2],
    /// Associate to each `Position` the `Positions` of all its neighbours.
    pub neighbours: Vec<Positions>,
}

impl Default for Board {
    fn default() -> Self {
        Board::new(Default::default())
    }
}

impl Board {
    /// Compute new `Board` structure from given holes.
    pub fn new(holes: Positions) -> Self {
        let mut board = Board {
            holes,
            individual_neighbours: [Vec::new(), Vec::new()],
            neighbours: Vec::new(),
        };
        board.fill_individual_neighbours();
        board.fill_neighbours();
        board
    }

    /// Load a `Board` from given file.
    pub fn load<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = File::open(Path::new("boards").join(path))?;
        let mut bit = 1u64;
        let mut positions_code = 0;
        for line in io::BufReader::new(file).lines() {
            for character in line?.chars() {
                if character == 'x' {
                    positions_code |= bit;
                }
                bit <<= 1;
            }
        }
        Ok(Board::new(Positions(positions_code)))
    }

    /// Pre-compute valid neighbours for each position.
    fn fill_individual_neighbours(&mut self) {
        for position in 0i8..64i8 {
            self.individual_neighbours[0].push(Vec::new());
            self.individual_neighbours[1].push(Vec::new());
            let x = position % 8;
            let y = position / 8;
            for neighbouring_x in max(0, x - 2)..=min(7, x + 2) {
                for neighbouring_y in max(0, y - 2)..=min(7, y + 2) {
                    let distance =
                        max((neighbouring_x - x).abs(), (neighbouring_y - y).abs()) as usize;
                    if distance != 0 {
                        let coordinate =
                            Position::from_2d(neighbouring_x as u8, neighbouring_y as u8);
                        if !self.holes.contains(coordinate) {
                            self.individual_neighbours[distance - 1][position as usize]
                                .push(coordinate);
                        }
                    }
                }
            }
        }
    }

    /// Compute neighbours `Positions`.
    fn fill_neighbours(&mut self) {
        for position in 0..64 {
            self.neighbours.push(
                self.individual_neighbours[0][position]
                    .iter()
                    .map(|p| Positions::single(*p))
                    .fold(Default::default(), |a, b| a.union_with(b)),
            );
        }
    }

    /// Deserialize serialized `Configuration` into `Board`.
    pub fn deserialize(string: &str) -> Self {
        let mut holes = 0;
        let mut bit = 1u64;
        for code in string.chars().skip(1) {
            match code {
                'h' => holes |= bit,
                ' ' | 'r' | 'b' => {}
                _ => panic!("invalid cell content"),
            }
            bit <<= 1;
        }
        Board::new(Positions(holes))
    }
}
