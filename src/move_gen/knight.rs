use crate::chess_move::Move;
use crate::game::Game;
use crate::chess_square::Square;
use crate::debug::{log_debug, log_value};
use crate::constants::{KNIGHT_OFFSETS};
use crate::piece::{PieceType, Color};
use crate::board::Board;

pub fn generate_knight_moves(game : &Game, from_square : Square) -> Vec<Move>{
    let Some(piece) = game.get_piece(from_square) else {
        return Vec::new();
    };


    let mut legal_moves: Vec<Move> = Vec::new();

    let offsets: [(isize, isize); 8] = [
        (2,-1),(2,1),
        (1,2) ,(-1,2),
        (-2,1),(-2,-1),
        (1,-2),(-1,-2)
    ];

    for (row_direction, col_direction) in offsets{
        let to_square : Option<Square> = Square::new(from_square.row as isize + row_direction, from_square.col as isize + col_direction);

        if let Some(to_square) = to_square{
            if game.is_occupied_by_opponent_color(to_square, piece.color){
                legal_moves.push(Move::new_capture(from_square, to_square));
            } else if !game.is_occupied(to_square){
                legal_moves.push(Move::new_normal(from_square, to_square));
            }
        }
    }

    legal_moves
}

 pub fn is_attacked_by_knight(board : &Board, square : Square, attacking_color : Color) -> bool{
        for (row_direction, col_direction) in KNIGHT_OFFSETS{
            let attacking_square : Option<Square> = Square::new(square.row as isize + row_direction, square.col as isize + col_direction);

            if let Some(attacking_square) = attacking_square{
                if board.get_piece(attacking_square).is_some_and(|attacking_piece| attacking_piece.color == attacking_color && attacking_piece.piece_type == PieceType::Knight) {return true;}
            }
        }
        false
    }
