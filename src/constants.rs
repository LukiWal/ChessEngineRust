use crate::piece::{Piece, PieceType, Color, };
use crate::chess_square::{Square};

pub const WHITE_KING_STARTING_SQUARE : Square = Square{row: 7, col: 4};

pub const ROOK_QUEENSIDE_STARTING_COL : usize = 0;
pub const ROOK_KINGSIDE_STARTING_COL : usize = 7;

pub const ROOK_QUEENSIDE_CASTLING_COL : usize = 3;
pub const ROOK_KINGSIDE_CASTLING_COL : usize = 5;

pub const KING_SHORT_CASTLING_COL : usize = 6;
pub const KING_LONG_CASTLING_COL : usize = 2;

pub const ROW_UP : isize = -1;
pub const ROW_DOWN : isize = 1;
pub const ROW_NEUTRAL : isize = 0;
pub const COL_LEFT : isize = -1;
pub const COL_RIGHT : isize = 1;
pub const COL_NEUTRAL : isize = 0;

pub const KNIGHT_OFFSETS : [(isize, isize); 8] = 
[
    (2,-1),(2,1),
    (1,2) ,(-1,2),
    (-2,1),(-2,-1),
    (1,-2),(-1,-2)
];



pub const DIRECTION_UP : (isize, isize) = (ROW_UP, COL_NEUTRAL);
pub const DIRECTION_UP_RIGHT : (isize, isize) = (ROW_UP, COL_RIGHT);
pub const DIRECTION_RIGHT : (isize, isize) = (ROW_NEUTRAL, COL_RIGHT);
pub const DIRECTION_DOWN_RIGHT : (isize, isize) = (ROW_DOWN, COL_RIGHT);
pub const DIRECTION_DOWN : (isize, isize) = (ROW_DOWN, COL_NEUTRAL);
pub const DIRECTION_DOWN_LEFT : (isize, isize) = (ROW_DOWN, COL_LEFT);
pub const DIRECTION_LEFT : (isize, isize) = (ROW_NEUTRAL, COL_LEFT);
pub const DIRECTION_UP_LEFT : (isize, isize) = (ROW_UP, COL_LEFT);




pub const ROOK_OFFSETS :  &[(isize,isize)] = 
&[
    DIRECTION_UP,
    DIRECTION_RIGHT,
    DIRECTION_DOWN,
    DIRECTION_LEFT,
];

pub const BISHOP_OFFSETS :  &[(isize,isize)] = 
&[
    DIRECTION_UP_RIGHT,
    DIRECTION_DOWN_RIGHT,
    DIRECTION_DOWN_LEFT,
    DIRECTION_UP_LEFT
];

pub const QUEEN_OFFSETS :  &[(isize,isize)] = 
&[
    DIRECTION_UP,
    DIRECTION_UP_RIGHT,
    DIRECTION_RIGHT,
    DIRECTION_DOWN_RIGHT,
    DIRECTION_DOWN,
    DIRECTION_DOWN_LEFT,
    DIRECTION_LEFT,
    DIRECTION_UP_LEFT
];

pub const KING_OFFSETS : [(isize,isize);8] = 
[
    DIRECTION_UP,
    DIRECTION_UP_RIGHT,
    DIRECTION_RIGHT,
    DIRECTION_DOWN_RIGHT,
    DIRECTION_DOWN,
    DIRECTION_DOWN_LEFT,
    DIRECTION_LEFT,
    DIRECTION_UP_LEFT
];

pub const STANDARD_BOARD_SETUP : [[Option<Piece>; 8]; 8] = [[
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
