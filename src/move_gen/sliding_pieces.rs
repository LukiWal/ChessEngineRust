use crate::chess_move::Move;
use crate::constants::{BISHOP_OFFSETS, ROOK_OFFSETS, QUEEN_OFFSETS};
use crate::game::Game;
use crate::chess_square::Square;
use crate::debug::{log_debug, log_value};
use crate::board::Board;
use crate::piece::{PieceType, Color};

pub fn generate_rook_moves(game : &Game, from_square : Square) -> Vec<Move>{
    generate_sliding_moves(game, from_square, ROOK_OFFSETS)
}

pub fn generate_bishop_moves(game : &Game, from_square : Square) -> Vec<Move>{
    generate_sliding_moves(game, from_square, BISHOP_OFFSETS)
}

pub fn generate_queen_moves(game : &Game, from_square : Square) -> Vec<Move>{
    generate_sliding_moves(game, from_square, QUEEN_OFFSETS)
}

pub fn is_attacked_by_rook(board : &Board, square : Square, attacking_color : Color) -> bool{  
    is_attacked_by_sliding_piece(&board, square, attacking_color, ROOK_OFFSETS, PieceType::Rook)
}

pub fn is_attacked_by_bishop(board : &Board, square : Square, attacking_color : Color) -> bool{  
    is_attacked_by_sliding_piece(&board, square, attacking_color, BISHOP_OFFSETS, PieceType::Bishop)
}

pub fn is_attacked_by_queen(board : &Board, square : Square, attacking_color : Color) -> bool{  
    is_attacked_by_sliding_piece(&board, square, attacking_color, QUEEN_OFFSETS, PieceType::Queen)
}

fn generate_sliding_moves(game : &Game, from_square : Square, offsets : &[(isize,isize)] ) -> Vec<Move>{
    let Some(piece) = game.get_piece(from_square) else {
        return Vec::new();
    };


    let mut legal_moves: Vec<Move> = Vec::new();


    for (row_direction, col_direction) in offsets{
        let mut row = from_square.row  as isize + row_direction;
        let mut col = from_square.col as isize + col_direction;

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

fn is_attacked_by_sliding_piece(board : &Board, square : Square, attacking_color : Color, offsets : &[(isize,isize)], attacking_piece_type : PieceType) -> bool{  
    for &(row_direction, col_direction) in offsets{
        let mut row = square.row  as isize + row_direction;
        let mut col = square.col as isize + col_direction;

        while let Some(attacking_square) = Square::new(row, col){
            if board.is_occupied(attacking_square){
                if board.get_piece(attacking_square).is_some_and(|attacking_piece| attacking_piece.color == attacking_color && attacking_piece.piece_type == attacking_piece_type) {return true;}
                break;
            }
            row += row_direction;
            col += col_direction;
        }
    }
    
    false
}