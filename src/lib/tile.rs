use super::board::SIZE;
use std::vec::Vec;

#[derive(Debug, Clone)]
pub struct Tile{
    data: Vec<usize>,
    setted: bool
}

impl Tile{
    pub fn new() -> Self {
        Tile {
            data: Vec::from_iter(1..=SIZE),
            setted: false,
        }
    }

    pub fn _len(&self)-> usize{
        self.data.len()
    }

    pub fn collapse(&mut self, value: usize) -> (){
        if !self.setted{
            self.data.retain(|&x| x != value);
        }
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