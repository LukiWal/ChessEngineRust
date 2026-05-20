#[derive(Clone, Copy, Debug)]
#[derive(PartialEq, Eq)]
pub struct Piece{
    pub piece_type : PieceType,
    pub color : Color,
}


impl Piece{
    pub const fn new(piece_type : PieceType, color: Color) -> Self{
        Self{
            piece_type,
            color
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[derive(PartialEq, Eq)]
pub enum PieceType{
    King,
    Queen,
    Rook,
    Knight,
    Bishop,
    Pawn
}

#[derive(Clone, Copy, Debug)]
#[derive(PartialEq, Eq)]
pub enum Color{
    White,
    Black
}

impl Color{
    pub fn opposite(self) -> Self{
        match self{
            Color::White => Color::Black,
            Color::Black => Color::White
        }
           
    }
}