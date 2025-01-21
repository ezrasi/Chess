use crate::movegen::*;
use crate::utils::*;
use std::thread;
use std::time::Instant;

pub fn perft(board: &Board, depth: u8) -> u64 {
    let now = Instant::now();
    let moves = legal_moves(board);
    if depth == 1 {
        println!("{}", moves.len());
        return moves.len() as u64;
    }

    let mut handles = Vec::with_capacity(moves.len());

    for ply in moves.iter() {
        let new_board = make_move(board, &ply);
        let handle = thread::spawn(move || perft_helper(&new_board, depth - 1));
        handles.push(handle);
    }

    let mut count = 0;

    for handle in handles {
        let result = handle.join().unwrap();
        count += result;
    }

    println!("");
    println!("{}", count);

    println!("");
    println!("Finished in {} seconds", now.elapsed().as_secs());

    count
}
fn perft_helper(board: &Board, depth: u8) -> u64 {
    let mut count = 0;
    let moves = legal_moves(board);

    if depth == 1 {
        return moves.len() as u64;
    }

    for ply in moves.iter() {
        let new_board = make_move(board, &ply);
        count += perft_helper(&new_board, depth - 1);
    }
    count
}

/*
pub fn perft(board: &Board, depth: u8, first: bool) -> u64 {
    if depth == 0 {
        return 1;
    };
    if !first {
        let mut count = 0;
        let moves = legal_moves(board);
        for ply in moves.iter() {
            let new_board = make_move(board, &ply);
            count += perft(&new_board, depth - 1, false);
        }
        count
    } else {
        let mut count = 0;
        let moves = legal_moves(board);
        for ply in moves.iter() {
            let new_board = make_move(board, &ply);
            let i = perft(&new_board, depth - 1, false);
            count += i;
            let name = match ply.kind {
                KNIGHT_PROMO => format!(
                    "{}{}n",
                    SQUARES[ply.from as usize], SQUARES[ply.to as usize]
                ),
                KNIGHT_PROMO_CAPTURE => format!(
                    "{}{}n",
                    SQUARES[ply.from as usize], SQUARES[ply.to as usize]
                ),
                BISHOP_PROMO => format!(
                    "{}{}b",
                    SQUARES[ply.from as usize], SQUARES[ply.to as usize]
                ),
                BISHOP_PROMO_CAPTURE => format!(
                    "{}{}b",
                    SQUARES[ply.from as usize], SQUARES[ply.to as usize]
                ),
                ROOK_PROMO => format!(
                    "{}{}r",
                    SQUARES[ply.from as usize], SQUARES[ply.to as usize]
                ),
                ROOK_PROMO_CAPTURE => format!(
                    "{}{}r",
                    SQUARES[ply.from as usize], SQUARES[ply.to as usize]
                ),
                QUEEN_PROMO => format!(
                    "{}{}q",
                    SQUARES[ply.from as usize], SQUARES[ply.to as usize]
                ),
                QUEEN_PROMO_CAPTURE => format!(
                    "{}{}q",
                    SQUARES[ply.from as usize], SQUARES[ply.to as usize]
                ),
                _ => format!("{}{}", SQUARES[ply.from as usize], SQUARES[ply.to as usize]),
            };
            println!("{} {}", name, i);
        }
        println!("");
        println!("{}", count);
        count
    }
}
*/
fn perft_captures(board: &Board, depth: u8) -> u64 {
    if depth == 1 {
        let moves = legal_moves(board);
        let mut count = 0;
        for ply in moves.iter() {
            if ply.kind == CAPTURE {
                count += 1;
            }
        }
        return count;
    };
    let mut count = 0;

    let moves = legal_moves(board);
    for ply in moves.iter() {
        let new_board = make_move(board, &ply);
        count += perft_captures(&new_board, depth - 1);
    }
    count
}

fn perft_ep(board: &Board, depth: u8) -> u64 {
    if depth == 1 {
        let moves = legal_moves(board);
        let mut count = 0;
        for ply in moves.iter() {
            if ply.kind == EN_PASSANT {
                count += 1;
            }
        }
        return count;
    };
    let mut count = 0;

    let moves = legal_moves(board);
    for ply in moves.iter() {
        let new_board = make_move(board, &ply);
        count += perft_ep(&new_board, depth - 1);
    }
    count
}
