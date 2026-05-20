use crate::chess_move::Move;
use crate::game::Game;
use crate::chess_square::Square;
use crate::debug::{log_debug, log_value};

fn generate_sliding_moves(game : &Game, from_square : Square, offsets : Vec<(isize, isize)> ) -> Vec<Move>{
    let Some(piece) = game.get_piece(from_square) else {
        return Vec::new();
    };


    let mut legal_moves: Vec<Move> = Vec::new();


    for (row_direction, col_direction) in offsets{
        let mut row = from_square.row  as isize + row_direction;
        let mut col = from_square.col as isize + col_direction;
        let mut counter = 1;

        while let Some(to_square) = Square::new(row, col){
            if game.is_occupied_by_opponent_color(to_square, piece.color){
                legal_moves.push(Move::new_capture(from_square, to_square));
                break;
            } else if !game.is_occupied(to_square) {
                legal_moves.push(Move::new_normal(from_square, to_square));
            } else {
                break;
            }

            row += row_direction;
            col += col_direction;
        
        }
    }
    
    legal_moves
}

pub fn generate_rook_moves(game : &Game, from_square : Square) -> Vec<Move>{
    let offsets:Vec<(isize, isize)>  = vec![
        (1,0),(-1,0),
        (0,1),(0,-1)
    ];

    generate_sliding_moves(game, from_square, offsets)
}

pub fn generate_bishop_moves(game : &Game, from_square : Square) -> Vec<Move>{
    let offsets:Vec<(isize, isize)>  = vec![
        (-1,-1),(-1,1),
        (1,1),(1,-1)
    ];

    generate_sliding_moves(game, from_square, offsets)
}

pub fn generate_queen_moves(game : &Game, from_square : Square) -> Vec<Move>{
    let offsets:Vec<(isize, isize)>  = vec![
        (-1,-1),(-1,1),
        (1,1),(1,-1), 
        (1,0),(-1,0),
        (0,1),(0,-1)
    ];

    generate_sliding_moves(game, from_square, offsets)
}