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
            const WHITE_KINGSIDE_BISHOP_STARTING_SQUARE : Square = Square{row: 7, col: 5};
            const WHITE_KINGSIDE_KNIGHT_STARTING_SQUARE : Square = Square{row: 7, col: 6};
            const WHITE_KING_STARTING_SQUARE : Square = Square{row: 7, col: 4} ;  
          
            if !game.is_square_occupied_or_attacked(WHITE_KINGSIDE_BISHOP_STARTING_SQUARE, Color::Black) && 
               !game.is_square_occupied_or_attacked(WHITE_KINGSIDE_KNIGHT_STARTING_SQUARE, Color::Black) &&
               !game.is_king_in_check(Color::White){
                    legal_moves.push(Move::new_castle(WHITE_KING_STARTING_SQUARE, WHITE_KINGSIDE_KNIGHT_STARTING_SQUARE));
            } 
        }

          if game.catling_rights.white_queenside{
            const WHITE_QUEENSIDE_QUEEN_STARTING_SQUARE : Square = Square{row: 7, col: 3};
            const WHITE_QUEENSIDE_BISHOP_STARTING_SQUARE : Square = Square{row: 7, col: 2};
            const WHITE_KING_STARTING_SQUARE : Square = Square{row: 7, col: 4} ;  
          
            if !game.is_square_occupied_or_attacked(WHITE_QUEENSIDE_QUEEN_STARTING_SQUARE, Color::Black) && 
               !game.is_square_occupied_or_attacked(WHITE_QUEENSIDE_BISHOP_STARTING_SQUARE, Color::Black) &&
               !game.is_king_in_check(Color::White){
                    legal_moves.push(Move::new_castle(WHITE_KING_STARTING_SQUARE, WHITE_QUEENSIDE_BISHOP_STARTING_SQUARE));
            } 
        }
    }

     if game.color_to_move == Color::Black{
        if game.catling_rights.black_kingside{
            const BLACK_KINGSIDE_BISHOP_STARTING_SQUARE : Square = Square{row: 0, col: 5};
            const BLACK_KINGSIDE_KNIGHT_STARTING_SQUARE : Square = Square{row: 0, col: 6};
            const BLACK_KING_STARTING_SQUARE : Square = Square{row: 0, col: 4} ;  
          
            if !game.is_square_occupied_or_attacked(BLACK_KINGSIDE_BISHOP_STARTING_SQUARE, Color::Black) && 
               !game.is_square_occupied_or_attacked(BLACK_KINGSIDE_KNIGHT_STARTING_SQUARE, Color::Black) &&
               !game.is_king_in_check(Color::Black){
                    legal_moves.push(Move::new_castle(BLACK_KING_STARTING_SQUARE, BLACK_KINGSIDE_KNIGHT_STARTING_SQUARE));
            } 
        }

          if game.catling_rights.black_queenside{
            const BLACK_QUEENSIDE_QUEEN_STARTING_SQUARE : Square = Square{row: 0, col: 3};
            const BLACK_QUEENSIDE_BISHOP_STARTING_SQUARE : Square = Square{row: 0, col: 2};
            const BLACK_KING_STARTING_SQUARE : Square = Square{row: 0, col: 4} ;  
          
            if !game.is_square_occupied_or_attacked(BLACK_QUEENSIDE_QUEEN_STARTING_SQUARE, Color::Black) && 
               !game.is_square_occupied_or_attacked(BLACK_QUEENSIDE_BISHOP_STARTING_SQUARE, Color::Black) &&
               !game.is_king_in_check(Color::Black){
                    legal_moves.push(Move::new_castle(BLACK_KING_STARTING_SQUARE, BLACK_QUEENSIDE_BISHOP_STARTING_SQUARE));
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