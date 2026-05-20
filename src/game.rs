use crate::piece::{Piece, PieceType, Color};
use crate::board::{Board};
use crate::chess_move::{Move};
use crate::move_gen::pawn::generate_pawn_moves;
use crate::chess_square::Square;
use crate::debug::{log_debug};


pub struct Game{
    pub board : Board,
    pub color_to_move : Color,
    pub en_passant : Option<Square>
}


impl Game{

    pub fn new() -> Self{
        Self { board: Board::new(), color_to_move: Color::White, en_passant: None}
    }


    pub fn generate_all_legal_moves(self : &Game) -> Vec<Move>{
        let mut all_legal_moves : Vec<Move> = Vec::new();
      
        for (square, piece) in self.occupied_squares_by_color(self.color_to_move){

            let moves = match piece.piece_type{
                PieceType::Pawn => generate_pawn_moves(&self, square),
               // PieceType::Knight => generate_knight_moves(&self, Square { row, col }),
                _ => Vec::new(),
            };

            all_legal_moves.extend(moves);       
        }
           
        all_legal_moves
    }

    pub fn apply_position_from_startpos_uci(&mut self, startpos_uci : &str){
        self.board = Board::new();
        self.color_to_move = Color::White;


        let Some(_) = startpos_uci.split(" moves ").nth(1) else {
            return;
        };

        let startpos_uci = startpos_uci.replace("position startpos moves ", "");

        log_debug(&startpos_uci);

        

        for uci_move in startpos_uci.split_whitespace() {    
            let mut chess_move = Move::tranlate_uci_to_move(uci_move);

            if self.is_move_en_passant(chess_move){
                chess_move.is_en_passant = true;
            }
            self.apply_move(&chess_move);
        }


        

    }

    pub fn apply_move(&mut self, chess_move : &Move){

        self.move_piece(chess_move.from_square, chess_move.to_square);
     
        if chess_move.is_en_passant{
            if let Some(en_passant_square) = self.en_passant{
                self.remove_piece(en_passant_square);
            } else{
                panic!("Missing en_passant sqaure");
            }
        }

        if let Some(promotion) = chess_move.promotion{
            self.set_piece(chess_move.to_square, Piece::new(promotion, self.color_to_move));
        }
       
        self.check_for_en_pasant(chess_move);
 

        self.color_to_move = self.color_to_move.opposite(); 
    }

    pub fn check_for_en_pasant(&mut self, chess_move : &Move){
        let en_passant_from_row = if self.color_to_move == Color::White {6} else {1};
        let en_passant_to_row = if self.color_to_move == Color::White {4} else {3};

        if  chess_move.from_square.row == en_passant_from_row
            && chess_move.to_square.row == en_passant_to_row{
            let moved_piece = self.get_piece(chess_move.to_square);

            if let Some(moved_piece) = moved_piece{
                if moved_piece.piece_type == PieceType::Pawn{
                    
                    self.en_passant = Some(chess_move.to_square);

                    return;
                }
            }
        }
        self.en_passant = None;
    }

    pub fn is_move_en_passant(&self, chess_move : Move) -> bool{
        if let Some(piece) = self.get_piece(chess_move.from_square){
            let is_pawn = piece.piece_type == PieceType::Pawn;
            let is_capture = 0 != chess_move.from_square.col as isize - chess_move.to_square.col as isize;
            let is_empty_to_square = !self.is_occupied(chess_move.to_square);

            if is_pawn && is_capture && is_empty_to_square{
                return true;
            }

        }
        false
    }


    pub fn is_occupied(&self, square : Square) -> bool{
        self.board.is_occupied(square)
    }

    pub fn is_occupied_by_opponent_color(&self, square : Square, color : Color) -> bool{
        self.board.is_occupied_by_opponent_color(square, color)
    }

    pub fn get_piece(&self, square : Square) -> Option<Piece>{
        self.board.get_piece(square)
    }

    fn set_piece(&mut self, square : Square, piece : Piece){
        self.board.set_piece(square, piece);
    }

    fn remove_piece(&mut self, square : Square){
       self.board.remove_piece(square);
    }

    fn move_piece(&mut self, from_square : Square, to_square : Square){
        self.board.move_piece(from_square, to_square);
    }

    fn occupied_squares_by_color(&self, color : Color) -> Vec<(Square, Piece)>{
        self.board.occupied_squares_by_color(color)
    }
    

}


