use crate::piece::{Piece};
use crate::game::Game;

#[derive(Clone, Copy, Debug)]
#[derive(PartialEq, Eq)]
pub struct Square{
    pub row : usize,
    pub col : usize
}


impl Square{
    pub fn new(row : isize, col : isize) -> Option<Self>{
        if Self::check_if_square_is_in_bounds(row, col){
            Some(Self{ row : row as usize, col : col as usize})
        } else{
            None
        }
    }

    pub fn all() -> Vec<Square>{
        let mut all_squares = Vec::new();

        for i in 0..64{
            let row = i / 8;
            let col = i % 8;
            all_squares.push(Square{row : row, col : col});
        }

        all_squares
    }

    fn check_if_square_is_in_bounds (row : isize, col : isize) -> bool{
        row >= 0 &&
        row < 8 &&
        col >= 0 &&
        col < 8
    }    
}


