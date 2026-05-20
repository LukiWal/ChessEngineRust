mod test_data;

use rust_chess_engine::{chess_move::Move, game::Game, piece::Color};
use test_data::{
    UCI_MOVES, 
    POSITION_SIDE,
    PROMOTION_TEST_CASES, 
    PROMOTION_MOVE_GENERATION_TEST_CASES, 
    MAX_PROMOTION_MOVE_GENERATION_TEST_CASE_12, 
    EN_PASSANT_MOVE_GENERATION_TEST_CASES, 
    EN_PASSANT_APPLY_TEST_CASES
}; 



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

    let mut game = Game::new();
    game.apply_position_from_startpos_uci(test_uci);

    assert!(game.board[6][4].is_none());
    assert!(game.board[4][4].is_some());
    assert_eq!(game.color_to_move, Color::Black);
}

#[test]
fn position_sets_correct_side_to_move(){
    let mut game = Game::new();

    for uci_string in POSITION_SIDE{
        game.apply_position_from_startpos_uci(&uci_string.0);
        assert_eq!(game.color_to_move, uci_string.1);
    }
}

#[test]
fn promotion_move_generation_4(){
    let mut game = Game::new();

    for case in PROMOTION_MOVE_GENERATION_TEST_CASES{
        game.apply_position_from_startpos_uci(case.position);
        let generated_moves: Vec<Move> = game.generate_all_legal_moves();

        for expected_move in case.moves{
            assert!(
                generated_moves.contains(&expected_move),
                "Expected promotion move missing: {:#?}\nPosition: {}\nGenerated moves: {:#?}",
                expected_move,
                case.position,
                generated_moves
            );
        }
    }
}

#[test]
fn promotion_move_generation_12(){
    let mut game = Game::new();

    let case = MAX_PROMOTION_MOVE_GENERATION_TEST_CASE_12;
    game.apply_position_from_startpos_uci(case.position);
    let generated_moves: Vec<Move> = game.generate_all_legal_moves();

    for expected_move in case.moves{
        assert!(
            generated_moves.contains(&expected_move),
            "Expected promotion move missing: {:#?}\nPosition: {}\nGenerated moves: {:#?}",
            expected_move,
            case.position,
            generated_moves
        );
    }
    
}

#[test]
fn applying_promotion(){
    let mut game = Game::new();

    for promotion_test_case in PROMOTION_TEST_CASES{
        game.apply_position_from_startpos_uci(promotion_test_case.position);
        assert_eq!(game.get_piece_at_square(promotion_test_case.target_square), Some(promotion_test_case.expected_piece));
    }
} 

#[test]
fn en_passant_move_generation() {
    for case in EN_PASSANT_MOVE_GENERATION_TEST_CASES {
        let mut game = Game::new();

        game.apply_position_from_startpos_uci(case.position);

        let generated_moves = game.generate_all_legal_moves();

        for expected_move in case.moves {
            assert!(
                generated_moves.contains(&expected_move),
                "Expected en passant move missing: {:#?}\nPosition: {}\nGenerated moves: {:#?}",
                expected_move,
                case.position,
                generated_moves
            );
        }
    }
}

#[test]
fn applying_en_passant_removes_captured_pawn() {
    for case in EN_PASSANT_APPLY_TEST_CASES {
        let mut game = Game::new();

        game.apply_position_from_startpos_uci(case.position);

        assert_eq!(
            game.get_piece_at_square(case.capturing_pawn_square),
            Some(case.expected_piece),
            "capturing pawn missing/wrong after en passant.\nPosition: {}",
            case.position
        );

        assert_eq!(
            game.get_piece_at_square(case.captured_pawn_square),
            None,
            "captured pawn was not removed after en passant.\nPosition: {}",
            case.position
        );
    }
}