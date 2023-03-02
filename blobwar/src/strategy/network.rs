//! Network player (server side)
use super::Strategy;
use crate::configuration::{Configuration, Movement};

use serde_json::{de, Deserializer, StreamDeserializer};
use std::fmt;
use std::io::prelude::*;
use std::net::TcpStream;

/// Let a remote client enter moves.
pub struct NetworkPlayer {
    connection: TcpStream,
    movements: StreamDeserializer<'static, de::IoRead<TcpStream>, Option<Movement>>,
    name: String,
}

impl fmt::Display for NetworkPlayer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "On network : {}", self.name)
    }
}

impl NetworkPlayer {
    /// Create a new network player
    pub fn new(data: TcpStream) -> Self {
        let origin = data.peer_addr().unwrap().to_string();
        let connection = data.try_clone().unwrap();
        let movements = Deserializer::from_reader(data).into_iter::<Option<Movement>>();
        NetworkPlayer {
            connection,
            movements,
            name: origin,
        }
    }
}

impl Strategy for NetworkPlayer {
    fn compute_next_move(&mut self, configuration: &Configuration) -> Option<Movement> {
        let mut message = configuration.serialize();
        message.push('\n');
        self.connection
            .write_all(message.into_bytes().as_slice())
            .expect("sending configuration remotely failed");
        self.movements.next().unwrap().unwrap()
    }
}
