mod board;
use board::*;

fn main() {

    let hi = Board {
        mailbox: [0; 64],
        white: 1,
        black: 1,
        white_pawn:1,
        white_knight: 1,
        white_bishop: 1,
        white_rook: 1,
        white_queen: 1,
        white_king: 1,
        black_pawn: 1,
        black_knight: 1,
        black_bishop: 1,
        black_rook: 1,
        black_queen: 1,
        black_king: 1,
    };
        dbg!(&hi);
}
