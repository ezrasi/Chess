use crate::utils::*;

// Bad evaluation function
pub fn eval(board: &Board) -> f32 {
    let white_num = set_bit_positions(board.white).len();
    let black_num = set_bit_positions(board.black).len();

    (white_num - black_num) as f32
}
