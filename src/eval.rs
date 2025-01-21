use crate::utils::*;

// Bad evaluation function
pub fn eval(board: &Board) -> f32 {
    let white_num = set_bit_positions(board.white).len() as f32;
    let black_num = set_bit_positions(board.black).len() as f32;

    white_num - black_num
}
