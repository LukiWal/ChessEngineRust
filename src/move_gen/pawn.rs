use crate::piece::{PieceType, Color};
use crate::chess_move::{Move};
use crate::chess_square::Square;
use crate::game::Game;
use crate::constants::{ROW_DOWN, ROW_UP, COL_LEFT, COL_RIGHT};
use crate::board::Board;

pub fn generate_pawn_moves(game : &Game , square : Square) -> Vec<Move>{
    let mut legal_moves: Vec<Move> = Vec::new();
    
    
    if let Some(piece) = game.get_piece(square) {
       
        let direction : isize = if piece.color == Color::White {-1} else {1}; 
        let start_row = if piece.color == Color::White{6} else{1};
        let promotion_row = if piece.color == Color::White{0} else {7};
        let row = square.row as isize;
        let col = square.col  as isize;

        let Some(from_square) = Square::new(row, col) else {
            return legal_moves;
        };

        let move_1sqr_forward = Square::new(row + direction, col);
        let move_2sqrs_forward = Square::new(row  + 2 * direction, col);
        let capture_left = Square::new(row + direction, col - 1); 
        let capture_right = Square::new(row + direction, col + 1);
                
        if let Some(move_1sqr_forward) = move_1sqr_forward {
            if !game.is_occupied(move_1sqr_forward){
                if move_1sqr_forward.row != promotion_row{
                    legal_moves.push(Move { from_square: from_square, to_square: move_1sqr_forward, is_capture: false, promotion: None, is_en_passant : false, is_castle: false });
                } else{
                    legal_moves.extend(generate_promotion_moves(from_square, move_1sqr_forward, false));
                }

                if let Some(move_2sqrs_forward) = move_2sqrs_forward{
                    if !game.is_occupied(move_2sqrs_forward) && from_square.row == start_row{
                        legal_moves.push(Move { from_square: from_square, to_square: move_2sqrs_forward, is_capture: false, promotion: None, is_en_passant : false, is_castle: false  });
                    }
                }    
            }     
        }   


        for square in [capture_left, capture_right]{
            if let Some(square) = square{
                if game.is_occupied_by_opponent_color(square, piece.color) {
                    if square.row != promotion_row{
                        legal_moves.push(Move { from_square: from_square, to_square: square, is_capture: true, promotion: None, is_en_passant: false, is_castle: false  });
                    } else{
                        legal_moves.extend(generate_promotion_moves(from_square, square, true));
                    }
                }

                if let Some (en_passant) = game.en_passant{
                    let en_passant_capture = Square::new(en_passant.row as isize + direction , en_passant.col as isize);

                    if let Some(en_passant_capture) = en_passant_capture{

                        if square == en_passant_capture{
                                legal_moves.push(Move { from_square, to_square: en_passant_capture, is_capture: true, promotion: None, is_en_passant : true, is_castle: false  });
                            }
                        
                    }
                    
                }
            }
        }
    } 

    legal_moves
   
}

fn generate_promotion_moves(from_square : Square, to_square : Square, is_capture : bool) -> Vec<Move>{
    let mut promotion_moves : Vec<Move> = Vec::new();

    for piece_type in [PieceType::Queen, PieceType::Rook, PieceType::Knight, PieceType::Bishop]{
        promotion_moves.push(Move { from_square, to_square, is_capture, promotion: Some(piece_type), is_en_passant : false, is_castle: false });
    }

    promotion_moves
}


pub fn is_attacked_by_pawn(board : &Board, square : Square, attacking_color : Color) -> bool{
        let direction = if attacking_color == Color::White {ROW_DOWN} else {ROW_UP};
        let offsets: [(isize, isize); 2] = [(direction, COL_LEFT), (direction, COL_RIGHT)];

        for (row_direction, col_direction) in offsets{
            let row = square.row as isize + row_direction;
            let col = square.col as isize + col_direction;
    
            if let Some(attacking_square) = Square::new(row, col){
                if board.get_piece(attacking_square).is_some_and(|attacking_piece| attacking_piece.color == attacking_color && attacking_piece.piece_type == PieceType::Pawn) {return true;}
            }
        }
        false
    }