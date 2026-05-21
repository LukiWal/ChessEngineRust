use crate::constants::{ROW_DOWN, ROW_UP, COL_LEFT, COL_RIGHT, KNIGHT_OFFSETS};
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
        self.is_attacked_by_pawn(square, attacking_color) || self.is_attacked_by_knight(square, attacking_color)
    }

    fn is_attacked_by_pawn(&self, square : Square, attacking_color : Color) -> bool{
        let direction = if attacking_color == Color::White {ROW_UP} else {ROW_DOWN};
        let offsets: [(isize, isize); 2] = [(direction, COL_LEFT), (direction, COL_RIGHT)];

        for (row_direction, col_direction) in offsets{
            let row = square.row as isize + row_direction;
            let col = square.col as isize + col_direction;
    
            if let Some(attacking_square) = Square::new(row, col){
                if self.get_piece(attacking_square).is_some_and(|attacking_piece| attacking_piece.color == attacking_color && attacking_piece.piece_type == PieceType::Pawn) {return true;}
            }
        }
        false
    }

    fn is_attacked_by_knight(&self, square : Square, attacking_color : Color) -> bool{
        for (row_direction, col_direction) in KNIGHT_OFFSETS{
            let attacking_square : Option<Square> = Square::new(square.row as isize + row_direction, square.col as isize + col_direction);

            if let Some(attacking_square) = attacking_square{
                if self.get_piece(attacking_square).is_some_and(|attacking_piece| attacking_piece.color == attacking_color && attacking_piece.piece_type == PieceType::Knight) {return true;}
            }
        }

        false
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