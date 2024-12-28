use crate::bitboard::init_bitboards;
use crate::utils::*;

// The Move representation
#[derive(Debug)]
struct Move {
    piece: u8,
    from: u8,
    to: u8,
    color: bool,
    kind: u8,
}

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
    pub turn: bool,
    pub white_kingside_castle: bool,
    pub white_queenside_castle: bool,
    pub black_kingside_castle: bool,
    pub black_queenside_castle: bool,
    // en-passant,
    pub ep_target: Option<u8>,
    pub halfmove: u16,
    pub fullmove: u16,
}
/*
// Takes in a board state and returns a Vec of all legal moves
fn legal_moves(board: &Board) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    let (pawns, knights, bishops, rooks, queens, king);
    // THE FOLLOWING LINE WILL BE INITIALIZED ELSWHERE. bitboards will be held in bitboard.rs as a
    // global variable.
    let ((rook_attacks, rook_magics), (bishop_attacks, bishop_magics)) = init_bitboards();

    if board.turn {
        knights = board.white_knight;
        bishops = board.white_bishop;
        rooks = board.white_rook;
        queens = board.white_queen;
        king = board.white_king;
    } else {
        knights = board.black_knight;
        bishops = board.black_bishop;
        rooks = board.black_rook;
        queens = board.black_queen;
        king = board.black_king;
    }

    // Add pawn moves
    moves.extend(pawn_moves(board));

    // Add rook moves
    let rook_positions = set_bit_positions(rooks);
    for position in rook_positions {
        moves.extend(rook_moves(
            board,
            position,
            &rook_attacks[position as usize],
            rook_magics[position as usize],
        ));
    }

    // Add bishop moves
    let bishop_positions = set_bit_positions(bishops);
    for position in bishop_positions {
        moves.extend(bishop_moves(
            board,
            position,
            &bishop_attacks[position as usize],
            bishop_magics[position as usize],
        ));
    }

    // Add knight moves
    let knight_positions = set_bit_positions(knights);
    for position in knight_positions {
        moves.extend(knight_moves(board, position));
    }

    // Add queen moves
    let queen_positions = set_bit_positions(queens);
    for position in queen_positions {
        moves.extend(queen_moves(board, position));
    }

    // Add king moves
    let king_position = set_bit_positions(king);
    for position in king_position {
        moves.extend(king_moves(board, position));
    }

    // TODO call helper functions for each piece type (pass along appropriate color)

    moves
}
*/

