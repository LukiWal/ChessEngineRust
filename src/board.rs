use crate::piece::{Piece, PieceType};
use crate::chess_square::{Square};


struct Board{
    squares : [[Option<Piece>;8];8]
}

impl Board{
    pub fn new() -> Self{
        Self{squares : STANDARD_BOARD_SETUP} 
    }

    fn is_occupied(&self, square : Square) -> bool{
        self.get_piece(square).is_some()
    }

    fn is_opponent_piece_at(&self, square : Square, own_piece : Piece) -> bool{
        if let Some(opponent_piece) = self.get_piece(square){
            return opponent_piece.is_white != own_piece .is_white
        }

        false
    }

    fn get_piece(&self, square : Square) -> Option<Piece>{
        self.squares[square.row][square.col]
    }

    fn set_piece(&mut self, square : Square, piece : Piece){
        self.squares[square.row][square.col] = Some(piece);
    }

    fn remove_piece(&mut self, square : Square){
        self.squares[square.row][square.col] = None;
    }

    fn move_piece(&mut self, from_square : Square, to_square : Square){
        if let Some(piece_to_move) = self.get_piece(from_square){
            self.set_piece(to_square, piece_to_move);
            self.remove_piece(from_square);
        } else{
            panic!("move_piece: No Piece to Move")
        }
    }

}


const STANDARD_BOARD_SETUP : [[Option<Piece>; 8]; 8] = [[
    new_piece(PieceType::Rook, false), 
    new_piece(PieceType::Knight, false),
    new_piece(PieceType::Bishop, false),
    new_piece(PieceType::Queen, false),
    new_piece(PieceType::King, false),
    new_piece(PieceType::Bishop, false),
    new_piece(PieceType::Knight, false),
    new_piece(PieceType::Rook, false),
],
[
    new_piece(PieceType::Pawn, false), 
    new_piece(PieceType::Pawn, false),
    new_piece(PieceType::Pawn, false),
    new_piece(PieceType::Pawn, false),
    new_piece(PieceType::Pawn, false),
    new_piece(PieceType::Pawn, false),
    new_piece(PieceType::Pawn, false),
    new_piece(PieceType::Pawn, false),
],
    [None;8],
    [None;8],
    [None;8],
    [None;8],
[
    new_piece(PieceType::Pawn, true), 
    new_piece(PieceType::Pawn, true),
    new_piece(PieceType::Pawn, true),
    new_piece(PieceType::Pawn, true),
    new_piece(PieceType::Pawn, true),
    new_piece(PieceType::Pawn, true),
    new_piece(PieceType::Pawn, true),
    new_piece(PieceType::Pawn, true),
],
[
    new_piece(PieceType::Rook, true), 
    new_piece(PieceType::Knight, true),
    new_piece(PieceType::Bishop, true),
    new_piece(PieceType::King, true),
    new_piece(PieceType::Queen, true),
    new_piece(PieceType::Bishop, true),
    new_piece(PieceType::Knight, true),
    new_piece(PieceType::Rook, true),
]];

const fn new_piece(piece_type : PieceType, is_white : bool) -> Option<Piece> {
    Some(Piece::new(piece_type, is_white))
}