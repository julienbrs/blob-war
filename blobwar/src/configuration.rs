//! Provide a `Configuration` for storing game state ; a `Movement` for storing moves to play.
use super::board::Board;
use super::positions::{BoardPosition, Position, Positions};
use super::strategy::Strategy;
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::iter::once;
use term;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
/// Movements : clone some blob or jump
pub enum Movement {
    /// For copy we just need the destination
    Duplicate(u8),
    /// For jumps we need a source and a destination
    Jump(u8, u8),
}

#[derive(Copy, Clone)]
/// Game state. We know, who should play, what is the board and where every blob is located.
pub struct Configuration<'a> {
    blobs: [Positions; 2],
    board: &'a Board,
    /// Who should play now.
    pub current_player: bool,
}

impl<'a> Configuration<'a> {
    /// Create an initial game configuration out of given `Board`.
    pub fn new(board: &'a Board) -> Self {
        Configuration {
            blobs: [
                Positions::single(0).union_with(Positions::single(63)),
                Positions::single(7).union_with(Positions::single(56)),
            ],
            board,
            current_player: false,
        }
    }

    /// Play given move on self.
    pub fn apply_movement(&mut self, movement: &Movement) {
        let me = self.current_player as usize;
        let him = !self.current_player as usize;
        let destination = match *movement {
            Movement::Jump(source, destination) => {
                self.blobs[me].remove(Positions::single(source));
                destination
            }
            Movement::Duplicate(destination) => destination,
        };
        let adversary_blobs = self.blobs[him];
        let changing_blobs =
            adversary_blobs.intersection_with(self.board.neighbours[destination as usize]);
        self.blobs[me].add(changing_blobs);
        self.blobs[me].add(Positions::single(destination));
        self.blobs[him].remove(changing_blobs);
        self.current_player = !self.current_player;
    }

    /// Create a new `Configuration` by playing given `Movement` on self.
    pub fn play(&self, movement: &Movement) -> Self {
        let mut new_configuration = *self;
        new_configuration.apply_movement(movement);
        new_configuration
    }

    /// Create a new `Configuration` by skipping turn on self.
    pub fn skip_play(&self) -> Self {
        let mut new_configuration = *self;
        new_configuration.current_player = !new_configuration.current_player;
        new_configuration
    }

    /// Does target player has blob at target position set ?
    pub fn free_position_at(&self, position: Position) -> bool {
        !self.blobs[0].union_with(self.blobs[1]).contains(position)
    }

    /// Iterate on all `Position`(s) of empty cells.
    pub fn empty_cells(&self) -> impl Iterator<Item = Position> {
        self.blobs[0]
            .union_with(self.blobs[1])
            .union_with(self.board.holes)
            .invert()
            .positions()
    }

    /// Return the configuration value (#other_player - #current_player)
    pub fn value(&self) -> i8 {
        if self.current_player {
            (self.blobs[0].len() - self.blobs[1].len()) as i8
        } else {
            -(self.blobs[0].len() - self.blobs[1].len()) as i8
        }
    }

    /// Return if given movement is correct for current configuration.
    pub fn check_move(&self, movement: &Movement) -> bool {
        let destination = match *movement {
            Movement::Jump(source, destination) => {
                if !(self.blobs[self.current_player as usize].contains(source))
                    || source.distance_to(destination) != 2
                {
                    return false;
                }
                destination
            }
            Movement::Duplicate(destination) => {
                if self.board.neighbours[destination as usize]
                    .intersection_with(self.blobs[self.current_player as usize])
                    .is_empty()
                {
                    return false;
                }
                destination
            }
        };
        !self.board.holes.contains(destination)
            && !self.blobs[0].contains(destination)
            && !self.blobs[1].contains(destination)
    }

    /// Play a match between the given players starting from current `Configuration`.
    pub fn battle<T: Strategy, U: Strategy>(&mut self, mut player_one: T, mut player_two: U) {
        while !self.game_over() {
            println!(
                "{} player's turn (he is losing by {} before playing)",
                ["red", "blue"][self.current_player as usize],
                self.value()
            );
            println!("{}", self);
            let play_attempt = if self.current_player {
                player_two.compute_next_move(self)
            } else {
                player_one.compute_next_move(self)
            };
            if let Some(ref next_move) = play_attempt {
                assert!(self.check_move(next_move));
                self.apply_movement(next_move);
            } else {
                self.current_player = !self.current_player;
            }
        }

        let value = self.blobs[0].len() - self.blobs[1].len();
        match value {
            x if x > 0 => println!("RED ({}) wins over BLUE ({})!", player_one, player_two),
            x if x < 0 => println!("BLUE ({}) wins over RED ({})!", player_two, player_one),
            _ => println!("DRAW!"),
        }
        println!("{}", self);
        println!("GAME OVER (red value of {})", value);
    }

