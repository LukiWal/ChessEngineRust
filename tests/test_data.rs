use rust_chess_engine::{chess_move::Move, chess_square::Square, piece::Piece, piece::PieceType, piece::Color};

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

pub const POSITION_SIDE : &[(&str, Color)] = &[
    ("position startpos", Color::White),
    ("position startpos moves e2e4", Color::Black),
    ("position startpos moves e2e4 e7e5", Color::White),
    ("position startpos moves e2e4 e7e5 g1f3", Color::Black),
    ("position startpos moves h2h3 c7c5 b2b4 c5b4 a2a3 b4a3 g2g3 a3a2 e2e4 a2b1b", Color::White),
    ("position startpos moves e2e3 a7a5 f2f4 d7d5 h2h4 c7c5 c2c4 d5c4 a2a3 e7e5 f4e5 g7g6 b2b4 c4b3 h4h5 g6h5 e3e4 c5c4 g2g3 b7b5 d2d4 c4d3 a3a4 b5a4 e5e6 f7e6 g3g4 h5g4 e4e5 g4g3", Color::White),
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
        expected_piece: Piece::new(PieceType::Queen, Color::White)
    },

    // White capture promotion: g7xh8 = Knight
    PromotionTestCase {
        position: "position startpos moves g2g4 a7a6 g4g5 a6a5 g5g6 a5a4 g6g7 b7b6 g7h8n",
        target_square: Square { row: 0, col: 7 }, // h8
        expected_piece: Piece::new(PieceType::Knight, Color::White)
    },

    // Black non-capture promotion: h2 -> h1 = Rook
    PromotionTestCase {
        position: "position startpos moves g1f3 h7h5 a2a3 h5h4 a3a4 h4h3 a4a5 h3h2 h1g1 h2h1r",
        target_square: Square { row: 7, col: 7 }, // h1
        expected_piece: Piece::new(PieceType::Rook, Color::Black)
    },

    // Black capture promotion: a2xb1 = Bishop
    PromotionTestCase {
        position: "position startpos moves h2h3 c7c5 b2b4 c5b4 a2a3 b4a3 g2g3 a3a2 e2e4 a2b1b",
        target_square: Square { row: 7, col: 1 }, // b1
        expected_piece: Piece::new(PieceType::Bishop, Color::Black)
    },
];

pub struct PromotionMoveGenerationTestCase {
    pub position: &'static str,
    pub moves : [Move;4]
}

