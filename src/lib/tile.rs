use super::board::SIZE;
use std::vec::Vec;

#[derive(Debug, Clone)]
pub struct Tile{
    data: Vec<usize>,
    setted: bool,
    pub row: usize,
    pub col: usize
}

impl Tile{
    pub fn new(row: usize, col: usize) -> Self {
        Tile {
            data: Vec::from_iter(1..=SIZE),
            setted: false,
            row,
            col
        }
    }

    pub fn len(&self)-> usize{
        self.data.len()
    }

    pub fn is_setted(&self) -> bool{
        self.setted
    }

    pub fn get_the_value(&self)-> Result<usize, ()>{
        if self.len() != 1{
            return Err(());
        }
        Ok(self.data[0])
    }

    pub fn collapse(&mut self, value: usize, dbg_print: Option<&dyn Fn()>) -> Result<(), ()>{
        if !self.setted{
            if self.len() == 1 && self.data[0] == value{
                println!("Removing last element in cell ({},{}) with value {}", self.row+1, self.col+1, value);
                println!("Values presents: {:?}", self.data);
                
                return Err(());
            }
            // if !self.contains(value){
            //     println!("Tring to collapse cell ({},{}) with value {} but it is not present", self.row+1, self.col+1, value);
            //     println!("Values presents: {:?}", self.data);
            //     return Err(());
            // }
            self.data.retain(|&x| x != value);
            if let Some(cb) = dbg_print { cb(); }
        }
        Ok(())
    }

    pub fn set(&mut self, value: usize) -> (){
        self.setted = true;
        self.data = Vec::from_iter(value..(value+1));
    }

    pub fn contains(&mut self, value: usize)->bool{
        self.data.contains(&value)
    }
}

impl std::fmt::Display for Tile{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.setted{
            write!(f, "{:?}", self.data[0])?;
        }else{
            write!(f, "{:?}", self.data)?;
        }
        Ok(())
    }
}