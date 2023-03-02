use blobwar::board::Board;
use blobwar::configuration::Configuration;
use blobwar::strategy::{IterativeDeepening, IterativeStrategy, Strategy};

use std::env::args;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::net::TcpStream;

fn main() {
    let address = args().nth(1).expect("missing machine name or IP address");
    let mut strategy = IterativeDeepening::new(IterativeStrategy::AlphaBeta);
    let mut sending =
        TcpStream::connect((address.as_str(), 12_345)).expect("failed connecting to server");
    sending.set_nodelay(true).expect("failed setting no delay");
    let receiving = BufReader::new(sending.try_clone().expect("failed cloning socket"));

    for line in receiving
        .lines()
        .map(|r| r.expect("failed reading configuration from server"))
    {
        let board = Board::deserialize(&line);
        let game = Configuration::deserialize(&line, &board);
        let next_move = strategy.compute_next_move(&game);
        serde_json::to_writer(&mut sending, &next_move).expect("sending back movement failed");
        sending.write(b"\n").expect("newline failed");
    }
}
