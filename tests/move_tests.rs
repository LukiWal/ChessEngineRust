mod test_data;

use rust_chess_engine::{chess_move::Move, game::Game, piece::Piece};
use test_data::{UCI_MOVES, POSITION_SIDE, PROMOTION_TEST_CASES}; 
use rust_chess_engine::piece::PieceType;


#[test]
fn test_uci_to_move_and_back(){
    for uci_move_orginal in UCI_MOVES{
        let chess_move = Move::tranlate_uci_to_move(uci_move_orginal);
        let uci_move_translated = chess_move.translate_move_to_uci();
        assert_eq!(*uci_move_orginal, uci_move_translated);
    } 
}

#[test]
fn applying_position_e2e4_updates_board_and_turn(){
    let test_uci = "position startpos moves e2e4";

    let mut game = Game{board : Game::initialize_board(), en_passant: None, white_to_move : false};
    game.apply_position_from_startpos_uci(test_uci);

    assert!(game.board[6][4].is_none());
    assert!(game.board[4][4].is_some());
    assert_eq!(game.white_to_move, false);
}

#[test]
fn position_sets_correct_side_to_move(){
    let mut game = Game{board : Game::initialize_board(), en_passant: None, white_to_move : false};

    for uci_string in POSITION_SIDE{
        game.apply_position_from_startpos_uci(&uci_string.0);
        assert_eq!(game.white_to_move, uci_string.1);
    }
}

#[test]
fn promotion_move_generation(){
    let mut game = Game{board : Game::initialize_board(), en_passant: None, white_to_move : false};

    for promotion_test_case in PROMOTION_TEST_CASES{
        game.apply_position_from_startpos_uci(promotion_test_case.position);
        let legal_moves = game.generate_all_legal_moves();
        
        assert_eq!(game.get_piece_at_square(promotion_test_case.target_square), Some(promotion_test_case.expected_piece));
    }
}

#[test]
fn applying_promotion(){
    let mut game = Game{board : Game::initialize_board(), en_passant: None, white_to_move : false};

    for promotion_test_case in PROMOTION_TEST_CASES{
        game.apply_position_from_startpos_uci(promotion_test_case.position);
        assert_eq!(game.get_piece_at_square(promotion_test_case.target_square), Some(promotion_test_case.expected_piece));
    }
} 