// extern crate blobwar;
//use blobwar::board::Board;
use blobwar::configuration::Configuration;
use blobwar::strategy::IterativeDeepening;
use blobwar::strategy::{
    AlphaBeta, AlphaBetaPass, AlphaBetaTable, Greedy, Human, IterativeStrategy, MinMax, MinMaxPar
};

fn main() {
    //let board = Board::load("x").expect("failed loading board");
    let board = Default::default();
    let mut game = Configuration::new(&board);
    game.battle(
        IterativeDeepening::new(IterativeStrategy::MinMax),
        IterativeDeepening::new(IterativeStrategy::MinMaxPar),
    );
}
