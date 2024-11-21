//TODO: revisit whether or not Move needs a color field. if not, clean up all the code that relies on it.


//TODO pawn moves, then in-check function, then king moves, then update all move functions to avoid putting own king in check.

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
    

    debug_assert!(
        position <= 63,
        "knight_moves received invalid position: {}",
        position
    );
    let mut moves: Vec<Move> = Vec::new();
    //set piece and colors
    let piece = if color { WHITE_KNIGHT } else { BLACK_KNIGHT };
    let board_color = if color { &board.white } else { &board.black };
    let other_color = if color { &board.black } else { &board.white };

    //north jumps
    if position < 56 {
        //north long jumps
        if position < 48 {
            //noWe long jumps. Bound check, make sure no piece of same color is on destination square. Add move if all good.
            if position % 8 > 0 {
                let (new_move, _) =
                    move_helper(position, color, 15, piece, board_color, other_color);
                if new_move.is_some() {
                    moves.push(new_move.unwrap());
                }
            }
            //noEa long jumps
            if position % 8 < 7 {
                let (new_move, _) =
                    move_helper(position, color, 17, piece, board_color, other_color);
                if new_move.is_some() {
                    moves.push(new_move.unwrap());
                }
            }
        }

        //noWe wide jumps
        if position % 8 > 1 {
            let (new_move, _) = move_helper(position, color, 6, piece, board_color, other_color);
            if new_move.is_some() {
                moves.push(new_move.unwrap());
            }
        }
        //noEA wide jumps
        if position % 8 < 6 {
            let (new_move, _) = move_helper(position, color, 10, piece, board_color, other_color);
            if new_move.is_some() {
                moves.push(new_move.unwrap());
            }
        }
    }

    //south jumps
    if position > 7 {
        //soWe wide jumps
        if position % 8 > 1 {
            let (new_move, _) = move_helper(position, color, -10, piece, board_color, other_color);
            if new_move.is_some() {
                moves.push(new_move.unwrap());
            }
        }
        //soEa wide jumps
        if position % 8 < 6 {
            let (new_move, _) = move_helper(position, color, -6, piece, board_color, other_color);
            if new_move.is_some() {
                moves.push(new_move.unwrap());
            }
        }
        //south long jumps
        if position > 15 {
            //soWe long jumps
            if position % 8 > 0 {
                let (new_move, _) =
                    move_helper(position, color, -17, piece, board_color, other_color);
                if new_move.is_some() {
                    moves.push(new_move.unwrap());
                }
            }
            //soEa long jumps
            if position % 8 < 7 {
                let (new_move, _) =
                    move_helper(position, color, -15, piece, board_color, other_color);
                if new_move.is_some() {
                    moves.push(new_move.unwrap());
                }
            }
        }
    }
    moves
}

fn bishop_moves(board: &Board, position: u8, color: bool) -> Vec<Move> {
    debug_assert!(
        position <= 63,
        "bishop_moves received invalid position: {}",
        position
    );
    let mut moves: Vec<Move> = Vec::new();
    //set piece and colors
    let piece = if color { WHITE_BISHOP } else { BLACK_BISHOP };
    let board_color = if color { &board.white } else { &board.black };
    let other_color = if color { &board.black } else { &board.white };

    let mut obstructed = false;
    //noWe loop
    let mut distance: i8 = 7;
    if (position < 56) && (position % 8 > 0) {
        //check that it doesn't get too high, wrap around, or hit anything
        while (position as i8 + distance <= 63)
            && ((position as i8 + distance) % 8 < 7)
            && !obstructed
        {
            let (new_move, is_blocked) =
                move_helper(position, color, distance, piece, board_color, other_color);
            if new_move.is_some() {
                moves.push(new_move.unwrap());
            }
            obstructed = is_blocked;
            distance += 7;
        }
    }
    obstructed = false;
    //noEa loop
    if (position < 56) && (position % 8 < 7) {
        distance = 9;
        //check that it doesn't get too high, wrap around, or hit anything
        while (position as i8 + distance <= 63)
            && ((position as i8 + distance) % 8 > 0)
            && !obstructed
        {
            let (new_move, is_blocked) =
                move_helper(position, color, distance, piece, board_color, other_color);
            if new_move.is_some() {
                moves.push(new_move.unwrap());
            }
            obstructed = is_blocked;
            distance += 9;
        }
    }
    obstructed = false;
    //soWe loop
    if (position > 7) && (position % 8 > 0) {
        distance = -9;
        //check that it doesn't get too high, wrap around, or hit anything
        while (0 <= position as i8 + distance)
            && ((position as i8 + distance) % 8 < 7)
            && !obstructed
        {
            let (new_move, is_blocked) =
                move_helper(position, color, distance, piece, board_color, other_color);
            if new_move.is_some() {
                moves.push(new_move.unwrap());
            }
            obstructed = is_blocked;
            distance -= 9;
        }
    }
    obstructed = false;
    //soEa loop
    if (position > 7) && (position % 8 < 7) {
        distance = -7;
        //check that it doesn't get too high, wrap around, or hit anything
        while (0 <= position as i8 + distance)
            && ((position as i8 + distance) % 8 > 0)
            && !obstructed
        {
            let (new_move, is_blocked) =
                move_helper(position, color, distance, piece, board_color, other_color);
            if new_move.is_some() {
                moves.push(new_move.unwrap());
            }
            obstructed = is_blocked;
            distance -= 7;
        }
    }

    moves
}

