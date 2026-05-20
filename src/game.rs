use crate::piece::{Piece, PieceType, Color};
use crate::chess_move::{Move};
use crate::move_gen::pawn::generate_pawn_moves;
use crate::chess_square::Square;
use crate::debug::{log_debug};


pub struct Game{
    pub board : [[Option<Piece>; 8]; 8],
    pub color_to_move : Color,
    pub en_passant : Option<Square>
}


impl Game{

    pub fn new() -> Self{
        Self { board: Self::initialize_board(), color_to_move: Color::White, en_passant: None}
    }
    
    pub fn initialize_board() -> [[Option<Piece>; 8]; 8]{
        [[
    new_piece(PieceType::Rook,   Color::Black), 
    new_piece(PieceType::Knight, Color::Black),
    new_piece(PieceType::Bishop, Color::Black),
    new_piece(PieceType::Queen,  Color::Black),
    new_piece(PieceType::King,   Color::Black),
    new_piece(PieceType::Bishop, Color::Black),
    new_piece(PieceType::Knight, Color::Black),
    new_piece(PieceType::Rook,   Color::Black),
],
[
    new_piece(PieceType::Pawn, Color::Black), 
    new_piece(PieceType::Pawn, Color::Black),
    new_piece(PieceType::Pawn, Color::Black),
    new_piece(PieceType::Pawn, Color::Black),
    new_piece(PieceType::Pawn, Color::Black),
    new_piece(PieceType::Pawn, Color::Black),
    new_piece(PieceType::Pawn, Color::Black),
    new_piece(PieceType::Pawn, Color::Black),
],
    [None;8],
    [None;8],
    [None;8],
    [None;8],
[
    new_piece(PieceType::Pawn, Color::White), 
    new_piece(PieceType::Pawn, Color::White),
    new_piece(PieceType::Pawn, Color::White),
    new_piece(PieceType::Pawn, Color::White),
    new_piece(PieceType::Pawn, Color::White),
    new_piece(PieceType::Pawn, Color::White),
    new_piece(PieceType::Pawn, Color::White),
    new_piece(PieceType::Pawn, Color::White),
],
[
    new_piece(PieceType::Rook,   Color::White), 
    new_piece(PieceType::Knight, Color::White),
    new_piece(PieceType::Bishop, Color::White),
    new_piece(PieceType::Queen,  Color::White),
    new_piece(PieceType::King,   Color::White),
    new_piece(PieceType::Bishop, Color::White),
    new_piece(PieceType::Knight, Color::White),
    new_piece(PieceType::Rook,   Color::White),
]]
    }

    pub fn generate_all_legal_moves(self : &Game) -> Vec<Move>{
        let mut all_legal_moves : Vec<Move> = Vec::new();
        for row in 0..8{
            for col in 0..8{
                if let Some(piece) = &self.board[row][col]{

                    if piece.color == self.color_to_move{
                        let moves = match piece.piece_type{
                            PieceType::Pawn => generate_pawn_moves(&self, row, col),
                           // PieceType::Knight => generate_knight_moves(&self, Square { row, col }),
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
        self.color_to_move = Color::White;


        let Some(_) = startpos_uci.split(" moves ").nth(1) else {
            return;
    
        };

        let startpos_uci = startpos_uci.replace("position startpos moves ", "");

        log_debug(&startpos_uci);

        

        for move_uci in startpos_uci.split_whitespace() {    
            let mut chess_move = Move::tranlate_uci_to_move(move_uci);

            log_debug("what");
            if self.is_move_en_passant(chess_move){
                chess_move.is_en_passant = true;
            }
            self.apply_move(&chess_move);
        }


        

    }

    pub fn apply_move(&mut self, chess_move : &Move){

        self.board[chess_move.to_square.row][chess_move.to_square.col] = self.board[chess_move.from_square.row][chess_move.from_square.col];
        self.board[chess_move.from_square.row][chess_move.from_square.col] = None;

        if chess_move.is_en_passant{
            if let Some(en_passant) = self.en_passant{
                self.board[en_passant.row][en_passant.col] = None;
            } else{
                panic!("Missing en_passant sqaure");
            }
        }

        if let Some(promotion) = chess_move.promotion{
            self.board[chess_move.to_square.row][chess_move.to_square.col] = Some(Piece{piece_type: promotion, color: self.color_to_move});
        }
       
        self.check_for_en_pasant(chess_move);
 

        //log_debug(&format!("Color Switch from {} to {}", self.white_to_move,  !self.white_to_move));
        self.color_to_move = self.color_to_move.opposite(); 
    }

    pub fn check_for_en_pasant(&mut self, last_move : &Move){
        let en_passant_from_row = if self.color_to_move == Color::White {6} else {1};
        let en_passant_to_row = if self.color_to_move == Color::White {4} else {3};

        if  last_move.from_square.row == en_passant_from_row
            && last_move.to_square.row == en_passant_to_row{
            let moved_piece = self.board[last_move.to_square.row][last_move.to_square.col];

            

            if let Some(moved_piece) = moved_piece{
                if moved_piece.piece_type == PieceType::Pawn{
                    
                    self.en_passant = Some(last_move.to_square);

                    return;
                }
            }
        }
        self.en_passant = None;
    }

    pub fn is_move_en_passant(&self, chess_move : Move) -> bool{
        if let Some(piece) = self.get_piece_at_square(chess_move.from_square){
            let is_pawn = piece.piece_type == PieceType::Pawn;
            let is_capture = 0 != chess_move.from_square.col as isize - chess_move.to_square.col as isize;
            let is_empty_to_square = !self.is_piece_at_square(chess_move.to_square);

            log_debug(&format!("is_pawn: {}, is_capture: {}, is_empty_to_square: {}", is_pawn, is_capture, is_empty_to_square));
            if is_pawn && is_capture && is_empty_to_square{
                return true;
            }

        }
        false
    }

    pub fn get_piece_at_square(&self, square : Square) -> Option<Piece>{
        self.board[square.row][square.col]
    }

    pub fn is_piece_at_square(&self, square : Square) -> bool{
        self.board[square.row][square.col].is_some()
    }

}

fn new_piece (piece_type : PieceType, color : Color) -> Option<Piece> {
    Some(Piece{piece_type, color})
}

