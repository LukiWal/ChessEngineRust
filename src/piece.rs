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

#[derive(Clone, Copy, Debug)]
#[derive(PartialEq, Eq)]
pub struct CastlingRights{
    pub white_kingside : bool,
    pub white_queenside : bool,
    pub black_kingside : bool,
    pub black_queenside : bool
}


impl CastlingRights{
    pub fn new() -> Self{
        Self { white_kingside: true, white_queenside: true, black_kingside: true, black_queenside: true }
    }
}