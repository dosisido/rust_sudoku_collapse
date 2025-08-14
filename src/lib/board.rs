use super::tile::Tile;
pub const SIZE: usize = 9;
use super::lib::screen_clear;
use rand::seq::SliceRandom;
use std::thread::sleep;

const SLEEP: std::time::Duration = std::time::Duration::from_millis(500);

#[derive(Clone)]
pub struct Board{
    board: [[Tile; SIZE]; SIZE],
    print: bool,
    automatic: bool,
    debug_tiles: Vec<(usize, usize)>
}

impl Board{
    pub fn new(pr: Option<bool>, auto: Option<bool>) -> Self{
        Board{
            board: core::array::from_fn(|row| core::array::from_fn(|col| Tile::new(row, col))),
            print: pr.unwrap_or(false),
            automatic: auto.unwrap_or(false),
            debug_tiles: Vec::new(),
        }
    }

    pub fn add_debug(&mut self, tile: (usize, usize))->(){
        self.debug_tiles.push(tile);
    }

    fn collapse_tile(&mut self, row:usize, col: usize, value: usize, orig_row: usize, orig_col: usize)->(){
        let dbg = self.debug_tiles.iter().any(|&(r, c)| r == row && c == col);
        let mut c: Option<Box<dyn Fn()>> = None;

        if dbg{
            let closure = || println!("Collapsing tile ({}, {}) with value {} from tile ({}, {})", row, col, value, orig_row, orig_col);
            c = Some(Box::new(closure));
        }
        self.board[row][col].collapse(value, c.as_deref()).unwrap();
    }

    pub fn from_text_matrix(&mut self, data: String, null_element: char) ->(){
        let data = data.trim().replace(' ', "");
        let data: Vec<String> = data.split('\n').map(|s| s.to_string()).collect();
        for (row, row_string) in data.iter().enumerate(){
            for (col, char) in row_string.trim().to_string().chars().enumerate(){
                if char != null_element{
                    self.collapse(row, col, char.to_digit(10).unwrap() as usize);
                    if self.print{
                        self.print_wrap(Some(
                                &|| println!("Fixing ({}, {}), with value {}\n", row+1, col+1, char),
                            ),
                            Some((row, col))
                        );
                    }
                }
            }
        }
    }

    fn collapse(&mut self, row: usize, col: usize, value: usize)->(){
        self.board[row][col].set(value);

        for i in 0..SIZE{
            self.collapse_tile(row, i, value, row, col);
            self.collapse_tile(i, col, value, row, col);
        }
        let s = (SIZE as f64).sqrt() as usize;
        let (r, c) = ( (row as f64/s as f64).floor() as usize, (col as f64/s as f64).floor() as usize );
        for i in (r*s)..(r*s + s) {
            for j in (c* s)..(c*s + s) {
                self.collapse_tile(i, j, value, row, col);
            }
        }
    }

    pub fn collapse_all(&mut self) -> (){
        loop{
            let mut analize: Vec<&Tile> = Vec::new();
            self.board.iter().for_each(|row| {
                row.iter().for_each(|tile| {
                    if tile.len() == 1 && !tile.is_setted(){
                        analize.push(tile);
                    }
                });
            });
            let analize = analize;

            if analize.len() == 0{ return;}

            let chosen_element: &Tile = analize.choose(&mut rand::thread_rng()).unwrap();
            let (row, col, value) = (chosen_element.row, chosen_element.col, chosen_element.get_the_value().unwrap());

            self.collapse(row, col, value);

            self.print_wrap(Some(
                &|| println!("Collapsing ({}, {}), with value {}\n",row+1, col+1, value)    
                ),
                Some((row, col))
            );

        }
    }

    fn print_wrap(&mut self, callback: Option<&dyn Fn()>, highlight_cell: Option<(usize, usize)>){
        screen_clear();
        if let Some(cb) = callback { cb(); }
        self.pretty_print(highlight_cell);
        if self.automatic{
            sleep(SLEEP);
        }else{
            super::lib::wait_for_enter();
        }
    }

    pub fn pretty_print(&mut self, highlight_cell: Option<(usize, usize)>)->(){
        let s = (SIZE as f64).sqrt() as usize;

        let get_sub_number = |sub_row: usize, sub_col: usize| -> usize {
            s * sub_row + sub_col
        };

        let mut out_str = String::new();
        let str_concat = [
                " ".repeat((s+1) * s),
                "| ".to_string(),
                " ".repeat((s+1) * s),
                "| ".to_string(),
                " ".repeat((s+1) * s),
                "\n".to_string(),
            ].join("");

        for row in 0..SIZE{
            for subrow in 0..s{
                for col in 0..SIZE{
                    for subcol in 1..=s{
                        let n = get_sub_number(subrow, subcol);
                        let tmp_str: String;
                        out_str.push_str(
                            match self.board[row][col].contains(n) {
                                true => {
                                    tmp_str = n.to_string();
                                    tmp_str.as_str()
                                }
                                false => {
                                    if self.board[row][col].is_setted(){
                                        " "
                                    }else if highlight_cell.map_or(false, |(r, c)| row == r && col == c){
                                        "*"
                                    }else{
                                        "_"
                                    }
                                }
                            }
                        );
                    }
                    out_str.push_str(" ");
                    if col % s == s - 1 && col != SIZE-1{
                        out_str.push_str("| ");
                    }
                }
                out_str.push_str("\n");
            }
            
            if row!=SIZE-1{
                out_str.push_str(&str_concat);
            }
            if row % s == s - 1 && row!=SIZE-1{
                out_str.push_str(&"-".repeat((s+1) * SIZE + 2*(s-1)));
                out_str.push_str("\n");
            }
        }

        println!("{}", out_str);
    }

}

impl std::fmt::Display for Board{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.board.iter(){
            for tile in row.iter(){
                write!(f, "{} ", tile)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}