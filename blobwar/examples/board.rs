use glob::glob;

use blobwar::board::Board;
use blobwar::configuration::Configuration;

fn main() {
    for board_name in glob("boards/*").expect("missing boards directory") {
        match board_name {
            Ok(path) => {
                let board = Board::load(path.file_name().unwrap()).expect("failed loading map");
                let configuration = Configuration::new(&board);
                println!("{}{}", path.to_str().unwrap(), configuration);
            }
            Err(e) => println!("{:?}", e),
        }
    }
}
