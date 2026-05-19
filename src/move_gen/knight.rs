use crate::chess_move::Move;
use crate::debug::log_value;
use crate::game::Game;
use crate::chess_square::Square;


pub fn generate_knight_moves(game : &Game, square : Square) -> Vec<Move>{
    let mut legal_moves: Vec<Move> = Vec::new();


    let offsets: [(isize, isize); 8] = [
        (2,-1),(2,1),
        (1,2) ,(-1,2),
        (-2,1),(-2,-1),
        (1,-2),(-1,-2)
    ];

    for offset in offsets{
        let to_square : Option<Square> = Square::new(square.row as isize + offset.0, square.col as isize + offset.1);

        if let Some(to_square) = to_square{
            if to_square.check_if_square_is_capturable(game){
                legal_moves.push(Move { from_square: square, to_square: to_square, is_capture: true, promotion: None, is_en_passant: false });
            } else if !to_square.check_if_square_is_occupied(&game.board){
                legal_moves.push(Move { from_square: square, to_square: to_square, is_capture: false, promotion: None, is_en_passant: false });
            }
        }
    }
    log_value("label", &legal_moves);
    legal_moves
   
}