pub const PROMOTION_MOVE_GENERATION_TEST_CASES: [PromotionMoveGenerationTestCase; 4] = [
    // White non-capture promotion: e7 -> e8
    // e8, d8, f8 wurden vorher "freigeräumt", damit nur e7e8 Promotions entstehen.
    PromotionMoveGenerationTestCase {
        position: "position startpos moves e2e4 e8e7 e4e5 d8d7 e5e6 f8f7 e6e7 a7a6",
        moves: [
            Move::new_promotion(Square { row: 1, col: 4 }, Square { row: 0, col: 4 }, false, PieceType::Queen),
            Move::new_promotion(Square { row: 1, col: 4 }, Square { row: 0, col: 4 }, false, PieceType::Rook),
            Move::new_promotion(Square { row: 1, col: 4 }, Square { row: 0, col: 4 }, false, PieceType::Bishop),
            Move::new_promotion(Square { row: 1, col: 4 }, Square { row: 0, col: 4 }, false, PieceType::Knight),
        ],
    },

    // White non-capture promotion: h7 -> h8
    // h8 und g8 wurden freigeräumt.
    PromotionMoveGenerationTestCase {
        position: "position startpos moves h2h4 h8h7 h4h5 g8g7 h5h6 a7a6 h6h7 b7b6",
        moves: [
            Move::new_promotion(Square { row: 1, col: 7 }, Square { row: 0, col: 7 }, false, PieceType::Queen),
            Move::new_promotion(Square { row: 1, col: 7 }, Square { row: 0, col: 7 }, false, PieceType::Rook),
            Move::new_promotion(Square { row: 1, col: 7 }, Square { row: 0, col: 7 }, false, PieceType::Bishop),
            Move::new_promotion(Square { row: 1, col: 7 }, Square { row: 0, col: 7 }, false, PieceType::Knight),
        ],
    },

    // Black non-capture promotion: e2 -> e1
    // e1, d1, f1 wurden freigeräumt. Danach ist Schwarz am Zug.
    PromotionMoveGenerationTestCase {
        position: "position startpos moves e2e4 e7e5 e1e2 e5e4 d1d3 e4e3 f1f3 e3e2 a2a3",
        moves: [
            Move::new_promotion(Square { row: 6, col: 4 }, Square { row: 7, col: 4 }, false, PieceType::Queen),
            Move::new_promotion(Square { row: 6, col: 4 }, Square { row: 7, col: 4 }, false, PieceType::Rook),
            Move::new_promotion(Square { row: 6, col: 4 }, Square { row: 7, col: 4 }, false, PieceType::Bishop),
            Move::new_promotion(Square { row: 6, col: 4 }, Square { row: 7, col: 4 }, false, PieceType::Knight),
        ],
    },

    // Black capture promotion: a2xb1
    // a1 ist blockiert, b1 ist von weißem Springer besetzt, Schwarz ist am Zug.
    PromotionMoveGenerationTestCase {
        position: "position startpos moves h2h3 c7c5 b2b4 c5b4 a2a3 b4a3 g2g3 a3a2 e2e4",
        moves: [
            Move::new_promotion(Square { row: 6, col: 0 }, Square { row: 7, col: 1 }, true, PieceType::Queen),
            Move::new_promotion(Square { row: 6, col: 0 }, Square { row: 7, col: 1 }, true, PieceType::Rook),
            Move::new_promotion(Square { row: 6, col: 0 }, Square { row: 7, col: 1 }, true, PieceType::Bishop),
            Move::new_promotion(Square { row: 6, col: 0 }, Square { row: 7, col: 1 }, true, PieceType::Knight),
        ],
    },
];

pub struct PromotionMoveGenerationTestCase12 {
    pub position: &'static str,
    pub moves : [Move;12]
}

