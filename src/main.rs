#![allow(unused)]

mod bitboard;
mod eval;
mod movegen;
mod perft;
mod play;
mod search;
mod utils;
mod hash;
use crate::play::*;
use crate::search::*;
use crate::utils::*;
use regex::Regex;
use std::io::{stdin, stdout, Write};
use std::thread;

fn main() {
    println!("Ike: A Chess Engine by Ezra S-I");

    // initialize bitboards
    let mut handle = None;
    handle = Some(thread::spawn(|| {
        let _ = &*bitboard::MAGIC_TABLES;
    }));

    let mut board = starting_position();

    loop {
        let mut command_string = String::new();
        stdin()
            .read_line(&mut command_string)
            .expect("failed to readline");
        let command = command_string.trim();

        if command == "quit" {
            break;
        }
        if command == "uci" {
            println!("id name Ike");
            println!("id author Ezra S-I");

            println!("uciok");
        }
        if command == "isready" {
            if let Some(thread_handle) = handle.take() {
                let _ = thread_handle.join();
            }
            println!("readyok");
        }

        //finish implementing the actual position command.
        let re = Regex::new(r"position (startpos|.+) moves (.*)").unwrap();
        if let Some(captures) = re.captures(command) {
            let move_list = &captures[2];
            for mv in move_list.split_whitespace() {
                make_user_move(mv, &mut board);
            }
            print_binary_board(board.white | board.black);
        }

        if command == "go" {
            let (best, eval) = best_move(&board, 1);
            println!("{:?}", best);
            println!("evaluation: {eval}");
        }

        if command == "play" {
            play_game(&board);
        }
    }
}
