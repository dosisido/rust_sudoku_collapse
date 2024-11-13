mod lib{
    pub mod board;
    pub mod tile;
    pub mod lib;
}
use lib::board::Board;
use lib::lib::screen_clear;

// const SUDOKU_TABLE: &str = "
//     0 0 0 0 0 0 0 0 0
//     0 0 0 0 0 0 0 0 0
//     0 0 0 0 0 0 0 0 0
//     0 0 0 0 0 0 0 0 0
//     0 0 0 0 0 0 0 0 0
//     0 0 0 0 0 0 0 0 0
//     0 0 0 0 0 0 0 0 0
//     0 0 0 0 0 0 0 0 0
//     0 0 0 0 0 0 0 0 0
// ";
const SUDOKU_TABLE: &str = "
    0 0 1 0 0 2 4 6 0
    6 0 0 0 0 5 0 1 8
    8 2 0 9 1 0 6 0 0
    0 0 8 0 4 0 5 0 0
    1 0 0 8 0 0 0 6 9
    0 0 0 0 0 8 2 7 0
    5 1 4 7 3 0 6 2 0
    3 0 7 0 2 4 0 0 0
    0 2 0 0 1 3 7 0 0
";


fn main() {

    let mut sudoku = Board::new(true);
    
    println!("Reading matrix");
    sudoku.from_text_matrix(SUDOKU_TABLE.to_string(), '0');

    screen_clear();
    println!("Final cofiguration");
    sudoku.pretty_print();

}