/* A GOOD REFACTORING GUIDE FOR PAWN_MOVES
* fn calculate_left_captures(board: &Board, pawns: u64) -> u64 {
    let (shift, file_mask, rank_mask, friendly_pieces, enemy_pieces) = if board.turn {
        (7, !H_FILE, !EIGHTH_RANK, board.white, board.black)
    } else {
        (-7, !A_FILE, !FIRST_RANK, board.black, board.white)
    };

    let mut targets = enemy_pieces;
    if let Some(ep_square) = board.ep_target {
        targets |= 1 << ep_square;
    }

    let possible_captures = if shift > 0 {
        pawns << shift
    } else {
        pawns >> shift.abs()
    };

    possible_captures & file_mask & rank_mask & !friendly_pieces & targets
} */
fn pawn_moves(board: &Board) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();

    let (pawns, piece);

    if board.turn {
        pawns = board.white_pawn;
        piece = WHITE_PAWN;
    } else {
        pawns = board.black_pawn;
        piece = BLACK_PAWN;
    };

    // forward single move
    let mut forward_single_move: u64 = if board.turn {
        (pawns << 8) & !EIGHTH_RANK
    } else {
        (pawns >> 8) & !FIRST_RANK
    };
    forward_single_move &= !(board.white | board.black);
    let forward_single_positions = set_bit_positions(forward_single_move);
    for destination in forward_single_positions {
        let from = if board.turn {
            destination - 8
        } else {
            destination + 8
        };
        let new_move = Move {
            piece,
            from,
            to: destination,
            color: board.turn,
            kind: QUIET_MOVE,
        };
        // make the move, make sure it doesnt leave king in check
        moves.push(new_move);
    }

    // forward double move
    let mut forward_double_move: u64 = if board.turn {
        (((pawns & SECOND_RANK) << 8) & !(board.white | board.black)) << 8
    } else {
        (((pawns & SEVENTH_RANK) >> 8) & !(board.white | board.black)) >> 8
    };
    forward_double_move &= !(board.white | board.black);
    let forward_double_positions = set_bit_positions(forward_double_move);
    for destination in forward_double_positions {
        let from = if board.turn {
            destination - 16
        } else {
            destination + 16
        };
        let new_move = Move {
            piece,
            from,
            to: destination,
            color: board.turn,
            kind: DOUBLE_PAWN_PUSH,
        };
        // make the move, make sure it doesnt leave king in check
        moves.push(new_move);
    }

    // left capture
    let mut left_capture: u64 = if board.turn {
        let mut tmp = (pawns << 7) & !H_FILE & !EIGHTH_RANK & !board.white;
        let mut black_targets = board.black;
        if board.ep_target.is_some() {
            black_targets |= 1 << board.ep_target.unwrap();
        }
        tmp &= black_targets;
        tmp
    } else {
        let mut tmp = (pawns >> 7) & !A_FILE & !FIRST_RANK & !board.black;
        let mut white_targets = board.white;
        if board.ep_target.is_some() {
            white_targets |= 1 << board.ep_target.unwrap();
        }
        tmp &= white_targets;
        tmp
    };
    let left_capture_positions = set_bit_positions(left_capture);
    for destination in left_capture_positions {
        let move_type = match board.ep_target {
            Some(ep_square) if destination == ep_square => EN_PASSANT,
            _ => CAPTURE,
        };
        let from = if board.turn {
            destination - 7
        } else {
            destination + 7
        };
        let new_move = Move {
            piece,
            from,
            to: destination,
            color: board.turn,
            kind: move_type,
        };
        // if doesnt leave king in check
        moves.push(new_move);
    }

    // right captures
    let mut right_capture: u64 = if board.turn {
        let mut tmp = (pawns << 9) & !A_FILE & !EIGHTH_RANK & !board.white;
        let mut black_targets = board.black;
        if board.ep_target.is_some() {
            black_targets |= 1 << board.ep_target.unwrap();
        }
        tmp &= black_targets;
        tmp
    } else {
        let mut tmp = (pawns >> 9) & !H_FILE & !FIRST_RANK & !board.black;
        let mut white_targets = board.white;
        if board.ep_target.is_some() {
            white_targets |= 1 << board.ep_target.unwrap();
        }
        tmp &= white_targets;
        tmp
    };
    let right_capture_positions = set_bit_positions(right_capture);
    for destination in right_capture_positions {
        let move_type = match board.ep_target {
            Some(ep_square) if destination == ep_square => EN_PASSANT,
            _ => CAPTURE,
        };
        let from = if board.turn {
            destination - 9
        } else {
            destination + 9
        };
        let new_move = Move {
            piece,
            from,
            to: destination,
            color: board.turn,
            kind: move_type,
        };
        // if doesnt leave king in check
        moves.push(new_move);
    }

    moves
}

