use rust_chess_engine::{chess_move::Move, game::Game, chess_square::Square, piece::Piece, piece::PieceType};

pub const UCI_MOVES : &[&str] = &[
    // pawns
    "e2e4", "d2d4", "a2a3", "h2h4", "e7e5", "d7d5", "a7a6", "h7h5",
    // knights
    "g1f3", "g1h3", "b1a3", "b1c3", "g8f6", "g8h6", "b8a6", "b8c6",
    // bishops
    "f1e2", "f1d3", "f1c4", "f1b5", "f1a6",
    "c1d2", "c1e3", "c1f4", "c1g5", "c1h6",
    "f8e7", "f8d6", "f8c5", "f8b4", "f8a3",
    // rooks
    "a1a2", "a1a3", "a1a4", "h1h2", "h1h3", "h1h4",
    "a8a7", "a8a6", "a8a5", "h8h7", "h8h6", "h8h5",
    // queen / king / castling
    "d1d2", "d1d3", "d1h5", "d1a4", "d8d7", "d8d6", "d8h4", "d8a5",
    "e1e2", "e1d1", "e1f1", "e1d2", "e1f2",
    "e8e7", "e8d8", "e8f8", "e8d7", "e8f7",
    "e1g1", "e1c1", "e8g8", "e8c8",
    // captures
    "e4d5", "e4f5", "d5e4", "d5c4", "c4d5",
    "f4e5", "g5h6", "b5a6", "c6d4", "f6e4",
    // promotions
    "a7a8q", "a7a8r", "a7a8b", "a7a8n",
    "h7h8q", "h7h8r", "h7h8b", "h7h8n",
    "a2a1q", "a2a1r", "a2a1b", "a2a1n",
    "h2h1q", "h2h1r", "h2h1b", "h2h1n",
    // promotion captures
    "a7b8q", "a7b8r", "a7b8b", "a7b8n",
    "h7g8q", "h7g8r", "h7g8b", "h7g8n",
    "a2b1q", "a2b1r", "a2b1b", "a2b1n",
    "h2g1q", "h2g1r", "h2g1b", "h2g1n",
    // en passant format
    "e5d6", "d5e6", "e4d3", "d4e3",
];

pub const POSITION_SIDE : &[(&str, bool)] = &[
    ("position startpos", true),
    ("position startpos moves e2e4", false),
    ("position startpos moves e2e4 e7e5", true),
    ("position startpos moves e2e4 e7e5 g1f3", false),
    ("position startpos moves h2h3 c7c5 b2b4 c5b4 a2a3 b4a3 g2g3 a3a2 e2e4 a2b1b", true),
    ("position startpos moves e2e3 a7a5 f2f4 d7d5 h2h4 c7c5 c2c4 d5c4 a2a3 e7e5 f4e5 g7g6 b2b4 c4b3 h4h5 g6h5 e3e4 c5c4 g2g3 b7b5 d2d4 c4d3 a3a4 b5a4 e5e6 f7e6 g3g4 h5g4 e4e5 g4g3", true),
];

pub struct PromotionTestCase {
    pub position: &'static str,
    pub target_square: Square,
    pub expected_piece: Piece,
}

pub const PROMOTION_TEST_CASES : [PromotionTestCase;4] = [
    // White non-capture promotion: a7 -> a8 = Queen
    PromotionTestCase {
        position: "position startpos moves a2a4 h7h6 a4a5 h6h5 a5a6 h5h4 a6a7 a8b8 a7a8q",
        target_square: Square { row: 0, col: 0 }, // a8
        expected_piece: Piece {
            kind: PieceType::Queen,
            is_white: true,
        },
    },

    // White capture promotion: g7xh8 = Knight
    PromotionTestCase {
        position: "position startpos moves g2g4 a7a6 g4g5 a6a5 g5g6 a5a4 g6g7 b7b6 g7h8n",
        target_square: Square { row: 0, col: 7 }, // h8
        expected_piece: Piece {
            kind: PieceType::Knight,
            is_white: true,
        },
    },

    // Black non-capture promotion: h2 -> h1 = Rook
    PromotionTestCase {
        position: "position startpos moves g1f3 h7h5 a2a3 h5h4 a3a4 h4h3 a4a5 h3h2 h1g1 h2h1r",
        target_square: Square { row: 7, col: 7 }, // h1
        expected_piece: Piece {
            kind: PieceType::Rook,
            is_white: false,
        },
    },

    // Black capture promotion: a2xb1 = Bishop
    PromotionTestCase {
        position: "position startpos moves h2h3 c7c5 b2b4 c5b4 a2a3 b4a3 g2g3 a3a2 e2e4 a2b1b",
        target_square: Square { row: 7, col: 1 }, // b1
        expected_piece: Piece {
            kind: PieceType::Bishop,
            is_white: false,
        },
    },
];

pub struct PromotionMoveGenerationTestCase {
    pub position: &'static str,
    pub moves : [Move;4]
}