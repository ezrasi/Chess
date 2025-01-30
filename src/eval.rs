use crate::movegen::*;
use crate::utils::*;

// Bad evaluation function
pub fn eval(board: &Board) -> f32 {
    if legal_moves(board).len() == 0 {
        if in_check(board, board.turn) {
            if board.turn {
                return f32::NEG_INFINITY;
            } else {
                return f32::INFINITY;
            }
        }
        return 0.0;
    }
    let mut evaluation = 0.0;

    evaluation += count_ones(board.white_pawn) as f32;
    evaluation += 3.0 * count_ones(board.white_knight) as f32;
    evaluation += 3.2 * count_ones(board.white_bishop) as f32;
    evaluation += 5.0 * count_ones(board.white_rook) as f32;
    evaluation += 9.0 * count_ones(board.white_queen) as f32;
    evaluation -= count_ones(board.black_pawn) as f32;
    evaluation -= 3.0 * count_ones(board.black_knight) as f32;
    evaluation -= 3.2 * count_ones(board.black_bishop) as f32;
    evaluation -= 5.0 * count_ones(board.black_rook) as f32;
    evaluation -= 9.0 * count_ones(board.black_queen) as f32;

    evaluation
}
/*
* white_pawn = 0
* white_knight = 1
* white_bishop = 2
* white_rook = 3
* white_queen = 4
* white_king = 5
* black_pawn = 6
* black_knight = 7
* black_bishop = 8
* black_rook = 9
* black_queen = 10
* black_king = 11
*/
