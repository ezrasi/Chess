use crate::bitboard::MAGIC_TABLES;
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

/* A GOOD REFACTORING GUIDE FOR PAWN_MOVES (from Claude)
fn pawn_moves(board: &Board) -> Vec<Move> {
    let (pawns, piece, forward_shift, double_start_rank, promotion_rank) = if board.turn {
        (board.white_pawn, WHITE_PAWN, 8, SECOND_RANK, !EIGHTH_RANK)
    } else {
        (board.black_pawn, BLACK_PAWN, -8, SEVENTH_RANK, !FIRST_RANK)
    };

    let empty = !(board.white | board.black);
    let mut moves = Vec::new();

    // Helper function to create moves
    let create_move = |from: u8, to: u8, kind: u8| Move {
        piece, from, to, color: board.turn, kind
    };

    // Single moves
    let single_moves = shift_bits(pawns, forward_shift) & empty & promotion_rank;
    for to in set_bit_positions(single_moves) {
        moves.push(create_move(
            (to as i8 - forward_shift) as u8,
            to,
            QUIET_MOVE
        ));
    }

    // Double moves
    let double_moves = shift_bits(
        shift_bits(pawns & double_start_rank, forward_shift) & empty,
        forward_shift
    ) & empty;
    for to in set_bit_positions(double_moves) {
        moves.push(create_move(
            (to as i8 - forward_shift * 2) as u8,
            to,
            DOUBLE_PAWN_PUSH
        ));
    }

    // Captures
    let enemy = if board.turn { board.black } else { board.white };
    let enemy_with_ep = enemy | board.ep_target.map_or(0, |sq| 1 << sq);

    // Helper for capture moves
    let add_captures = |shift: i8, file_mask: u64| {
        let captures = shift_bits(pawns, shift) & file_mask & promotion_rank & enemy_with_ep;
        for to in set_bit_positions(captures) {
            let kind = if Some(to) == board.ep_target { EN_PASSANT } else { CAPTURE };
            moves.push(create_move(
                (to as i8 - shift) as u8,
                to,
                kind
            ));
        }
    };

    add_captures(7, !H_FILE);  // Left captures
    add_captures(9, !A_FILE);  // Right captures

    moves
}

// Helper function to handle both positive and negative shifts
fn shift_bits(bits: u64, shift: i8) -> u64 {
    if shift >= 0 {
        bits << shift
    } else {
        bits >> -shift
    }
}*/
fn pawn_moves(board: &Board) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();

    let (pawns, piece) = if board.turn {
        (board.white_pawn, WHITE_PAWN)
    } else {
        (board.black_pawn, BLACK_PAWN)
    };

    let is_promo = |dest: u8| -> bool {
        if board.turn {
            dest >= 56
        } else {
            dest <= 7
        }
    };

    let add_promo_moves = |moves: &mut Vec<Move>, from: u8, to: u8, is_capture: bool| {
        let kinds = if is_capture {
            vec![
                KNIGHT_PROMO_CAPTURE,
                BISHOP_PROMO_CAPTURE,
                ROOK_PROMO_CAPTURE,
                QUEEN_PROMO_CAPTURE,
            ]
        } else {
            vec![KNIGHT_PROMO, BISHOP_PROMO, ROOK_PROMO, QUEEN_PROMO]
        };

        for kind in kinds {
            moves.push(Move {
                piece,
                from,
                to,
                color: board.turn,
                kind,
            });
        }
    };
    let add_regular_move = |moves: &mut Vec<Move>, from: u8, to: u8, kind: u8| {
        moves.push(Move {
            piece,
            from,
            to,
            color: board.turn,
            kind,
        });
    };

    // forward single move
    let mut forward_single_move: u64 = if board.turn { pawns << 8 } else { pawns >> 8 };
    forward_single_move &= !(board.white | board.black);
    let forward_single_positions = set_bit_positions(forward_single_move);

    for destination in forward_single_positions {
        let from = if board.turn {
            destination - 8
        } else {
            destination + 8
        };

        if is_promo(destination) {
            add_promo_moves(&mut moves, from, destination, false);
        } else {
            add_regular_move(&mut moves, from, destination, QUIET_MOVE);
        }
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
        add_regular_move(&mut moves, from, destination, DOUBLE_PAWN_PUSH);
    }

    // left capture
    let left_capture: u64 = if board.turn {
        let mut tmp = (pawns << 7) & !H_FILE & !board.white;
        let mut black_targets = board.black;
        if board.ep_target.is_some() {
            black_targets |= 1 << board.ep_target.unwrap();
        }
        tmp &= black_targets;
        tmp
    } else {
        let mut tmp = (pawns >> 7) & !A_FILE & !board.black;
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

        if is_promo(destination) {
            add_promo_moves(&mut moves, from, destination, true);
        } else {
            add_regular_move(&mut moves, from, destination, move_type);
        }
    }

    // right captures
    let right_capture: u64 = if board.turn {
        let mut tmp = (pawns << 9) & !A_FILE & !board.white;
        let mut black_targets = board.black;
        if board.ep_target.is_some() {
            black_targets |= 1 << board.ep_target.unwrap();
        }
        tmp &= black_targets;
        tmp
    } else {
        let mut tmp = (pawns >> 9) & !H_FILE & !board.black;
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
        if is_promo(destination) {
            add_promo_moves(&mut moves, from, destination, true);
        } else {
            add_regular_move(&mut moves, from, destination, move_type);
        }
    }

    moves
}

