mod bitboard;
mod board;
mod movegen;
mod utils;

use crate::utils::fen_to_board;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        println!("Usage: {} perft <depth> <fen> [moves]", args[0]);
        return;
    }

    if args[1] != "perft" {
        println!("Only 'perft' command is supported");
        return;
    }

    let depth: u8 = match args[2].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid depth value");
            return;
        }
    };

    let fen = &args[3];
    let moves = if args.len() > 4 { Some(&args[4]) } else { None };

    // Convert FEN to board
    let mut board = fen_to_board(fen);

    // Apply moves if provided
    if let Some(move_list) = moves {
        for mv in move_list.split_whitespace() {
            let from = square_to_index(&mv[0..2]);
            let to = square_to_index(&mv[2..4]);

            // Find the move in legal moves that matches from and to squares
            let legal_moves = movegen::legal_moves(&board);
            let mv = match legal_moves.iter().find(|m| m.from == from && m.to == to) {
                Some(m) => {
                    m
                }
                None => {
                    println!("Invalid move: {}", mv);
                    return;
                }
            };

            board = movegen::make_move(&board, mv);
        }
    }

    // Run perft
    movegen::perft(&board, depth, true);
}

fn square_to_index(square: &str) -> u8 {
    let chars: Vec<char> = square.chars().collect();
    let file = chars[0] as u8 - b'a';
    let rank = chars[1] as u8 - b'1';
    (rank * 8) + file
}
