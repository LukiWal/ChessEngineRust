use crate::{chess_square::Square, piece::PieceType};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Move{
    pub from_square : Square,
    pub to_square : Square,
    pub is_capture : bool,
    pub is_en_passant : bool,
    pub promotion : Option<PieceType>,
}

impl Move{
    pub const fn new_normal(from_square : Square, to_square : Square) -> Self{
        Self { from_square, to_square, is_capture: false, is_en_passant: false, promotion: None}
    }

    pub const fn new_capture(from_square : Square, to_square : Square) -> Self{
        Self { from_square, to_square, is_capture: true, is_en_passant: false, promotion: None}
    }

    pub const fn new_en_passant(from_square : Square, to_square : Square) -> Self{
        Self { from_square, to_square, is_capture: true, is_en_passant: true, promotion: None}
    }

    pub const fn new_promotion(from_square : Square, to_square : Square, is_capture : bool, promotion : PieceType) -> Self{
        Self { from_square, to_square, is_capture: is_capture, is_en_passant: false, promotion: Some(promotion)}
    }

    pub fn translate_move_to_uci(&self) -> String{
        let from_rank: char = Self::col_to_rank(self.from_square.col);
        let from_file = Self::row_to_file(self.from_square.row);

        let to_rank = Self::col_to_rank(self.to_square.col);
        let to_file = Self::row_to_file(self.to_square.row);

        let mut uci = format!("{}{}{}{}", from_rank, from_file, to_rank, to_file);
       
        if let Some(promotion) = self.promotion{
            uci.push(Self::promotion_to_char(promotion));
        }

        uci
    }

    pub fn tranlate_uci_to_move(uci : &str) -> Self{
        let uci = uci.as_bytes();

        let from_file = uci[0] as char; 
        let from_rank = uci[1] as char; 
        let to_file = uci[2] as char;  
        let to_rank = uci[3] as char;

        let from_row =  Self::rank_to_row(from_rank);
        let from_col = Self::file_to_col(from_file);
        let to_row =  Self::rank_to_row(to_rank);
        let to_col = Self::file_to_col(to_file); 

        let promotion: Option<PieceType> = if uci.len() > 4{
            let promotion = uci[4] as char;
            Some(Self::char_to_promotion(promotion))
        } else{
            None
        };

        Self{
            from_square : Square { row: from_row, col: from_col },
            to_square : Square { row: to_row, col: to_col },
            is_capture : false,
            promotion : promotion,
            is_en_passant : false, //FIX
        }
    }

    fn col_to_rank(col : usize) -> char{
        match col{
            0 => 'a',
            1 => 'b',
            2 => 'c',
            3 => 'd',
            4 => 'e',
            5 => 'f',
            6 => 'g',
            7 => 'h',
            _ => panic!("Invalid col"),
        }
    }

    fn row_to_file(row : usize) -> char{
        match row{
            0 => '8',
            1 => '7',
            2 => '6',
            3 => '5',
            4 => '4',
            5 => '3',
            6 => '2',
            7 => '1',
            _ => panic!("Invalid row"),
        } 
    }

    fn file_to_col(file: char) -> usize {
        match file {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => panic!("Invalid file"),
        }
    }
    
    fn rank_to_row(rank: char) -> usize {
        match rank {
            '8' => 0,
            '7' => 1,
            '6' => 2,
            '5' => 3,
            '4' => 4,
            '3' => 5,
            '2' => 6,
            '1' => 7,
            _ => panic!("Invalid rank"),
        }
    }

    fn promotion_to_char(promotion : PieceType) -> char {
        match promotion {
            PieceType::Queen => 'q',
            PieceType::Rook => 'r',
            PieceType::Knight => 'n',
            PieceType::Bishop => 'b',
            _ => panic!("Invalid promotion_to_char translation"),
        }
    }

    fn char_to_promotion(promotion_char : char) -> PieceType {
        match promotion_char {
            'q' => PieceType::Queen,
            'r' => PieceType::Rook,
            'n' =>PieceType::Knight,
            'b' => PieceType::Bishop,
            _ => panic!("Invalid char_to_promotion tranlation"),
        }
    }
}
