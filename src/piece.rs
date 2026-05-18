#[derive(Clone, Copy, Debug)]
pub struct Piece{
    pub kind : PieceType,
    pub is_white : bool,
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