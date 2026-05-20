#[derive(Clone, Copy, Debug)]
#[derive(PartialEq, Eq)]
pub struct Piece{
    pub piece_type : PieceType,
    pub is_white : bool,
}


impl Piece{
    pub fn new(piece_type : PieceType, is_white: bool) -> Self{
        Self{
            piece_type,
            is_white
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