/*
fn king_moves(board: &Board, position: u8) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();

    moves
}

fn knight_moves(board: &Board, position: u8) -> Vec<Move> {
    debug_assert!(
        position <= 63,
        "knight_moves received invalid position: {}",
        position
    );
    //set piece and colors
    let piece = if board.turn {
        WHITE_KNIGHT
    } else {
        BLACK_KNIGHT
    };
    let board_color = if board.turn {
        &board.white
    } else {
        &board.black
    };
    let other_color = if board.turn {
        &board.black
    } else {
        &board.white
    };

    // All the destinations it could jump (ignoring leaving the king in check)
    let potentials = KNIGHT_MOVE_MASKS[position as usize] & !board_color;

    moves_helper(piece, position, potentials, board)
}

fn moves_helper(piece: u8, position: u8, potentials: u64, board: &Board) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    let potential_list = set_bit_positions(potentials);
    // TODO go thru potential list, make the move, and send resulting board to in_check and make
    // sure it's false. if it's false, add the move to the final list

    moves
}

// Checks if a given color's king is in check
fn in_check(board: &Board, color: bool) -> bool {}

// This function checks if a potential move is legal. If it is, it creates a Move instance and returns it.
// If the path is obstructed by either color piece,
// it sets the obstructed boolean flag to true. If the move is illegal, it returns None.
fn move_helpers(
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

//This function will generate all legal knight moves as a Vec of Moves. It should never be called if the king is already in check
//or if the game should already have ended. It will boundary check and then make sure no same-color piece is on dest square.
fn knight_movers(board: &Board, position: u8) -> Vec<Move> {
    debug_assert!(
        position <= 63,
        "knight_moves received invalid position: {}",
        position
    );
    let mut moves: Vec<Move> = Vec::new();
    //set piece and colors
    let piece = if board.turn {
        WHITE_KNIGHT
    } else {
        BLACK_KNIGHT
    };
    let board_color = if board.turn {
        &board.white
    } else {
        &board.black
    };
    let other_color = if board.turn {
        &board.black
    } else {
        &board.white
    };

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

fn bishop_moves(board: &Board, position: u8, attacks: &Vec<u64>, magic: u64) -> Vec<Move> {
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

    // TODO use bitboard and mask with board state

    moves
}

fn rook_moves(board: &Board, position: u8, attacks: &Vec<u64>, magic: u64) -> Vec<Move> {
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

    // TODO use bitboard and mask with board state

    moves
}

fn queen_moves(board: &Board, position: u8) -> Vec<Move> {
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
                    kind: QUIET_MOVE,
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
                            kind: QUIET_MOVE,
                        };
                        moves.push(new_move);
                    }
                }
            }
        } // end single north push non-promo

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
                    kind: KNIGHT_PROMO,
                };
                moves.push(promote_knight);
                let promote_bishop = Move {
                    piece,
                    from: position,
                    to: position + 8,
                    color,
                    kind: BISHOP_PROMO,
                };
                moves.push(promote_bishop);
                let promote_rook = Move {
                    piece,
                    from: position,
                    to: position + 8,
                    color,
                    kind: ROOK_PROMO,
                };
                moves.push(promote_rook);
                let promote_queen = Move {
                    piece,
                    from: position,
                    to: position + 8,
                    color,
                    kind: QUEEN_PROMO,
                };
                moves.push(promote_queen);
            }
        } // end promotion (non-capture)

        //noWe capture
        if position % 8 > 0 {
            //noWe promo capture

            if (47 < position) && (position < 56) {
                //set mask for captured square
                let shifted_dest = 1 << (position + 7);
                //make sure opposite color piece there

                if (shifted_dest & board.black) == shifted_dest {
                    let promo_knight_capture = Move {
                        piece,
                        from: position,
                        to: position + 7,
                        color,
                        kind: KNIGHT_PROMO_CAPTURE,
                    };
                    moves.push(promo_knight_capture);
                    let promo_bishop_capture = Move {
                        piece,
                        from: position,
                        to: position + 7,
                        color,
                        kind: BISHOP_PROMO_CAPTURE,
                    };
                    moves.push(promo_bishop_capture);
                    let promo_rook_capture = Move {
                        piece,
                        from: position,
                        to: position + 7,
                        color,
                        kind: ROOK_PROMO_CAPTURE,
                    };
                    moves.push(promo_rook_capture);
                    let promo_queen_capture = Move {
                        piece,
                        from: position,
                        to: position + 7,
                        color,
                        kind: QUEEN_PROMO_CAPTURE,
                    };
                    moves.push(promo_queen_capture);
                }
            }
            //non-promo noWe capture
            else {
                //set mask for captured square
                let shifted_dest = 1 << (position + 7);
                //make sure opposite color piece there
                if (shifted_dest & board.black) == shifted_dest {
                    let capture = Move {
                        piece,
                        from: position,
                        to: position + 7,
                        color,
                        kind: CAPTURE,
                    };
                    moves.push(capture);
                }
            }
        } // end noWe capture

        //noEa capture
        if position % 8 < 7 {
            //noEa promo capture
            if (47 < position) && (position < 56) {
                //set mask for captured square
                let shifted_dest = 1 << (position + 9);
                //make sure opposite color piece there
                if (shifted_dest & board.black) == shifted_dest {
                    let promo_knight_capture = Move {
                        piece,
                        from: position,
                        to: position + 9,
                        color,
                        kind: KNIGHT_PROMO_CAPTURE,
                    };
                    moves.push(promo_knight_capture);
                    let promo_bishop_capture = Move {
                        piece,
                        from: position,
                        to: position + 9,
                        color,
                        kind: BISHOP_PROMO_CAPTURE,
                    };
                    moves.push(promo_bishop_capture);
                    let promo_rook_capture = Move {
                        piece,
                        from: position,
                        to: position + 9,
                        color,
                        kind: ROOK_PROMO_CAPTURE,
                    };
                    moves.push(promo_rook_capture);
                    let promo_queen_capture = Move {
                        piece,
                        from: position,
                        to: position + 9,
                        color,
                        kind: QUEEN_PROMO_CAPTURE,
                    };
                    moves.push(promo_queen_capture);
                }
            }
            //non-promo noEa capture
            else {
                //set mask for captured square
                let shifted_dest = 1 << (position + 9);
                //make sure opposite color piece there
                if (shifted_dest & board.black) == shifted_dest {
                    let capture = Move {
                        piece,
                        from: position,
                        to: position + 9,
                        color,
                        kind: CAPTURE,
                    };
                    moves.push(capture);
                }
            }
        }
    }
    //end white pawn block

    //black pawn
    else {
    }
    moves
}
*/

