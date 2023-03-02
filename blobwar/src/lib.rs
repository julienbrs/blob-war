//! rust alpha - beta implementation for the blobwar game.
#![deny(missing_docs)]
#![warn(clippy::all)]

pub mod board;
pub mod configuration;
pub(crate) mod positions;
pub(crate) mod shmem;
pub mod strategy;
