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
    let white_num = set_bit_positions(board.white).len() as f32;
    let black_num = set_bit_positions(board.black).len() as f32;

    2.0 * (white_num - black_num)
}
