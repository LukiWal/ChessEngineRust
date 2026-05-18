use crate::piece::{Piece, PieceType};
use crate::chess_move::{Move};
use crate::move_gen::pawn::generate_pawn_moves;
use crate::chess_square::Square;
use crate::debug::{log_debug, log_value};


use std::io::{self, BufRead, Write};
use rand::seq::IndexedRandom;
use std::fs::OpenOptions;

pub struct Game{
    pub board : [[Option<Piece>; 8]; 8],
    pub en_passant : Option<Square>,
    pub white_to_move : bool
}


impl Game{
    pub fn initialize_board() -> [[Option<Piece>; 8]; 8]{
        [[
            p(PieceType::Rook, false), 
            p(PieceType::Knight, false),
            p(PieceType::Bishop, false),
            p(PieceType::Queen, false),
            p(PieceType::King, false),
            p(PieceType::Bishop, false),
            p(PieceType::Knight, false),
            p(PieceType::Rook, false),
        ],
        [
            p(PieceType::Pawn, false), 
            p(PieceType::Pawn, false),
            p(PieceType::Pawn, false),
            p(PieceType::Pawn, false),
            p(PieceType::Pawn, false),
            p(PieceType::Pawn, false),
            p(PieceType::Pawn, false),
            p(PieceType::Pawn, false),
        ],
            [None;8],
            [None;8],
            [None;8],
            [None;8],
        [
            p(PieceType::Pawn, true), 
            p(PieceType::Pawn, true),
            p(PieceType::Pawn, true),
            p(PieceType::Pawn, true),
            p(PieceType::Pawn, true),
            p(PieceType::Pawn, true),
            p(PieceType::Pawn, true),
            p(PieceType::Pawn, true),
        ],
        [
            p(PieceType::Rook, true), 
            p(PieceType::Knight, true),
            p(PieceType::Bishop, true),
            p(PieceType::King, true),
            p(PieceType::Queen, true),
            p(PieceType::Bishop, true),
            p(PieceType::Knight, true),
            p(PieceType::Rook, true),
        ]]
    }

    pub fn generate_all_legal_moves(self : &Game) -> Vec<Move>{
        let mut all_legal_moves : Vec<Move> = Vec::new();
        for row in 0..8{
            for col in 0..8{
                if let Some(piece) = &self.board[row][col]{

                    if piece.is_white == false{
                        let moves = match piece.kind{
                            PieceType::Pawn => generate_pawn_moves(&self, row, col),
                            _ => Vec::new(),
                        };
        
                        all_legal_moves.extend(moves);                       
                    }
                }
            }
        }

        all_legal_moves
    }

    pub fn apply_position_from_startpos_uci(&mut self, startpos_uci : &str){
        self.board = Game::initialize_board();

        let startpos_uci = startpos_uci.replace("position startpos moves ", "");

        log_debug(&startpos_uci);

        
        self.white_to_move = true; //Correct init needed, maybe?

        let mut move_counter = 0;
        for move_uci in startpos_uci.split_whitespace() {    
            let chess_move = Move::tranlate_uci_to_move(move_uci);

            self.apply_move(&chess_move);
        }


        

    }

    pub fn apply_move(&mut self, chess_move : &Move){
        self.board[chess_move.to_square.row][chess_move.to_square.col] = self.board[chess_move.from_square.row][chess_move.from_square.col];
        self.board[chess_move.from_square.row][chess_move.from_square.col] = None;

       

       
        self.check_for_en_pasant(chess_move);
 

        self.white_to_move = !self.white_to_move; 
    }

    pub fn check_for_en_pasant(&mut self, last_move : &Move){
        let en_passant_from_row = if self.white_to_move {6} else {1};
        let en_passant_to_row = if self.white_to_move {4} else {3};

        if  last_move.from_square.row == en_passant_from_row
            && last_move.to_square.row == en_passant_to_row{
            let moved_piece = self.board[last_move.to_square.row][last_move.to_square.col];

            

            if let Some(moved_piece) = moved_piece{
                if moved_piece.kind == PieceType::Pawn{
                    
                    self.en_passant = Some(last_move.to_square);

                    return;
                }
            }
        }
        self.en_passant = None;
    }

}

fn p (kind : PieceType, is_white : bool) -> Option<Piece> {
    Some(Piece{kind, is_white})
}