// Checks if the king of the current player is in check
fn in_check(board: &Board) -> bool {
    // check knight squares
    // check diagonals until obstructed for queens/bishops
    //      check appropriate squares for pawn attacks
    // check straights until obstructed for queens/rooks
    // check also for king proximity
    let (
        king,
        other_color,
        other_pawn,
        other_knight,
        other_bishop,
        other_rook,
        other_queen,
        other_king,
    ) = if board.turn {
        (
            board.white_king,
            board.black,
            board.black_pawn,
            board.black_knight,
            board.black_bishop,
            board.black_rook,
            board.black_queen,
            board.black_king,
        )
    } else {
        (
            board.black_king,
            board.white,
            board.white_pawn,
            board.white_knight,
            board.white_bishop,
            board.white_rook,
            board.white_queen,
            board.white_king,
        )
    };

    let king_pos_usize = set_bit_positions(king)[0] as usize;

    if KNIGHT_MOVE_MASKS[king_pos_usize] & other_knight != 0 {
        return true;
    };

    // diagonals
    let mut diagonal_blockers = (board.white | board.black) & BISHOP_MOVE_MASKS[king_pos_usize];
    if king_pos_usize < 56 {
        diagonal_blockers &= !EIGHTH_RANK;
    }
    if king_pos_usize > 7 {
        diagonal_blockers &= !FIRST_RANK;
    }
    if king_pos_usize % 8 < 7 {
        diagonal_blockers &= !H_FILE;
    }
    if king_pos_usize % 8 > 0 {
        diagonal_blockers &= !A_FILE;
    }

    let index = diagonal_blockers.wrapping_mul(MAGIC_TABLES.bishop_magics[king_pos_usize])
        >> (64 - BBITS[king_pos_usize]);

    let diagonal_encounters =
        MAGIC_TABLES.bishop_attacks[king_pos_usize][index as usize] & other_color;

    if (diagonal_encounters & other_bishop) | (diagonal_encounters & other_queen) != 0 {
        return true;
    }
    // pawns
    let relevant_pawns = diagonal_encounters & other_pawn;
    if (relevant_pawns) != 0 {
        let pawn_pos = set_bit_positions(relevant_pawns);
        for pos in pawn_pos {
            if board.turn {
                if (pos == (king_pos_usize as u8) + 7) | (pos == (king_pos_usize as u8) + 9) {
                    return true;
                }
            } else {
                if (pos == (king_pos_usize as u8) - 9) | (pos == (king_pos_usize as u8) - 7) {
                    return true;
                }
            }
        }
    }

    // straights
    let mut straight_blockers = (board.white | board.black) & ROOK_MOVE_MASKS[king_pos_usize];
    if king_pos_usize < 56 {
        straight_blockers &= !EIGHTH_RANK;
    }
    if king_pos_usize > 7 {
        straight_blockers &= !FIRST_RANK;
    }
    if king_pos_usize % 8 < 7 {
        straight_blockers &= !H_FILE;
    }
    if king_pos_usize % 8 > 0 {
        straight_blockers &= !A_FILE;
    }
    let index = straight_blockers.wrapping_mul(MAGIC_TABLES.rook_magics[king_pos_usize])
        >> (64 - RBITS[king_pos_usize]);

    let straight_encounters = MAGIC_TABLES.rook_attacks[king_pos_usize][index as usize] & other_color;

    if (straight_encounters & other_rook) | (straight_encounters & other_queen) != 0 {
        return true;
    }

    // king proximity
    if KING_MOVE_MASKS[king_pos_usize] & other_king != 0 {
        return true;
    }

    false
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
    fn in_check_02() {
        let mut board = create_test_board();
        
        board.white_king = 1 << 14;
        board.white |= board.white_king;
        board.white_pawn = 1 << 28;
        board.white |= board.white_pawn;

        board.black_bishop = 1 << 35;
        board.black |= board.black_bishop;
        board.black_queen = 1 << 42;
        board.black |= board.black_queen;
        board.black_pawn |= 1 << 22;
        board.black |= board.black_pawn;
        board.black_rook |= 1 << 30;
        board.black |= board.black_rook;
        board.black_king = 1 << 7;
        board.black |= board.black_king;


        print_binary_board(board.white | board.black);
        println!("In check: {}", in_check(&board));
    }
    #[test]
    fn in_check_01() {
        let mut board = create_test_board();
        board.white_king |= 1 << 35;
        board.white |= board.white_king;
        board.black_bishop |= 1 << 62;
        board.black_rook |= 1 << 53;
        board.black_queen |= 1 << 39;
        board.black |= board.black_bishop;
        board.black |= board.black_rook;
        board.black |= board.black_queen;

        print_binary_board(board.white | board.black);
        println!("In check: {}", in_check(&board));
    }
    #[test]
    fn black_pawn_moves() {
        let mut board = create_test_board();
        board.black_pawn |= (1 << 48) | (1 << 33) | (1 << 26) | (1 << 13);
        board.black |= board.black_pawn;

        board.white_pawn |= (1 << 41) | (1 << 24) | (1 << 18) | (1 << 4);
        board.white |= board.white_pawn;
        board.turn = false;
        board.ep_target = Some(19);

        let moves = pawn_moves(&board);
        for one_move in moves.iter() {
            println!("{:?}", one_move);
        }
        assert_eq!(14, moves.len());
    }

    #[test]
    fn white_pawn_moves() {
        let mut board = create_test_board();
        board.white_pawn |= (1 << 8) | (1 << 25) | (1 << 42) | (1 << 54) | (1 << 55);
        board.white |= board.white_pawn;
        board.black_pawn |= (1 << 32) | (1 << 34) | (1 << 50) | (1 << 62) | (1 << 63);
        board.black |= board.black_pawn;
        let moves = pawn_moves(&board);
        for one_move in moves.iter() {
            println!("{:?}", one_move);
        }
        assert_eq!(13, moves.len());
    }
    #[test]
    fn promo_right_capture() {
        let mut board = create_test_board();
        board.white_pawn = (1 << 49) | (1 << 51) | (1 << 53);
        board.white |= board.white_pawn;
        board.black_bishop = (1 << 58) | (1 << 60);
        board.black |= board.black_bishop;
        let moves = pawn_moves(&board);
        for one_move in moves.iter() {
            println!("{:?}", one_move);
        }
        assert_eq!(28, moves.len());
    }

    #[test]
    fn promo_left_capture() {
        let mut board = create_test_board();

        board.white_pawn = (1 << 49) | (1 << 51) | (1 << 55);
        board.white |= board.white_pawn;
        board.black_bishop = (1 << 56) | (1 << 58);
        board.black |= board.black_bishop;
        let moves = pawn_moves(&board);
        for one_move in moves.iter() {
            println!("{:?}", one_move);
        }
        assert_eq!(20, moves.len());
    }
    #[test]
    fn pawn_promo() {
        let mut board = create_test_board();
        board.white_pawn = SEVENTH_RANK;
        board.white |= board.white_pawn;
        board.black_pawn |= (1 << 56);
        board.black |= (1 << 56);
        let moves = pawn_moves(&board);
        for one_move in moves.iter() {
            println!("{:?}", one_move);
        }
        assert_eq!(28, moves.len());
    }
    #[test]
    fn pawn_no_promo() {
        let mut board = create_test_board();
        board.white_pawn = (1 << 10) | (1 << 17) | (1 << 28) | (1 << 36) | (1 << 38) | (1 << 42);
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
