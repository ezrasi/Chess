use crate::eval::*;
use crate::movegen::legal_moves;
use crate::movegen::make_move;
use crate::utils::*;
use std::collections::HashMap;

fn best_move(board: &Board, depth: u8) -> Move {
    let possibilites = legal_moves(board);
}

fn best_move_helper(board: &Board, depth: u8) -> f32 {
    if depth == 0 {
        return eval(board);
    }
    let possibilites = legal_moves(board);
    let mut evaluations: HashMap<Move, f32> = HashMap::new();

    for ply in possibilites.into_iter() {
        let made_move = make_move(board, &ply);
        evaluations.insert(ply, best_move_helper(&made_move, depth - 1));
    }

    let best = if board.turn {
        evaluations
            .values()
            .fold(f32::NEG_INFINITY, |a, &b| a.max(b));
    } else {
        evaluations.values().fold(f32::INFINITY, |a, &b| a.min(b));
    };

    7.0
}
