use crate::piece::{Piece, PieceType, Color};
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
            return opponent_piece.color != own_piece.color
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
    new_piece(PieceType::Rook,   Color::Black), 
    new_piece(PieceType::Knight, Color::Black),
    new_piece(PieceType::Bishop, Color::Black),
    new_piece(PieceType::Queen,  Color::Black),
    new_piece(PieceType::King,   Color::Black),
    new_piece(PieceType::Bishop, Color::Black),
    new_piece(PieceType::Knight, Color::Black),
    new_piece(PieceType::Rook,   Color::Black),
],
[
    new_piece(PieceType::Pawn, Color::Black), 
    new_piece(PieceType::Pawn, Color::Black),
    new_piece(PieceType::Pawn, Color::Black),
    new_piece(PieceType::Pawn, Color::Black),
    new_piece(PieceType::Pawn, Color::Black),
    new_piece(PieceType::Pawn, Color::Black),
    new_piece(PieceType::Pawn, Color::Black),
    new_piece(PieceType::Pawn, Color::Black),
],
    [None;8],
    [None;8],
    [None;8],
    [None;8],
[
    new_piece(PieceType::Pawn, Color::White), 
    new_piece(PieceType::Pawn, Color::White),
    new_piece(PieceType::Pawn, Color::White),
    new_piece(PieceType::Pawn, Color::White),
    new_piece(PieceType::Pawn, Color::White),
    new_piece(PieceType::Pawn, Color::White),
    new_piece(PieceType::Pawn, Color::White),
    new_piece(PieceType::Pawn, Color::White),
],
[
    new_piece(PieceType::Rook,   Color::White), 
    new_piece(PieceType::Knight, Color::White),
    new_piece(PieceType::Bishop, Color::White),
    new_piece(PieceType::Queen,  Color::White),
    new_piece(PieceType::King,   Color::White),
    new_piece(PieceType::Bishop, Color::White),
    new_piece(PieceType::Knight, Color::White),
    new_piece(PieceType::Rook,   Color::White),
]];

const fn new_piece(piece_type : PieceType, color : Color) -> Option<Piece> {
    Some(Piece::new(piece_type, color))
}