fn main(){
    let mut board: [[Option<Piece>; 8]; 8] = [
        [
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
        ]
    ];

    let mut all_legal_moves : Vec<Move> = Vec::new();


    for row in 0..8{
        for col in 0..8{
            if let Some(piece) = board[row][col]{

                let moves = match piece.kind{
                    PieceType::Pawn => generate_legal_moves_pawn(&board, row, col),
                    _ => Vec::new(),
                };

                all_legal_moves.extend(moves);

            }
        }
    }

    println!("{:#?}", all_legal_moves);

}


#[derive(Clone, Copy)]
struct Piece{
    kind : PieceType,
    is_white : bool,
}

#[derive(Clone, Copy)]
enum PieceType{
    King,
    Queen,
    Rook,
    Knight,
    Bishop,
    Pawn
}

fn p (kind : PieceType, is_white : bool) -> Option<Piece> {
    Some(Piece{kind, is_white})
}

fn generate_legal_moves_pawn(board : &[[Option<Piece>; 8]; 8] , row : usize, col : usize) -> Vec<Move>{
    let mut legal_moves: Vec<Move> = Vec::new();
    
    
    if let Some(piece) = board[row][col] {
       
        let direction : isize = if piece.is_white {-1} else {1}; 
        let start_row = if piece.is_white{6} else{1};

      
            //let capture_left = (row + 1, col-1);
            //let capture_right = (row + 1, col+1);

            
        let move_1sqr_forward = (row as isize + direction, col as isize);

        if check_if_square_is_in_bounds(move_1sqr_forward) {
            let move_1sqr_forward = (move_1sqr_forward.0 as usize, move_1sqr_forward.1 as usize);

            if !check_if_square_is_occupied(board, move_1sqr_forward){
                let current_move : Move = Move { from_row: row, from_col: col, to_row: move_1sqr_forward.0, to_col: move_1sqr_forward.1 };
                legal_moves.push(current_move);
            }    

            let move_2sqrs_forward = (row as isize + 2 * direction, col as isize);

            if check_if_square_is_in_bounds(move_2sqrs_forward) && row == start_row{
                
                let move_2sqrs_forward = (move_2sqrs_forward.0 as usize, move_2sqrs_forward.1 as usize);
                
                if !check_if_square_is_occupied(board, move_2sqrs_forward){

                    let current_move : Move = Move { from_row: row, from_col: col, to_row: move_2sqrs_forward.0, to_col: move_2sqrs_forward.1 };
                    legal_moves.push(current_move);
                } 
            }
        }
         

      
    } 

    legal_moves
   
}


fn check_if_square_is_in_bounds (square : (isize, isize)) -> bool{
    square.0 >= 0 &&
    square.0 < 8 &&
    square.1 >= 0 &&
    square.1 < 8
}
  
fn check_if_square_is_occupied(board : &[[Option<Piece>; 8]; 8] ,square :  (usize, usize)) -> bool{
    let (row, col) = square;

    if let Some(_) = board[row][col]{
        true
    }else{
        false
    }
}

#[derive(Clone, Copy, Debug)]

struct Move{
    from_row : usize,
    from_col : usize,
    to_row : usize,
    to_col : usize
}

struct Move_In_Chess_Notation{
    from_row : usize,
    from_col : usize,
    to_row : usize,
    to_col : usize
}


impl Move{
    fn translate_move_to_chess_notation (self) {

    }
}