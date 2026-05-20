use crate::chess_move::Move;
use crate::game::Game;
use crate::chess_square::Square;
use crate::debug::{log_debug, log_value};

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