fn main(){
    let mut board: [[Option<Piece>; 8]; 8] = [
        [
            p(PieceType::Rook, false), 
            p(PieceType::Knight, false),
            p(PieceType::Bishop, false),
            p(PieceType::Queen, false),
            p(PieceType::King, false),
            p(PieceType::Bishop, false),
            p(PieceType::Knight, false),
            p(PieceType::Rook, false),
        ],
        [
            p(PieceType::Pawn, false), 
            p(PieceType::Pawn, false),
            p(PieceType::Pawn, false),
            p(PieceType::Pawn, false),
            p(PieceType::Pawn, false),
            p(PieceType::Pawn, false),
            p(PieceType::Pawn, false),
            p(PieceType::Pawn, false),
        ],
        [None;8],
        [None;8],
        [None;8],
        [None;8],
        [
            p(PieceType::Pawn, true), 
            p(PieceType::Pawn, true),
            p(PieceType::Pawn, true),
            p(PieceType::Pawn, true),
            p(PieceType::Pawn, true),
            p(PieceType::Pawn, true),
            p(PieceType::Pawn, true),
            p(PieceType::Pawn, true),
        ],
        [
            p(PieceType::Rook, true), 
            p(PieceType::Knight, true),
            p(PieceType::Bishop, true),
            p(PieceType::King, true),
            p(PieceType::Queen, true),
            p(PieceType::Bishop, true),
            p(PieceType::Knight, true),
            p(PieceType::Rook, true),
        ]
    ];

    
}


#[derive(Clone, Copy)]
struct Piece{
    kind : PieceType,
    is_white : bool,
}

#[derive(Clone, Copy)]
enum PieceType{
    King,
    Queen,
    Rook,
    Knight,
    Bishop,
    Pawn
}

fn p (kind : PieceType, is_white : bool) -> Option<Piece> {
    Some(Piece{kind, is_white})
}