fn rook_moves(board: &Board, position: u8, color: bool) -> Vec<Move> {
    debug_assert!(
        position <= 63,
        "rook_moves received invalid position: {}",
        position
    );
    let mut moves: Vec<Move> = Vec::new();

    
    //set piece and colors
    let piece = if color { WHITE_ROOK } else { BLACK_ROOK };
    let board_color = if color { &board.white } else { &board.black };
    let other_color = if color { &board.black } else { &board.white };

    let mut obstructed = false;
    //north loop
    let mut distance: i8 = 8;
    if position < 56 {
        //check that it doesn't get too high or hit anything
        while (position as i8 + distance <= 63) && !obstructed {
            let (new_move, is_blocked) =
                move_helper(position, color, distance, piece, board_color, other_color);
            if new_move.is_some() {
                moves.push(new_move.unwrap());
            }
            obstructed = is_blocked;
            distance += 8;
        }
    }
    obstructed = false;
    //west loop
    if position % 8 > 0 {
        distance = -1;
        //check that it doesn't wrap around or hit anything. >= 0 check needed because % is remainder NOT modulus
        while ((position as i8 + distance) >= 0)
            && (((position as i8 + distance) % 8) < 7)
            && !obstructed
        {
            let (new_move, is_blocked) =
                move_helper(position, color, distance, piece, board_color, other_color);
            if new_move.is_some() {
                moves.push(new_move.unwrap());
            }
            obstructed = is_blocked;
            distance -= 1;
        }
    }
    obstructed = false;
    //east loop
    if position % 8 < 7 {
        distance = 1;
        //check that it doesn't get too high, wrap around, or hit anything
        while ((position as i8 + distance) % 8 > 0) && !obstructed {
            let (new_move, is_blocked) =
                move_helper(position, color, distance, piece, board_color, other_color);
            if new_move.is_some() {
                moves.push(new_move.unwrap());
            }
            obstructed = is_blocked;
            distance += 1;
        }
    }
    obstructed = false;
    //south loop
    if position > 7 {
        distance = -8;
        //check that it doesn't get too low or hit anything
        while (0 <= position as i8 + distance) && !obstructed {
            let (new_move, is_blocked) =
                move_helper(position, color, distance, piece, board_color, other_color);
            if new_move.is_some() {
                moves.push(new_move.unwrap());
            }
            obstructed = is_blocked;
            distance -= 8;
        }
    }

    moves
}

fn queen_moves(board: &Board, position: u8, color: bool) -> Vec<Move> {
    debug_assert!(
        position <= 63,
        "queen_moves received invalid position: {}",
        position
    );
    let mut moves = bishop_moves(board, position, color);
    moves.extend(rook_moves(board, position, color));
    moves
}
        

//TODO en passant
fn pawn_moves(board: &Board, position: u8, color: bool) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    

    debug_assert!(
        position <= 56,
        "pawn_moves received invalid position: {}",
        position
    );

    let piece = if color { WHITE_ROOK } else { BLACK_ROOK };
    //white pawn
    if (color) {
        
        //north single push (non-promotion)
        if (position < 48) {
            //create mask for one square north
            let shifted_dest = 1u64 << (position + 8);
            //only add move if square one north is completely empty
            if shifted_dest & (board.black | board.white) == 0 {
                let new_move = Move {
                    piece,
                    from: position,
                    to: position + 8,
                    color,
                    kind: QUIET_MOVE 
                };
                moves.push(new_move);
                //north double push
                if (position < 16) {
                    //mask for two squares north
                    let shifted_dest = 1u64 << (position + 16);
                    //only add move if two squares north is empty (already confirmed one square north was empty)
                    if shifted_dest & (board.black | board.white) == 0 {
                        let new_move = Move {
                            piece,
                            from: position,
                            to: position + 16,
                            color,
                            kind: QUIET_MOVE 
                        };
                        moves.push(new_move);
                    }
                }
            }
        }
            
        //promotion (non-capture)
        if (47 < position) && (position < 56) {
           
            //create mask for one square north
            let shifted_dest = 1u64 << (position + 8);
            //only add move if square one north is completely empty
            if shifted_dest & (board.black | board.white) == 0 {
                let promote_knight = Move {
                    piece,
                    from: position,
                    to: position + 8,
                    color,
                    kind: KNIGHT_PROMO 
                };
                moves.push(promote_knight);
                let promote_bishop = Move {
                    piece,
                    from: position,
                    to: position + 8,
                    color,
                    kind: BISHOP_PROMO 
                };
                moves.push(promote_bishop);
                let promote_rook = Move {
                    piece,
                    from: position,
                    to: position + 8,
                    color,
                    kind: ROOK_PROMO 
                };
                moves.push(promote_rook);
                let promote_queen = Move {
                    piece,
                    from: position,
                    to: position + 8,
                    color,
                    kind: QUEEN_PROMO 
                };
                moves.push(promote_queen);
                return moves;
            }
        }
        //noWe capture
        

        //noEa capture
            

        } //end white pawn block
        
    
    //black pawn
    else {
        
    }
    moves
}


