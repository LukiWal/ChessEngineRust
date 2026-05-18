mod piece;
mod move_gen;
mod chess_move;
mod game;
mod chess_square;
mod debug;


use std::io::{self, BufRead, Write};
use rand::seq::IndexedRandom;
use crate::chess_move::Move;
use crate::debug::{log_debug, log_value};
use crate::game::Game;



fn main(){
    //let mut board: [[Option<Piece>; 8]; 8] = [
       

    let mut game = Game{
        board : Game::initialize_board(),
        white_to_move : false,
        en_passant: None    
    };

    

    


    let stdin = io::stdin();

    for line in stdin.lock().lines() {

        let input = match line {

            Ok(line) => line.trim().to_string(),

            Err(_) => break,

        };

        if input == "uci" {

            send("id name MyFirstEngine");

            send("id author Lukas");

            send("uciok");

        } else if input == "isready" {

            send("readyok");

        } else if input == "ucinewgame" {

            // Neues Spiel starten / Board resetten

        } else if input.starts_with("position") {

            // Hier kommt z.B.:

            // position startpos

            // position startpos moves e2e4 e7e5
            
            Game::apply_position_from_startpos_uci(&mut game, &input);
           

            

        } else if input.starts_with("go") {

            // GUI fragt: "Mach einen Zug"


            let all_legal_moves= game.generate_all_legal_moves();

            let mut very_good_moves : Vec<Move> = Vec::new();
            let mut good_moves : Vec<Move> = Vec::new();
            let mut moves : Vec<Move> = Vec::new();

            for chess_move in all_legal_moves{
                if chess_move.is_en_passant == true || chess_move.promotion.is_some(){
                    very_good_moves.push(chess_move);
                } else if chess_move.is_capture == true{
                    good_moves.push(chess_move);
                } else{
                    moves.push(chess_move);
                }
            }

            let all_legal_moves = 
            if very_good_moves.len() > 0{
                very_good_moves
            } else if good_moves.len() > 0{
                good_moves
            } else{
                moves
            };

            let random_move = match all_legal_moves.choose(&mut rand::rng()) {
                Some(i) => i,
                None => panic!("rip")
            };

            log_debug("\n Apply Engine Move: ");
            game.apply_move(&random_move);

            let string = format!("bestmove {}", random_move.translate_move_to_uci());
            println!("{}", string);
            io::stdout().flush().unwrap();

        } else if input == "stop" {

            // später wichtig, wenn deine Engine länger rechnet

        } else if input == "quit" {

            break;

        }

    }

    //for x in &all_legal_moves{
    //    println!("{}", x.translate_move_to_uci());
    //}

    //println!("{:#?}", &all_legal_moves);

}


fn send(message: &str) {
    println!("{}", message);
    io::stdout().flush().unwrap();
}
