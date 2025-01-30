use crate::eval::*;
use crate::movegen::*;
use crate::search::*;
use crate::utils::*;
use crate::Regex;
use std::io::{stdin, stdout, Write};

pub fn play_game(board_param: &Board) {
    let mut board = board_param.clone();
    println!("white or black?");
    let mut user_color = String::new();
    stdin()
        .read_line(&mut user_color)
        .expect("failed to readline");

    // white = 1, black = 0
    let mut engine_color = 0;

    if user_color.trim() == "black" {
        engine_color = 1;
    }

    println!("engine depth?");
    let mut depth_string = String::new();
    stdin()
        .read_line(&mut depth_string)
        .expect("failed to readline");
    let depth = depth_string.trim().parse::<u8>().unwrap();
    // returns true if game is over
    let ending = |evaluation: f32, is_user: bool| {
        if evaluation == 0.0 {
            println!("Stalemate");
        } else {
            println!("Checkmate.");
            if is_user {
                println!("You win!");
            } else {
                println!("I win!");
            }
        }
    };

    if engine_color == 0 {
        println!("Type moves with format e2e4, d7d8q, e1g1 (castling). Good luck. You may begin.");

        println!();
        println!("{}.", board.fullmove);

        // make user move
        let mut user_move = String::new();
        stdin()
            .read_line(&mut user_move)
            .expect("failed to readline");

        let re = Regex::new(r"^([A-Ha-h][1-8]){2}[nbrqNBRQ]?$").unwrap();

        while user_move.trim() != "quit" {
            // make sure move is valid syntax
            if re.is_match(user_move.trim()) {
                // make sure move is legal
                if make_user_move(&user_move, &mut board) {
                    // now make engine move
                    let (best, eval) = best_move(&board, depth);

                    if let Some(unwrapped) = best {
                        println!(
                            "{} to {}",
                            index_to_square(unwrapped.from),
                            index_to_square(unwrapped.to)
                        );
                        board = make_move(&board, &unwrapped);
                        let (user_option, user_eval) = best_move(&board, 1);
                        if user_option.is_none() {
                            ending(eval, false);
                            break;
                        }
                    } else {
                        ending(eval, true);
                        break;
                    }

                    println!();
                    println!("{}.", board.fullmove);
                }
            } else {
                println!("Invalid move: {}", user_move.trim());
                println!();
            }

            user_move.clear();
            stdin()
                .read_line(&mut user_move)
                .expect("failed to readline");
        }
    }
}

pub fn make_user_move(mv: &str, board: &mut Board) -> bool {
    let from = square_to_index(&mv[0..2]);
    let to = square_to_index(&mv[2..4]);
    let mut is_promotion = false;
    let mut quiet_promo = 0;
    let mut capture_promo = 0;

    if mv.trim().len() == 5 {
        let mvstring: Vec<char> = mv.chars().collect();
        is_promotion = true;
        if mvstring[4] == 'n' {
            quiet_promo = KNIGHT_PROMO;
            capture_promo = KNIGHT_PROMO_CAPTURE;
        }
        if mvstring[4] == 'b' {
            quiet_promo = BISHOP_PROMO;
            capture_promo = BISHOP_PROMO_CAPTURE;
        }
        if mvstring[4] == 'r' {
            quiet_promo = ROOK_PROMO;
            capture_promo = ROOK_PROMO_CAPTURE;
        } else if mvstring[4] == 'q' {
            quiet_promo = QUEEN_PROMO;
            capture_promo = QUEEN_PROMO_CAPTURE;
        }
    }
    // Find the move in legal moves that matches from and to squares
    let legal_moves = legal_moves(&board);
    let mv = match legal_moves.iter().find(|m| {
        m.from == from
            && m.to == to
            && (if is_promotion {
                m.kind == quiet_promo || m.kind == capture_promo
            } else {
                true
            })
    }) {
        Some(m) => m,
        None => {
            println!("Invalid move: {}", mv);
            println!();
            return false;
        }
    };
    *board = make_move(board, mv);
    true
}
