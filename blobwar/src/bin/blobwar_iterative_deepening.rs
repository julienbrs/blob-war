use blobwar::board::Board;
use blobwar::configuration::Configuration;
use blobwar::strategy::{alpha_beta_anytime, alpha_beta_pass_anytime, min_max_anytime, alpha_beta_table_anytime, min_max_par_anytime};
use std::env;

fn main() {
    let config_string = env::args()
        .nth(1)
        .expect("missing argument to iterative deepening");
    let strategy_index = env::args()
        .nth(2)
        .expect("missing argument to iterative deepening");
    let board = Board::deserialize(&config_string);
    let configuration = Configuration::deserialize(&config_string, &board);
    match strategy_index
        .parse::<usize>()
        .expect("error parsing strategy integer")
    {
        0 => min_max_anytime(&configuration),
        1 => alpha_beta_anytime(&configuration),
        2 => alpha_beta_pass_anytime(&configuration),
        3 => alpha_beta_table_anytime(&configuration),
        4 => min_max_par_anytime(&configuration),
        _ => panic!("invalid strategy number"),
    }
}
