mod lib{
    pub mod board;
    pub mod tile;
    pub mod lib;
}
use lib::board::Board;
use lib::lib::{screen_clear, wait_for_enter};

const SUDOKU_TABLE: &str = "
0 0 0 0 0 0 0 9 0
0 4 0 0 2 1 0 0 0
8 0 0 0 0 0 0 7 0
0 0 6 0 0 0 0 0 0
9 0 0 0 0 3 0 0 0
0 2 0 0 5 0 6 0 0
2 0 0 6 3 0 0 0 0
0 0 5 0 7 0 0 0 0
0 0 0 0 0 0 1 8 0
";


fn main() {

    let mut sudoku = Board::new(
        Some(true),
        Some(false),
        );
    
    println!("Reading matrix");
    sudoku.add_debug((6, 8));

    sudoku.from_text_matrix(SUDOKU_TABLE.to_string(), '0');

    screen_clear();
    println!("Input matrix loaded");
    sudoku.pretty_print(None);
    
    
    println!("Starting solving");
    wait_for_enter();
    
    sudoku.collapse_all();
    
    println!("Final configuration");
    screen_clear();
    sudoku.pretty_print(None);

}
