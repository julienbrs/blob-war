//! a `Positions` is a set of 64 bits locating something on the board.
//! we use it to keep track of blue blobs, red blobs and holes.
use std;
use std::fmt;
use std::iter::repeat;
use std::ops::Deref;

/// Coordinate of a board cell (between 0 and 64).
pub type Position = u8;

pub trait BoardPosition {
    /// Convert 2D coordinates to board coordinates.
    fn from_2d(x: u8, y: u8) -> Self;
    /// Convert board coordinates to 2d coordinates.
    fn to_2d(self) -> (u8, u8);
    /// Compute distance between two board coordinates.
    fn distance_to(self, other: Self) -> u8;
}

impl BoardPosition for Position {
    fn from_2d(x: u8, y: u8) -> Self {
        y * 8 + x
    }
    fn to_2d(self) -> (u8, u8) {
        (self % 8, self / 8)
    }
    fn distance_to(self, other: Self) -> u8 {
        let (x1, y1) = self.to_2d();
        let (x2, y2) = other.to_2d();
        std::cmp::max((x2 as i8 - x1 as i8).abs(), (y2 as i8 - y1 as i8).abs()) as u8
    }
}

#[derive(Copy, Clone)]
/// Set of `Position` as a bitfield (position 0 is bit of lowest weight)
/// This allows to store any combination of board cells in a very compact manner.
/// Moreover we can then use bit masking operations to compute intersections and unions...
pub struct Positions(pub u64);

impl Deref for Positions {
    type Target = u64;
    fn deref(&self) -> &u64 {
        &self.0
    }
}

impl Positions {
    /// Invert all positions we contain.
    pub fn invert(&self) -> Self {
        Positions(!self.0)
    }
    /// Iterate on bits from lowest to highest.
    /// Will stop as soon as all remaining bits are set to 0.
    pub fn bits(&self) -> BitIterator {
        BitIterator { remaining: self.0 }
    }
    /// Iterate on all our 64 bits.
    pub fn full_bits(&self) -> impl Iterator<Item = bool> {
        self.bits().chain(repeat(false)).take(64)
    }
    /// Do we have something on given `Position` ?
    pub fn contains(&self, position: Position) -> bool {
        !self
            .intersection_with(Positions::single(position))
            .is_empty()
    }
    /// Iterate on all `Position` inside us.
    pub fn positions(&self) -> impl Iterator<Item = Position> {
        self.bits()
            .enumerate()
            .filter(|&(_, bit)| bit)
            .map(|(position, _)| position as Position)
    }
    /// Do we contain nothing ?
    pub fn is_empty(&self) -> bool {
        self.eq(&0)
    }
    /// How many bits are set ?
    pub fn len(&self) -> i8 {
        self.count_ones() as i8
    }
    /// Return positions obtained when intersecting with given ones.
    pub fn intersection_with(&self, other: Positions) -> Positions {
        Positions(self.0 & other.0)
    }
    /// Return positions obtained when taking union with given ones.
    pub fn union_with(&self, other: Positions) -> Positions {
        Positions(self.0 | other.0)
    }
    /// Remove given `Positions` from us.
    pub fn remove(&mut self, to_remove: Positions) {
        self.0 &= !to_remove.0
    }
    /// Add given `Positions` inside us.
    pub fn add(&mut self, to_remove: Positions) {
        self.0 |= to_remove.0
    }
    /// `Positions` initialized with only given `Position` inside.
    pub fn single(position: Position) -> Self {
        Positions(1u64 << position)
    }
    /// Do we contain every possible `Position` ?
    pub fn is_all(&self) -> bool {
        self.0 == std::u64::MAX
    }
}

impl Default for Positions {
    fn default() -> Self {
        Positions(0)
    }
}

pub struct BitIterator {
    remaining: u64,
}

impl Iterator for BitIterator {
    type Item = bool;
    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            None
        } else {
            let bit = self.remaining & 1;
            self.remaining >>= 1;
            Some(bit == 1)
        }
    }
}

impl fmt::Display for Positions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut positions = self.positions();
        write!(f, "[")?;
        if let Some(position) = positions.next() {
            write!(f, "{}", position)?;
        }
        for position in positions {
            write!(f, ",{}", position)?;
        }
        write!(f, "]")?;
        Ok(())
    }
}