//This function checks if a potential move is legal. If it is, it creates a Move instance and returns it. If the path is obstructed by either color piece,
// it sets the obstructed boolean flag to true. If the move is illegal, it returns None.
fn move_helper(
    position: u8,
    color: bool,
    distance: i8,
    piece: u8,
    board_color: &u64,
    other_color: &u64,
) -> (Option<Move>, bool) {
    //get destination position
    let dest = position as i8 + distance;
    debug_assert!(dest >= 0, "dest should never be negative");
    let dest = dest as u8;
    //get destination bitmask
    let shifted_dest = 1u64 << dest;

    let mut obstructed = true;
    //check for same color piece on destination square
    if shifted_dest & board_color == 0 {
        //set move kind as capture or quiet move
        let kind = if shifted_dest & other_color == 0 {
            obstructed = false;
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
        return (Some(new_move), obstructed);
    }
    (None, obstructed)
}

fn create_test_board() -> Board {
    Board {
        mailbox: [0; 64],
        white: 0,
        black: 0,
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
    }
}
#[cfg(test)]
#[test]
fn knight_jumps() {
    let mut board = create_test_board();
    let mut new_moves = knight_moves(&board, 0, true);
    // new_moves.extend(knight_moves(&board, 5, true));
    let length = new_moves.len();
    // dbg!({ new_moves });
    board.white = 1 << 44;
    let new_moves2 = knight_moves(&board, 27, true);
    let length2 = new_moves2.len();

    board.white = 0;
    board.black = (1 << 38) | (1 << 6);
    let new_moves3 = knight_moves(&board, 21, true);
    let length3 = new_moves3.len();
    assert_eq!(length, 2);
    assert_eq!(length2, 7);
    assert_eq!(length3, 8);
}

#[test]
fn bishop_test() {
    let mut board = create_test_board();
    let new_moves1 = bishop_moves(&board, 1, true);
    let length1 = new_moves1.len();
    // dbg!({ new_moves1 });

    let new_moves2 = bishop_moves(&board, 27, false);
    let length2 = new_moves2.len();
    // dbg!({ new_moves2 });
    board.black = (1 << 42) | (1 << 14);
    let new_moves3 = bishop_moves(&board, 21, true);
    let length3 = new_moves3.len();
    // dbg!({ new_moves3 });
    assert_eq!(length1, 7);
    assert_eq!(length2, 13);
    assert_eq!(length3, 8);
}

#[test]
fn rook_test() {
    let mut board = create_test_board();
    let new_moves1 = rook_moves(&board, 1, true);
    let length1 = new_moves1.len();
    // dbg!({ new_moves1 });

    let new_moves2 = rook_moves(&board, 27, false);
    let length2 = new_moves2.len();
    // dbg!({ new_moves2 });
    board.black = (1 << 53) | (1 << 20);
    let new_moves3 = rook_moves(&board, 21, true);
    let length3 = new_moves3.len();
    // dbg!({ new_moves3 });
    assert_eq!(length1, 14);
    assert_eq!(length2, 14);
    assert_eq!(length3, 9);
}

#[test]
fn queen_test() {
    let mut board = create_test_board();
    let new_moves1 = queen_moves(&board, 1, true);
    let length1 = new_moves1.len();
    // dbg!({ new_moves1 });

    board.white = 1 << 30;
    let new_moves2 = queen_moves(&board, 27, true);
    let length2 = new_moves2.len();
    // dbg!({ new_moves2 });
    board.white = 0;
    board.black = (1 << 53) | (1 << 20) | (1 << 42);
    let new_moves3 = queen_moves(&board, 21, true);
    let length3 = new_moves3.len();
    // dbg!({ new_moves3 });
    assert_eq!(length1, 21);
    assert_eq!(length2, 25);
    assert_eq!(length3, 18);
}

#[test]
fn pawn_test() {
    let mut board = create_test_board();
    let new_moves1 = pawn_moves(&board, 53, true);
    let length1 = new_moves1.len();
    dbg!({ new_moves1 });

    assert_eq!(length1, 4);

}

