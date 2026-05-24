use crate::constants::{KING_OFFSETS};
use crate::chess_move::Move;
use crate::game::Game;
use crate::chess_square::Square;
use crate::piece::{PieceType, Color};
use crate::board::Board;
use crate::debug::{log_debug, log_value};

pub fn generate_king_moves(game : &Game, from_square : Square) -> Vec<Move>{
    let Some(piece) = game.get_piece(from_square) else {
        return Vec::new();
    };

    let mut legal_moves: Vec<Move> = Vec::new();

    for (row_direction, col_direction) in KING_OFFSETS{
        let to_square : Option<Square> = Square::new(from_square.row as isize + row_direction, from_square.col as isize + col_direction);

        if let Some(to_square) = to_square{
            if game.is_occupied_by_opponent_color(to_square, piece.color){
                legal_moves.push(Move::new_capture(from_square, to_square));
            } else if !game.is_occupied(to_square){
                legal_moves.push(Move::new_normal(from_square, to_square));
            }
        }
    }

    if game.color_to_move == Color::White{
        if game.catling_rights.white_kingside{

        }

        if game.catling_rights.white_kingside{
            let white_bishop_starting_square = Square{row: 7, col: 5};
            let white_knight_starting_square = Square{row: 7, col: 6};
            let white_king_rook_starting_square = Square{row: 7, col: 7};
            let white_king_starting_square = Square{row: 7, col: 4};

            if !game.is_occupied(white_bishop_starting_square) && !game.is_square_attacked(white_bishop_starting_square, Color::Black) &&
            !game.is_occupied(white_knight_starting_square) && !game.is_square_attacked(white_knight_starting_square, Color::Black){
                legal_moves.push(Move::new_castle(white_king_starting_square, white_knight_starting_square))
            }
        }
    }

    legal_moves
}

pub fn is_attacked_by_king(board : &Board, square : Square, attacking_color : Color) -> bool{  
    for (row_direction, col_direction) in KING_OFFSETS{    
        let row = square.row as isize + row_direction;
        let col = square.col as isize + col_direction;

        if let Some(attacking_square) = Square::new(row, col){
            if board.get_piece(attacking_square).is_some_and(|attacking_piece| attacking_piece.color == attacking_color && attacking_piece.piece_type == PieceType::King) {return true;}
        }
    }
    false
}