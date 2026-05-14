mod piece;
mod move_gen;
mod chess_move;
mod game;
mod chess_square;


use std::io::{self, BufRead, Write};
use rand::seq::IndexedRandom;
use std::fs::OpenOptions;

use crate::game::Game;



fn main(){
    //let mut board: [[Option<Piece>; 8]; 8] = [
       

    let mut game = Game{
        board : Game::initialize_board(),
        is_white : false
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
            log_debug(&input);

            

        } else if input.starts_with("go") {

            // GUI fragt: "Mach einen Zug"


            let all_legal_moves= game.generate_all_legal_moves();

            let random_move = match all_legal_moves.choose(&mut rand::rng()) {
                Some(i) => i,
                None => panic!("rip")
            };

            game.apply_move(&random_move);

            let string = format!("bestmove {}", random_move.translate_move_to_uci());
            log_debug(&string);
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


fn log_debug(message: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("./../../uci_debug.log")
        .unwrap();

    writeln!(file, "{}", message).unwrap();
}