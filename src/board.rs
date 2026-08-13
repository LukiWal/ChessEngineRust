use crate::constants::{STANDARD_BOARD_SETUP};
use crate::piece::{Piece, PieceType, Color};
use crate::chess_square::{Square};
use crate::move_gen::pawn::is_attacked_by_pawn;
use crate::move_gen::knight::is_attacked_by_knight;
use crate::move_gen::sliding_pieces::is_attacked_by_bishop;
use crate::move_gen::sliding_pieces::is_attacked_by_rook;
use crate::move_gen::sliding_pieces::is_attacked_by_queen;
use crate::move_gen::king::is_attacked_by_king;
use crate::debug::{log_debug, log_value};


#[derive(Clone, Debug)]
pub struct Board{
    squares : [[Option<Piece>;8];8]
}

impl Board{
    pub fn new() -> Self{
        Self{squares : STANDARD_BOARD_SETUP} 
    }

    pub fn is_occupied(&self, square : Square) -> bool{
        self.get_piece(square).is_some()
    }

    pub fn is_occupied_by_color(&self, square : Square, own_color : Color) -> bool{
        self.get_piece(square).is_some_and(|piece| piece.color == own_color) 
    }

    pub fn is_occupied_by_opponent_color(&self, square : Square, own_color : Color) -> bool{
        self.get_piece(square).is_some_and(|piece| piece.color != own_color) 
    }

    pub fn get_piece(&self, square : Square) -> Option<Piece>{
        self.squares[square.row][square.col]
    }

    pub fn set_piece(&mut self, square : Square, piece : Piece){
        self.squares[square.row][square.col] = Some(piece);
    }

    pub fn remove_piece(&mut self, square : Square){
        self.squares[square.row][square.col] = None;
    }

    pub fn move_piece(&mut self, from_square : Square, to_square : Square){
        if let Some(piece_to_move) = self.get_piece(from_square){
            self.set_piece(to_square, piece_to_move);
            self.remove_piece(from_square);
        } else{
            panic!("move_piece: No Piece to Move")
        }
    }

    pub fn occupied_squares_by_color(&self, color : Color) -> Vec<(Square, Piece)>{
        let mut all_occupied_squares_by_color : Vec<(Square, Piece)> = Vec::new();

        for square in Square::all(){
            if self.is_occupied_by_color(square, color){
                if let Some (piece) = self.get_piece(square){
                    all_occupied_squares_by_color.push((square, piece));
                }              
            }
        }

        all_occupied_squares_by_color
    }

    pub fn is_king_in_check(&self, color : Color) -> bool{
        let king_square = self.find_king_by_color(color);
        self.is_square_attacked(king_square, color.opposite())
    }

    pub fn find_king_by_color(&self, color : Color) -> Square{

        for square in Square::all(){
            if let Some(piece) = self.get_piece(square){
                if piece.piece_type == PieceType::King &&
                piece.color == color{
                    return square;
                }
            }
        }

        panic!("find_king_by_color: No king found!");
    }

    pub fn is_square_attacked(&self, square : Square, attacking_color : Color) -> bool{
        let attacked_by_pawn = is_attacked_by_pawn(self, square, attacking_color);
        let attacked_by_knight = is_attacked_by_knight(self, square, attacking_color);
        let attacked_by_bishop = is_attacked_by_bishop(self, square, attacking_color);
        let attacked_by_rook = is_attacked_by_rook(self, square, attacking_color);
        let attacked_by_queen = is_attacked_by_queen(self, square, attacking_color);
        let attacked_by_king = is_attacked_by_king(self, square, attacking_color);
        
        attacked_by_pawn || 
        attacked_by_knight || 
        attacked_by_bishop ||
        attacked_by_rook ||
        attacked_by_queen ||
        attacked_by_king
    }

    pub fn is_square_occupied_or_attacked(&self, square: Square, attacking_color : Color) -> bool{
       self.is_occupied(square) ||
       self.is_square_attacked(square, attacking_color)
    }
}


