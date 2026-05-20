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

    fn check_if_square_is_in_bounds (row : isize, col : isize) -> bool{
        row >= 0 &&
        row < 8 &&
        col >= 0 &&
        col < 8
    }

    pub fn check_if_square_is_occupied(&self, board : &[[Option<Piece>; 8]; 8]) -> bool{
        board[self.row][self.col].is_some()
    }

    pub fn check_if_square_is_capturable(&self, game : &Game) -> bool{
        game.board[self.row][self.col].is_some_and(|piece| piece.is_white != game.white_to_move)
    }
    
}