fn create_test_board() -> Board {
    Board {
        mailbox: [0; 64],
        white: 0,
        black: 0,
        white_pawn: 0,
        white_knight: 0,
        white_bishop: 0,
        white_rook: 0,
        white_queen: 0,
        white_king: 0,
        black_pawn: 0,
        black_knight: 0,
        black_bishop: 0,
        black_rook: 0,
        black_queen: 0,
        black_king: 0,
        turn: true,
        white_kingside_castle: true,
        white_queenside_castle: true,
        black_kingside_castle: true,
        black_queenside_castle: true,
        ep_target: None,
        halfmove: 0,
        fullmove: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;

    #[test]
    fn pawn_no_promo() {
        let mut board = create_test_board();
        board.white_pawn = (1 << 10) | (1 << 17) | (1 << 28) | (1 << 36) |  (1 << 38) | (1 << 42);
        board.white |= board.white_pawn;
        board.black_pawn = (1 << 24) | (1 << 25) | (1 << 35) | (1 << 37) | (1 << 49);
        board.black |= board.black_pawn;
        board.ep_target = Some(45);
        let moves = pawn_moves(&board);
        for one_move in moves {
            println!("{:?}", one_move);
        }

    }
    #[test]
    fn pawn_left_captures() {
        let mut board = create_test_board();
        board.white_pawn = (1 << 17) | (1 << 28) | (1 << 38) | (1 << 42);
        board.white |= board.white_pawn;
        board.black_pawn = (1 << 24) | (1 << 25) | (1 << 37) | (1 << 49);
        board.black |= board.black_pawn;
        board.ep_target = Some(45);
        let moves = pawn_moves(&board);
        for one_move in moves {
            println!("{:?}", one_move);
        }
    }
    #[test]
    fn first_rank_pawns() {
        let mut board = create_test_board();
        board.white_pawn = SECOND_RANK;
        board.white_pawn |= 1 << 18;
        board.white |= board.white_pawn;
        board.black_pawn = FOURTH_RANK;
        board.black |= board.black_pawn;
        board.black &= !(B_FILE | F_FILE);
        let moves = pawn_moves(&board);
        for one_move in moves {
            println!("{:?}", one_move);
        }
    }
}
