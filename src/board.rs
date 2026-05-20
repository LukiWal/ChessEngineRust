use crate::piece::{Piece, PieceType, Color};
use crate::chess_square::{Square};


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