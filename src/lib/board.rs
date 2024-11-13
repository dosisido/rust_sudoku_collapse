use super::tile::Tile;
pub const SIZE: usize = 9;
use super::lib::screen_clear;

#[derive(Clone)]
pub struct Board{
    board: [[Tile; SIZE]; SIZE],
    print: bool
}

impl Board{
    pub fn new(pr: bool) -> Self{
        Board{
            board: core::array::from_fn(|_| core::array::from_fn(|_| Tile::new())),
            print: pr
        }
    }

    pub fn from_text_matrix(&mut self, data: String, null_element: char) ->(){
        let data = data.trim().replace(' ', "");
        let data: Vec<String> = data.split('\n').map(|s| s.to_string()).collect();
        for (row, row_string) in data.iter().enumerate(){
            for (col, char) in row_string.trim().to_string().chars().enumerate(){
                if char != null_element{
                    if self.print{
                        screen_clear();
                        println!("Analizing ({}, {}), with value {}\n", row+1, col+1, char);
                        self.pretty_print();
                        std::io::stdin().read_line(&mut String::new()).unwrap();
                    }
                    self.collapse(row, col, char.to_digit(10).unwrap() as usize);
                }
            }
        }
    }

    fn collapse(&mut self, row: usize, col: usize, value: usize)->(){
        self.board[row][col].set(value);

        for i in 0..SIZE{
            self.board[row][i].collapse(value);
            self.board[i][col].collapse(value);
        }
        let s = (SIZE as f64).sqrt() as usize;
        let (r, c) = ( (row as f64/s as f64).floor() as usize, (col as f64/s as f64).floor() as usize );
        for i in (r*s)..(r*s + s) {
            for j in (c* s)..(c*s + s) {
                self.board[i][j].collapse(value);
            }
        }
    }

    pub fn pretty_print(&mut self)->(){
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
                                false => "_"
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