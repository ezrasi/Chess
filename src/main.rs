#![allow(unused)]

mod bitboard;
mod eval;
mod movegen;
mod perft;
mod search;
mod utils;
use std::io::{stdin, stdout, Write};

fn main() {
   
    println!("Ike: A Chess Engine by Ezra S-I");

    loop {

    let mut command_string = String::new();
    stdin().read_line(&mut command_string).expect("failed to readline");
    let command = command_string.trim();

    if command == "quit" {
        break;
    }
    if command == "uci" {
        println!("id name Ike");
        println!("id author Ezra S-I");

        println!("uciok");
    }



    }



}
