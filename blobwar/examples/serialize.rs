use blobwar::board::Board;
use blobwar::configuration::Configuration;
fn main() {
    let board: Board = Default::default();
    let start_state = Configuration::new(&board);
    println!("start: {}", start_state);
    let string = start_state.serialize();
    println!("string: {}", string);
    let deserialized_board = Board::deserialize(&string);
    let deserialized_configuration = Configuration::deserialize(&string, &deserialized_board);
    println!("deserialized: {}", deserialized_configuration);
}
