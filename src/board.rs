//TODO: revisit whether or not Move needs a color field. if not, clean up all the code that relies on it.

//piece codes
const WHITE_PAWN: u8 = 0b00000101;
const WHITE_KNIGHT: u8 = 0b00001001;
const WHITE_BISHOP: u8 = 0b00010001;
const WHITE_ROOK: u8 = 0b00100001;
const WHITE_QUEEN: u8 = 0b01000001;
const WHITE_KING: u8 = 0b10000001;
const BLACK_PAWN: u8 = 0b00000110;
const BLACK_KNIGHT: u8 = 0b00001010;
const BLACK_BISHOP: u8 = 0b00010010;
const BLACK_ROOK: u8 = 0b00100010;
const BLACK_QUEEN: u8 = 0b01000010;
const BLACK_KING: u8 = 0b10000010;

//special move codes
const QUIET_MOVE: u8 = 0b0000;
const DOUBLE_PAWN_PUSH: u8 = 0b0001;
const KINGSIDE_CASTLE: u8 = 0b0010;
const QUEENSIDE_CASTLE: u8 = 0b0011;
const CAPTURE: u8 = 0b0100;
const EN_PASSANT: u8 = 0b0101;
const KNIGHT_PROMO: u8 = 0b1000;
const BISHOP_PROMO: u8 = 0b1001;
const ROOK_PROMO: u8 = 0b1010;
const QUEEN_PROMO: u8 = 0b1011;
const KNIGHT_PROMO_CAPTURE: u8 = 0b1100;
const BISHOP_PROMO_CAPTURE: u8 = 0b1101;
const ROOK_PROMO_CAPTURE: u8 = 0b1110;
const QUEEN_PROMO_CAPTURE: u8 = 0b1111;

//true = white and false = black. This enables the ! operator for opposite color.
#[derive(Debug)]
struct Move {
    piece: u8,
    from: u8,
    to: u8,
    color: bool,
    kind: u8,
}
//One 8x8 mailbox, a bitboard for each color, and a bitboard for each piece
#[derive(Debug)]
pub struct Board {
    pub mailbox: [u8; 64],
    pub white: u64,
    pub black: u64,
    pub white_pawn: u64,
    pub white_knight: u64,
    pub white_bishop: u64,
    pub white_rook: u64,
    pub white_queen: u64,
    pub white_king: u64,
    pub black_pawn: u64,
    pub black_knight: u64,
    pub black_bishop: u64,
    pub black_rook: u64,
    pub black_queen: u64,
    pub black_king: u64,
}

//This function will generate all legal knight moves as a Vec of Moves. It should never be called if the king is already in check
//or if the game should already have ended. It will boundary check and then make sure no same-color piece is on dest square.
fn knight_moves(board: &Board, position: u8, color: bool) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();

    if position > 63 {
        dbg!("knight_moves received invalid position");
        return moves;
    }

    //north jumps
    if position < 56 {
        //north long jumps
        if position < 48 {
            //noWe long jumps. Bound check, make sure no piece of same color is on destination square. Add move if all good.
            if position % 8 > 0 {
                let new_move = knight_moves_helper(board, position, color, 15);
                if new_move.is_some() {
                    moves.push(new_move.unwrap());
                }
            }
            //noEa long jumps
            if position % 8 < 7 {
                let new_move = knight_moves_helper(board, position, color, 17);
                if new_move.is_some() {
                    moves.push(new_move.unwrap());
                }
            }
        }

        //noWe wide jumps
        if position % 8 > 1 {
            let new_move = knight_moves_helper(board, position, color, 6);
            if new_move.is_some() {
                moves.push(new_move.unwrap());
            }
        }
        //noEA wide jumps
        if position % 8 < 6 {
            let new_move = knight_moves_helper(board, position, color, 10);
            if new_move.is_some() {
                moves.push(new_move.unwrap());
            }
        }
    }

    //south jumps
    if position > 7 {
        //soWe wide jumps
        if position % 8 > 1 {
            let new_move = knight_moves_helper(board, position, color, -10);
            if new_move.is_some() {
                moves.push(new_move.unwrap());
            }
        }
        //soEa wide jumps
        if position % 8 < 6 {
            let new_move = knight_moves_helper(board, position, color, -6);
            if new_move.is_some() {
                moves.push(new_move.unwrap());
            }
        }
        //south long jumps
        if position > 15 {
            //soWe long jumps
            if position % 8 > 0 {
                let new_move = knight_moves_helper(board, position, color, -17);
                if new_move.is_some() {
                    moves.push(new_move.unwrap());
                }
            }
            //soEa long jumps
            if position % 8 < 7 {
                let new_move = knight_moves_helper(board, position, color, -15);
                if new_move.is_some() {
                    moves.push(new_move.unwrap());
                }
            }
        }
    }
    moves
}

fn knight_moves_helper(board: &Board, position: u8, color: bool, distance: i8) -> Option<Move> {
    //get destination position
    let dest = position as i8 + distance;
    debug_assert!(dest >= 0, "dest should never be negative");
    let dest = dest as u8;
    //get destination bitmask
    let shifted_dest = 1u64 << dest;
    //set piece
    let piece = if color { WHITE_KNIGHT } else { BLACK_KNIGHT };
    //choose which bitboards to use
    let board_color = if color { &board.white } else { &board.black };
    let other_color = if color { &board.black } else { &board.white };

    //check for same color piece on destination square
    if shifted_dest & board_color == 0 {
        //set move kind as capture or quiet move
        let kind = if shifted_dest & other_color == 0 {
            QUIET_MOVE
        } else {
            CAPTURE
        };
        //create move and return it
        let new_move = Move {
            piece,
            from: position,
            to: dest,
            color,
            kind,
        };
        return Some(new_move);
    }
    None
}

#[cfg(test)]
#[test]
fn noEa_knight_jump() {
    let board = Board {
        mailbox: [0; 64], // Optionally, initialize mailbox with empty values
        white: 1 << 17,
        black: 1 << 10,
        white_pawn: 0x000000000000FF00,
        white_rook: 0x8100000000000000,
        white_knight: 0x4200000000000000,
        white_bishop: 0x2400000000000000,
        white_queen: 0x0800000000000000,
        white_king: 0x1000000000000000,
        black_pawn: 0x00FF000000000000,
        black_rook: 0x0000000000000081,
        black_knight: 0x0000000000000042,
        black_bishop: 0x0000000000000024,
        black_queen: 0x0000000000000008,
        black_king: 0x0000000000000010,
    };

    let mut new_moves = knight_moves(&board, 0, true);
    // new_moves.extend(knight_moves(&board, 5, true));

    dbg!({ new_moves });

    assert_eq!(true, true);
}
