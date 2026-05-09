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

    generate_legal_moves_pawn(&board, 4,7);
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

fn generate_legal_moves_pawn(board : &[[Option<Piece>; 8]; 8] , col : usize, row : usize) ->Vec<(usize, usize)>{
    let piece = board[row][col];
    let mut legal_moves: Vec<(usize, usize)> = Vec::new();
    
    
    if let Some(piece) = board[row][col] {
       
        let direction = if piece.is_white {-1} else {1}; 

        if piece.is_white{
            let capture_left = (row + 1, col-1);
            let capture_right = (row + 1, col+1);
            let move_1sqr_forward = (row+1, col);
            let move_2sqrs_forward = (row+2, col);

            if !check_if_square_is_occupied(board, move_1sqr_forward){
                legal_moves.push(move_1sqr_forward);
            }

            if !check_if_square_is_occupied(board, move_1sqr_forward) &&
               !check_if_square_is_occupied(board, move_2sqrs_forward){
                legal_moves.push(move_2sqrs_forward);
            }


        } else{
            let capture_left = (row-1, col-1);
            let capture_right = (row-1, col+1);
            let move_1sqr_forward = (row-1, col);
            let move_2sqrs_forward = (row-2, col);

            if !check_if_square_is_occupied(board, move_1sqr_forward){
                legal_moves.push(move_1sqr_forward);
            }

            if !check_if_square_is_occupied(board, move_1sqr_forward){
                legal_moves.push(move_2sqrs_forward);
            }
        }
    

      
    } else {
    
        
        
    
    }

    legal_moves
   
}

fn check_if_square_is_occupied(board : &[[Option<Piece>; 8]; 8] ,square :  (usize, usize)) -> bool{
    let (row, col) = square;

    if let Some(piece) = board[row][col]{
        true
    }else{
        false
    }
}