pub const MAX_PROMOTION_MOVE_GENERATION_TEST_CASE_12: PromotionMoveGenerationTestCase12 =
    PromotionMoveGenerationTestCase12 {
        position: "position startpos moves e2e4 e7e5 e4e5 e8e7 e5e6 a7a6 e6e7 a6a5",
        moves: [
            // forward promotion: e7 -> e8
            Move::new_promotion(
                Square { row: 1, col: 4 },
                Square { row: 0, col: 4 },
                false,
                PieceType::Queen,
            ),
            Move::new_promotion(
                Square { row: 1, col: 4 },
                Square { row: 0, col: 4 },
                false,
                PieceType::Rook,
            ),
            Move::new_promotion(
                Square { row: 1, col: 4 },
                Square { row: 0, col: 4 },
                false,
                PieceType::Bishop,
            ),
            Move::new_promotion(
                Square { row: 1, col: 4 },
                Square { row: 0, col: 4 },
                false,
                PieceType::Knight,
            ),

            // left capture promotion: e7 -> d8
            Move::new_promotion(
                Square { row: 1, col: 4 },
                Square { row: 0, col: 3 },
                true,
                PieceType::Queen,
            ),
            Move::new_promotion(
                Square { row: 1, col: 4 },
                Square { row: 0, col: 3 },
                true,
                PieceType::Rook,
            ),
            Move::new_promotion(
                Square { row: 1, col: 4 },
                Square { row: 0, col: 3 },
                true,
                PieceType::Bishop,
            ),
            Move::new_promotion(
                Square { row: 1, col: 4 },
                Square { row: 0, col: 3 },
                true,
                PieceType::Knight,
            ),

            // right capture promotion: e7 -> f8
            Move::new_promotion(
                Square { row: 1, col: 4 },
                Square { row: 0, col: 5 },
                true,
                PieceType::Queen,
            ),
            Move::new_promotion(
                Square { row: 1, col: 4 },
                Square { row: 0, col: 5 },
                true,
                PieceType::Rook,
            ),
            Move::new_promotion(
                Square { row: 1, col: 4 },
                Square { row: 0, col: 5 },
                true,
                PieceType::Bishop,
            ),
            Move::new_promotion(
                Square { row: 1, col: 4 },
                Square { row: 0, col: 5 },
                true,
                PieceType::Knight,
            ),
        ],
    };

    pub struct EnPassantMoveGenerationTestCase {
        pub position: &'static str,
        pub moves: [Move; 1],
    }

    pub const EN_PASSANT_MOVE_GENERATION_TEST_CASES: [EnPassantMoveGenerationTestCase; 4] = [
    // White: e5xd6 en passant
    EnPassantMoveGenerationTestCase {
        position: "position startpos moves e2e4 a7a6 e4e5 d7d5",
        moves: [
            Move::new_en_passant(
                Square { row: 3, col: 4 }, // e5
                Square { row: 2, col: 3 }, // d6
            ),
        ],
    },

    // White: d5xe6 en passant
    EnPassantMoveGenerationTestCase {
        position: "position startpos moves d2d4 a7a6 d4d5 e7e5",
        moves: [
            Move::new_en_passant(
                Square { row: 3, col: 3 }, // d5
                Square { row: 2, col: 4 }, // e6
            ),
        ],
    },

    // Black: e4xd3 en passant
    EnPassantMoveGenerationTestCase {
        position: "position startpos moves a2a3 e7e5 a3a4 e5e4 d2d4",
        moves: [
            Move::new_en_passant(
                Square { row: 4, col: 4 }, // e4
                Square { row: 5, col: 3 }, // d3
            ),
        ],
    },

    // Black: d4xe3 en passant
    EnPassantMoveGenerationTestCase {
        position: "position startpos moves a2a3 d7d5 a3a4 d5d4 e2e4",
        moves: [
            Move::new_en_passant(
                Square { row: 4, col: 3 }, // d4
                Square { row: 5, col: 4 }, // e3
            ),
        ],
    },
];


pub struct EnPassantApplyTestCase {
    pub position: &'static str,
    pub capturing_pawn_square: Square,
    pub captured_pawn_square: Square,
    pub expected_piece: Piece,
}   

pub const EN_PASSANT_APPLY_TEST_CASES: [EnPassantApplyTestCase; 4] = [
    // White: e5xd6 en passant
    EnPassantApplyTestCase {
        position: "position startpos moves e2e4 a7a6 e4e5 d7d5 e5d6",
        capturing_pawn_square: Square { row: 2, col: 3 }, // d6
        captured_pawn_square: Square { row: 3, col: 3 },  // d5
        expected_piece: Piece::new(PieceType::Pawn, Color::White)
    },

    // White: d5xe6 en passant
    EnPassantApplyTestCase {
        position: "position startpos moves d2d4 a7a6 d4d5 e7e5 d5e6",
        capturing_pawn_square: Square { row: 2, col: 4 }, // e6
        captured_pawn_square: Square { row: 3, col: 4 },  // e5
        expected_piece: Piece::new(PieceType::Pawn, Color::White)
    },

    // Black: e4xd3 en passant
    EnPassantApplyTestCase {
        position: "position startpos moves a2a3 e7e5 a3a4 e5e4 d2d4 e4d3",
        capturing_pawn_square: Square { row: 5, col: 3 }, // d3
        captured_pawn_square: Square { row: 4, col: 3 },  // d4
        expected_piece: Piece::new(PieceType::Pawn, Color::Black)
    },

    // Black: d4xe3 en passant
    EnPassantApplyTestCase {
        position: "position startpos moves a2a3 d7d5 a3a4 d5d4 e2e4 d4e3",
        capturing_pawn_square: Square { row: 5, col: 4 }, // e3
        captured_pawn_square: Square { row: 4, col: 4 },  // e4
        expected_piece: Piece::new(PieceType::Pawn, Color::Black)
    },
];