use crate::utils::*;
use lazy_static::lazy_static;

/* Multiply position index by following coefficients
*
* white_pawn 
*   
*/
lazy_static! {
    pub static ref ZOBRIST_KEYS: Vec<u64> = generate_zobrist();
}

fn generate_zobrist() -> Vec<u64> {
    let mut result: Vec<u64> = Vec::with_capacity(781);
    let length = result.len();
    for i in 0..length {
        result[i] = rand::random();
    }
    result
}


pub fn zobrist_hash(board: &Board) -> u64 {




}
