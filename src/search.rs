use crate::eval::*;
use crate::movegen::legal_moves;
use crate::movegen::make_move;
use crate::utils::*;

// still need to account for checkmate and stalemate
pub fn best_move(board: &Board, depth: u8) -> (Option<Move>, f32) {
    let mut possibilities = legal_moves(board);

    // check for stalemate or checkmate
    if possibilities.len() == 0 {
        return (None, eval(board));
    }

    
    // do higher impact moves first
    possibilities.sort_by_key(|m| std::cmp::Reverse(m.kind));
    // go further into enemy position
    if board.turn {
        possibilities.sort_by_key(|m| std::cmp::Reverse(m.to));
    } else {
        possibilities.sort_by_key(|m| m.to);
    }


    // if in first 10 moves, get pieces off back rank
    if board.fullmove <= 10 {
        if board.turn {
            possibilities.sort_by_key(|m| m.from);
        } else {
            possibilities.sort_by_key(|m| std::cmp::Reverse(m.from));
        }
    }

    // make moves closer to center
    possibilities.sort_by_key(|m| {
        let file = m.to % 8;
        let dist_to_d = (file as i8 - 3).abs();
        let dist_to_e = (file as i8 - 4).abs();
        dist_to_d.min(dist_to_e)
    });

    // move less valuable pieces first
    possibilities.sort_by_key(|m| m.piece);

    let mut evaluations = Vec::new();

    for ply in possibilities.iter() {
        let made_move = make_move(board, &ply);
        evaluations.push(best_move_helper(&made_move, depth - 1));
    }

    let mut best = if board.turn {
        f32::NEG_INFINITY
    } else {
        f32::INFINITY
    };

    for evaluation in evaluations.iter() {
        if board.turn {
            if *evaluation > best {
                best = *evaluation;
            }
        } else {
            if *evaluation < best {
                best = *evaluation;
            }
        }
    }

    let mut i = 0;

    for evaluation in evaluations {
        if evaluation == best {
            break;
        }

        i += 1;
    }

    (Some(possibilities[i].clone()), best)
}

fn best_move_helper(board: &Board, depth: u8) -> f32 {
    if depth == 0 {
        return eval(board);
    }
    let possibilities = legal_moves(board);

    // check for stalemate or checkmate
    if possibilities.len() == 0 {
        return eval(board);
    }

    let mut first = true;
    let mut best = if board.turn {
        f32::NEG_INFINITY
    } else {
        f32::INFINITY
    };

    for ply in possibilities.into_iter() {
        let made_move = make_move(board, &ply);
        let evaluation = best_move_helper(&made_move, depth - 1);
        if first {
            best = evaluation;
            first = false;
        } else if board.turn {
            if evaluation > best {
                best = evaluation;
            }
        } else {
            if evaluation < best {
                best = evaluation;
            }
        }
    }

    best
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;

    #[test]
    fn best_start() {
        let board = starting_position();
        let (best, eval) = best_move(&board, 5);
        println!("Best: {:?}", best);
        println!("Eval: {eval}");
    }
}