    /// Return true if no empty space remains or someone died.
    fn game_over(&self) -> bool {
        self.blobs[0].is_empty()
            || self.blobs[1].is_empty()
            || self.blobs[0]
                .union_with(self.blobs[1])
                .union_with(self.board.holes)
                .is_all()
    }

    /// Iterate on all possible jumps for given player.
    fn jumps<'b>(&'b self) -> impl 'b + Iterator<Item = Movement> {
        self.blobs[self.current_player as usize]
            .positions()
            .flat_map(move |start| {
                // look at all distance 2 neighbours
                self.board.individual_neighbours[1][start as usize]
                    .iter()
                    .filter(move |&end| self.free_position_at(*end))
                    .map(move |end| Movement::Jump(start, *end))
            })
    }

    /// Iterate on all possible duplications for given player.
    fn duplicates<'b>(&'b self) -> impl 'b + Iterator<Item = Movement> {
        self.empty_cells()
            .filter(move |&p| {
                !self.blobs[self.current_player as usize]
                    .intersection_with(self.board.neighbours[p as usize])
                    .is_empty()
            })
            .map(Movement::Duplicate)
    }

    /// Iterate on all possible moves.
    pub fn movements<'b>(&'b self) -> impl 'b + Iterator<Item = Movement> {
        self.duplicates().chain(self.jumps())
    }

    /// Serialize `Configuration` into a `String`.
    /// Use in communications with sub-processes.
    pub fn serialize(&self) -> String {
        once(if self.current_player { '1' } else { '0' })
            .chain(
                self.board
                    .holes
                    .full_bits()
                    .zip(self.blobs[0].full_bits().zip(self.blobs[1].full_bits()))
                    .map(|(h, (r, b))| match (h, r, b) {
                        (true, false, false) => 'h',
                        (false, true, false) => 'r',
                        (false, false, true) => 'b',
                        (false, false, false) => ' ',
                        _ => panic!("invalid configuration"),
                    }),
            )
            .collect()
    }

    /// Deserialize given `String` into a `Configuration`. You need to deserialize the `Board`
    /// first.
    pub fn deserialize(string: &str, board: &'a Board) -> Self {
        let mut chars = string.chars();
        let current_player;
        if let Some(player_char) = chars.next() {
            current_player = match player_char {
                '1' => true,
                '0' => false,
                _ => panic!("invalid player code"),
            }
        } else {
            panic!("missing player code");
        }
        let mut blobs = [0; 2];
        let mut bit = 1u64;
        for code in chars {
            match code {
                'r' => blobs[0] |= bit,
                'b' => blobs[1] |= bit,
                ' ' | 'h' => {}
                _ => panic!("invalid cell content"),
            }
            bit <<= 1;
        }
        Configuration {
            board,
            blobs: [Positions(blobs[0]), Positions(blobs[1])],
            current_player,
        }
    }
}

impl<'a> fmt::Display for Configuration<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n  01234567 \n")?;
        write!(f, " +--------+\n")?;
        for (index, (hole, (red, blue))) in self
            .board
            .holes
            .full_bits()
            .zip(self.blobs[0].full_bits().zip(self.blobs[1].full_bits()))
            .enumerate()
        {
            if index % 8 == 0 {
                write!(f, "{}|", index / 8)?;
            }
            let mut t = term::stdout().unwrap();
            match (hole, red, blue) {
                (true, false, false) => write!(f, "x")?,
                (false, true, false) => {
                    t.fg(term::color::RED).unwrap();
                    write!(f, "x")?;
                    t.reset().unwrap();
                }
                (false, false, true) => {
                    t.fg(term::color::CYAN).unwrap();
                    write!(f, "o")?;
                    t.reset().unwrap();
                }
                (false, false, false) => write!(f, " ")?,
                _ => panic!("invalid board: {} {} {}", hole, red, blue),
            }
            if index % 8 == 7 {
                write!(f, "|\n")?;
            }
        }
        write!(f, " +--------+")?;
        Ok(())
    }
}
