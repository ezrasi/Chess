use crate::utils::*;
use lazy_static::lazy_static;


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

/* 
* 64(x) + position index
* x changes for each piece type:
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
*   
* turn = 64(12) 
* white_kingside_castle = 64(12) + 1
* white_queenside_castle = 64(12) + 2
* black_kingside_castle = 64(12) + 3
* black_queenside_castle = 64(12) + 4
*
* ep_target file = 64(12) + 5 + file
*/
pub fn zobrist_hash(board: &Board) -> u64 {
    let mut hash: u64 = 0;
    let white_pawns = set_bit_positions(board.white_pawn);
    for white_pawn in white_pawns {
        hash ^= ZOBRIST_KEYS[white_pawn as usize];
    }
    let white_knights = set_bit_positions(board.white_knight);
    for white_knight in white_knights {
        hash ^= ZOBRIST_KEYS[(64 + white_knight) as usize];
    }
    let white_bishops = set_bit_positions(board.white_bishop);
    for white_bishop in white_bishops {
        hash ^= ZOBRIST_KEYS[(128 + white_bishop) as usize];
    }
    let white_rooks = set_bit_positions(board.white_rook);
    for white_rook in white_rooks {
        hash ^= ZOBRIST_KEYS[(192 + white_rook) as usize];
    }
    let white_queens = set_bit_positions(board.white_queen);
    for white_queen in white_queens {
        hash ^= ZOBRIST_KEYS[256 + white_queen as usize];
    }
    let white_king = set_bit_positions(board.white_king)[0];
    hash ^= ZOBRIST_KEYS[320 + white_king as usize];
    let black_pawns = set_bit_positions(board.black_pawn);
    for black_pawn in black_pawns {
        hash ^= ZOBRIST_KEYS[384 + black_pawn as usize];
    }
    let black_knights = set_bit_positions(board.black_knight);
    for black_knight in black_knights {
        hash ^= ZOBRIST_KEYS[448 + black_knight as usize];
    }
    let black_bishops = set_bit_positions(board.black_bishop);
    for black_bishop in black_bishops {
        hash ^= ZOBRIST_KEYS[512 + black_bishop as usize];
    }
    let black_rooks = set_bit_positions(board.black_rook);
    for black_rook in black_rooks {
        hash ^= ZOBRIST_KEYS[576 + black_rook as usize];
    }
    let black_queens = set_bit_positions(board.black_queen);
    for black_queen in black_queens {
        hash ^= ZOBRIST_KEYS[640 + black_queen as usize];
    }
    let black_king = set_bit_positions(board.black_king)[0];
    hash ^= ZOBRIST_KEYS[704 + black_king as usize];

    if !board.turn {
        hash ^= ZOBRIST_KEYS[768];
    }

    if board.white_kingside_castle {
        hash ^= ZOBRIST_KEYS[769];
    }
    if board.white_queenside_castle {
        hash ^= ZOBRIST_KEYS[770];
    }
    if board.black_kingside_castle {
        hash ^= ZOBRIST_KEYS[771];
    }
    if board.black_queenside_castle {
        hash ^= ZOBRIST_KEYS[772];
    }

    if board.ep_target.is_some() {
        let file = board.ep_target.unwrap() % 8;
        hash ^= ZOBRIST_KEYS[773 + file as usize];
    }

    hash
   }               
