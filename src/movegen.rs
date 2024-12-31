use crate::bitboard::MAGIC_TABLES;
use crate::utils::*;

// The Move representation
#[derive(Debug)]
struct Move {
    piece: u8,
    from: u8,
    to: u8,
    kind: u8,
}

// Takes in a board  and a move and returns an updated board with the move made
fn make_move(before: &Board, ply: &Move) -> Board {
    let mut after: Board = before.clone();
    let from_mask = !(1 << ply.from);
    let to_mask = 1 << ply.to;

    // a white move
    if before.turn {
        after.white &= from_mask;
        after.white |= to_mask;
        match ply.kind {
            QUIET_MOVE => match ply.piece {
                WHITE_PAWN => {
                    after.white_pawn &= from_mask;
                    after.white_pawn |= to_mask;
                }
                WHITE_KNIGHT => {
                    after.white_knight &= from_mask;
                    after.white_knight |= to_mask;
                }
                WHITE_BISHOP => {
                    after.white_bishop &= from_mask;
                    after.white_bishop |= to_mask;
                }
                WHITE_ROOK => {
                    after.white_rook &= from_mask;
                    after.white_rook |= to_mask;
                    if ply.from == 0 {
                        after.white_queenside_castle = false;
                    } else if ply.from == 7 {
                        after.white_kingside_castle = false;
                    }
                }
                WHITE_QUEEN => {
                    after.white_queen &= from_mask;
                    after.white_queen |= to_mask;
                }
                WHITE_KING => {
                    after.white_king &= from_mask;
                    after.white_king |= to_mask;
                    after.white_kingside_castle = false;
                    after.white_queenside_castle = false;
                }
                _ => panic!("make_move white move has invalid piece code"),
            },

            DOUBLE_PAWN_PUSH => {
                after.white_pawn &= from_mask;
                after.white_pawn |= to_mask;

                after.ep_target = Some(ply.to - 8);
            }
            KINGSIDE_CASTLE => {
                after.white_king &= from_mask;
                after.white_king |= to_mask;
                after.white_rook &= !(1 << 7);
                after.white_rook |= 1 << 5;
                after.white &= !(1 << 7);
                after.white |= 1 << 5;
                after.white_kingside_castle = false;
                after.white_queenside_castle = false;
            }
            QUEENSIDE_CASTLE => {
                after.white_king &= from_mask;
                after.white_king |= to_mask;
                after.white_rook &= !(1 << 0);
                after.white_rook |= 1 << 3;
                after.white &= !(1 << 0);
                after.white |= 1 << 3;
                after.white_queenside_castle = false;
                after.white_kingside_castle = false;
            }
            CAPTURE => {
                match ply.piece {
                    WHITE_PAWN => {
                        after.white_pawn &= from_mask;
                        after.white_pawn |= to_mask;
                    }
                    WHITE_KNIGHT => {
                        after.white_knight &= from_mask;
                        after.white_knight |= to_mask;
                    }
                    WHITE_BISHOP => {
                        after.white_bishop &= from_mask;
                        after.white_bishop |= to_mask;
                    }
                    WHITE_ROOK => {
                        after.white_rook &= from_mask;
                        after.white_rook |= to_mask;
                        if ply.from == 0 {
                            after.white_queenside_castle = false;
                        } else if ply.from == 7 {
                            after.white_kingside_castle = false;
                        }
                    }
                    WHITE_QUEEN => {
                        after.white_queen &= from_mask;
                        after.white_queen |= to_mask;
                    }
                    WHITE_KING => {
                        after.white_king &= from_mask;
                        after.white_king |= to_mask;
                        after.white_kingside_castle = false;
                        after.white_queenside_castle = false;
                    }
                    _ => panic!("make_move white move has invalid piece code"),
                };
                if to_mask & after.black_pawn != 0 {
                    after.black_pawn &= !to_mask;
                } else if to_mask & after.black_knight != 0 {
                    after.black_knight &= !to_mask;
                } else if to_mask & after.black_bishop != 0 {
                    after.black_bishop &= !to_mask;
                } else if to_mask & after.black_rook != 0 {
                    if ply.to == 56 {
                        after.black_queenside_castle = false;
                    } else if ply.to == 63 {
                        after.black_kingside_castle = false;
                    }
                    after.black_rook &= !to_mask;
                } else if to_mask & after.black_queen != 0 {
                    after.black_queen &= !to_mask;
                };
                after.black &= !to_mask;
            }
            EN_PASSANT => {
                after.white_pawn &= from_mask;
                after.white_pawn |= to_mask;
                after.black_pawn &= !(1 << (ply.to - 8));
                after.black &= !(1 << (ply.to - 8));
            }
            KNIGHT_PROMO => {
                after.white_pawn &= from_mask;
                after.white_knight |= to_mask;
            }
            BISHOP_PROMO => {
                after.white_pawn &= from_mask;
                after.white_bishop |= to_mask;
            }
            ROOK_PROMO => {
                after.white_pawn &= from_mask;
                after.white_rook |= to_mask;
            }
            QUEEN_PROMO => {
                after.white_pawn &= from_mask;
                after.white_queen |= to_mask;
            }
            KNIGHT_PROMO_CAPTURE => {
                after.white_pawn &= from_mask;
                after.white_knight |= to_mask;
                if to_mask & after.black_pawn != 0 {
                    after.black_pawn &= !to_mask;
                } else if to_mask & after.black_knight != 0 {
                    after.black_knight &= !to_mask;
                } else if to_mask & after.black_bishop != 0 {
                    after.black_bishop &= !to_mask;
                } else if to_mask & after.black_rook != 0 {
                    if ply.to == 56 {
                        after.black_queenside_castle = false;
                    } else if ply.to == 63 {
                        after.black_kingside_castle = false;
                    }

                    after.black_rook &= !to_mask;
                } else if to_mask & after.black_queen != 0 {
                    after.black_queen &= !to_mask;
                };
                after.black &= !to_mask;
            }
            BISHOP_PROMO_CAPTURE => {
                after.white_pawn &= from_mask;
                after.white_bishop |= to_mask;
                if to_mask & after.black_pawn != 0 {
                    after.black_pawn &= !to_mask;
                } else if to_mask & after.black_knight != 0 {
                    after.black_knight &= !to_mask;
                } else if to_mask & after.black_bishop != 0 {
                    after.black_bishop &= !to_mask;
                } else if to_mask & after.black_rook != 0 {
                    if ply.to == 56 {
                        after.black_queenside_castle = false;
                    } else if ply.to == 63 {
                        after.black_kingside_castle = false;
                    }

                    after.black_rook &= !to_mask;
                } else if to_mask & after.black_queen != 0 {
                    after.black_queen &= !to_mask;
                };
                after.black &= !to_mask;
            }
            ROOK_PROMO_CAPTURE => {
                after.white_pawn &= from_mask;
                after.white_rook |= to_mask;
                if to_mask & after.black_pawn != 0 {
                    after.black_pawn &= !to_mask;
                } else if to_mask & after.black_knight != 0 {
                    after.black_knight &= !to_mask;
                } else if to_mask & after.black_bishop != 0 {
                    after.black_bishop &= !to_mask;
                } else if to_mask & after.black_rook != 0 {
                    if ply.to == 56 {
                        after.black_queenside_castle = false;
                    } else if ply.to == 63 {
                        after.black_kingside_castle = false;
                    }
                    after.black_rook &= !to_mask;
                } else if to_mask & after.black_queen != 0 {
                    after.black_queen &= !to_mask;
                };
                after.black &= !to_mask;
            }
            QUEEN_PROMO_CAPTURE => {
                after.white_pawn &= from_mask;
                after.white_queen |= to_mask;
                if to_mask & after.black_pawn != 0 {
                    after.black_pawn &= !to_mask;
                } else if to_mask & after.black_knight != 0 {
                    after.black_knight &= !to_mask;
                } else if to_mask & after.black_bishop != 0 {
                    after.black_bishop &= !to_mask;
                } else if to_mask & after.black_rook != 0 {
                    if ply.to == 56 {
                        after.black_queenside_castle = false;
                    } else if ply.to == 63 {
                        after.black_kingside_castle = false;
                    }
                    after.black_rook &= !to_mask;
                } else if to_mask & after.black_queen != 0 {
                    after.black_queen &= !to_mask;
                };
                after.black &= !to_mask;
            }
            _ => panic!("Move received in make_move has invalid Move.kind value"),
        }
    }
    // a black move
    else {
        after.black &= from_mask;
        after.black |= to_mask;
        match ply.kind {
            QUIET_MOVE => match ply.piece {
                BLACK_PAWN => {
                    after.black_pawn &= from_mask;
                    after.black_pawn |= to_mask;
                }
                BLACK_KNIGHT => {
                    after.black_knight &= from_mask;
                    after.black_knight |= to_mask;
                }
                BLACK_BISHOP => {
                    after.black_bishop &= from_mask;
                    after.black_bishop |= to_mask;
                }
                BLACK_ROOK => {
                    after.black_rook &= from_mask;
                    after.black_rook |= to_mask;
                    if ply.from == 56 {
                        after.black_queenside_castle = false;
                    } else if ply.from == 63 {
                        after.black_kingside_castle = false;
                    }
                }
                BLACK_QUEEN => {
                    after.black_queen &= from_mask;
                    after.black_queen |= to_mask;
                }
                BLACK_KING => {
                    after.black_king &= from_mask;
                    after.black_king |= to_mask;
                    after.black_kingside_castle = false;
                    after.black_queenside_castle = false;
                }
                _ => panic!("make_move black move has invalid piece code"),
            },

            DOUBLE_PAWN_PUSH => {
                after.black_pawn &= from_mask;
                after.black_pawn |= to_mask;

                after.ep_target = Some(ply.to + 8);
            }
            KINGSIDE_CASTLE => {
                after.black_king &= from_mask;
                after.black_king |= to_mask;
                after.black_rook &= !(1 << 63);
                after.black_rook |= 1 << 61;
                after.black &= !(1 << 63);
                after.black |= 1 << 61;
                after.black_kingside_castle = false;
                after.black_queenside_castle = false;
            }
            QUEENSIDE_CASTLE => {
                after.black_king &= from_mask;
                after.black_king |= to_mask;
                after.black_rook &= !(1 << 56);
                after.black_rook |= 1 << 59;
                after.black &= !(1 << 56);
                after.black |= 1 << 59;
                after.black_queenside_castle = false;
                after.black_kingside_castle = false;
            }
            CAPTURE => {
                match ply.piece {
                    BLACK_PAWN => {
                        after.black_pawn &= from_mask;
                        after.black_pawn |= to_mask;
                    }
                    BLACK_KNIGHT => {
                        after.black_knight &= from_mask;
                        after.black_knight |= to_mask;
                    }
                    BLACK_BISHOP => {
                        after.black_bishop &= from_mask;
                        after.black_bishop |= to_mask;
                    }
                    BLACK_ROOK => {
                        after.black_rook &= from_mask;
                        after.black_rook |= to_mask;
                        if ply.from == 56 {
                            after.black_queenside_castle = false;
                        } else if ply.from == 63 {
                            after.black_kingside_castle = false;
                        }
                    }
                    BLACK_QUEEN => {
                        after.black_queen &= from_mask;
                        after.black_queen |= to_mask;
                    }
                    BLACK_KING => {
                        after.black_king &= from_mask;
                        after.black_king |= to_mask;
                        after.black_kingside_castle = false;
                        after.black_queenside_castle = false;
                    }
                    _ => panic!("make_move black move has invalid piece code"),
                };
                if to_mask & after.white_pawn != 0 {
                    after.white_pawn &= !to_mask;
                } else if to_mask & after.white_knight != 0 {
                    after.white_knight &= !to_mask;
                } else if to_mask & after.white_bishop != 0 {
                    after.white_bishop &= !to_mask;
                } else if to_mask & after.white_rook != 0 {
                    if ply.to == 0 {
                        after.white_queenside_castle = false;
                    } else if ply.to == 7 {
                        after.white_kingside_castle = false;
                    }
                    after.white_rook &= !to_mask;
                } else if to_mask & after.white_queen != 0 {
                    after.white_queen &= !to_mask;
                };
                after.white &= !to_mask;
            }
            EN_PASSANT => {
                after.black_pawn &= from_mask;
                after.black_pawn |= to_mask;
                after.white_pawn &= !(1 << (ply.to + 8));
                after.white &= !(1 << (ply.to + 8));
            }
            KNIGHT_PROMO => {
                after.black_pawn &= from_mask;
                after.black_knight |= to_mask;
            }
            BISHOP_PROMO => {
                after.black_pawn &= from_mask;
                after.black_bishop |= to_mask;
            }
            ROOK_PROMO => {
                after.black_pawn &= from_mask;
                after.black_rook |= to_mask;
            }
            QUEEN_PROMO => {
                after.black_pawn &= from_mask;
                after.black_queen |= to_mask;
            }
            KNIGHT_PROMO_CAPTURE => {
                after.black_pawn &= from_mask;
                after.black_knight |= to_mask;
                if to_mask & after.white_pawn != 0 {
                    after.white_pawn &= !to_mask;
                } else if to_mask & after.white_knight != 0 {
                    after.white_knight &= !to_mask;
                } else if to_mask & after.white_bishop != 0 {
                    after.white_bishop &= !to_mask;
                } else if to_mask & after.white_rook != 0 {
                    if ply.to == 0 {
                        after.white_queenside_castle = false;
                    } else if ply.to == 7 {
                        after.white_kingside_castle = false;
                    }

                    after.white_rook &= !to_mask;
                } else if to_mask & after.white_queen != 0 {
                    after.white_queen &= !to_mask;
                };
                after.white &= !to_mask;
            }
            BISHOP_PROMO_CAPTURE => {
                after.black_pawn &= from_mask;
                after.black_bishop |= to_mask;
                if to_mask & after.white_pawn != 0 {
                    after.white_pawn &= !to_mask;
                } else if to_mask & after.white_knight != 0 {
                    after.white_knight &= !to_mask;
                } else if to_mask & after.white_bishop != 0 {
                    after.white_bishop &= !to_mask;
                } else if to_mask & after.white_rook != 0 {
                    if ply.to == 0 {
                        after.white_queenside_castle = false;
                    } else if ply.to == 7 {
                        after.white_kingside_castle = false;
                    }
                    after.white_rook &= !to_mask;
                } else if to_mask & after.white_queen != 0 {
                    after.white_queen &= !to_mask;
                };
                after.white &= !to_mask;
            }
            ROOK_PROMO_CAPTURE => {
                after.black_pawn &= from_mask;
                after.black_rook |= to_mask;
                if to_mask & after.white_pawn != 0 {
                    after.white_pawn &= !to_mask;
                } else if to_mask & after.white_knight != 0 {
                    after.white_knight &= !to_mask;
                } else if to_mask & after.white_bishop != 0 {
                    after.white_bishop &= !to_mask;
                } else if to_mask & after.white_rook != 0 {
                    if ply.to == 0 {
                        after.white_queenside_castle = false;
                    } else if ply.to == 7 {
                        after.white_kingside_castle = false;
                    }
                    after.white_rook &= !to_mask;
                } else if to_mask & after.white_queen != 0 {
                    after.white_queen &= !to_mask;
                };
                after.white &= !to_mask;
            }
            QUEEN_PROMO_CAPTURE => {
                after.black_pawn &= from_mask;
                after.black_queen |= to_mask;
                if to_mask & after.white_pawn != 0 {
                    after.white_pawn &= !to_mask;
                } else if to_mask & after.white_knight != 0 {
                    after.white_knight &= !to_mask;
                } else if to_mask & after.white_bishop != 0 {
                    after.white_bishop &= !to_mask;
                } else if to_mask & after.white_rook != 0 {
                    if ply.to == 0 {
                        after.white_queenside_castle = false;
                    } else if ply.to == 7 {
                        after.white_kingside_castle = false;
                    }
                    after.white_rook &= !to_mask;
                } else if to_mask & after.white_queen != 0 {
                    after.white_queen &= !to_mask;
                };
                after.white &= !to_mask;
            }
            _ => panic!("Move received in make_move has invalid Move.kind value"),
        }

        after.fullmove += 1;
    }

    if ply.kind != DOUBLE_PAWN_PUSH {
        after.ep_target = None;
    }

    after.halfmove += 1;
    after.turn = !before.turn;
    after
}

// Takes in a board state and returns a Vec of all legal moves
fn legal_moves(board: &Board) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    let rook_attacks = &MAGIC_TABLES.rook_attacks;
    let rook_magics = &MAGIC_TABLES.rook_magics;
    let bishop_attacks = &MAGIC_TABLES.bishop_attacks;
    let bishop_magics = &MAGIC_TABLES.bishop_magics;

    let (knights, bishops, rooks, queens, king) = if board.turn {
        (
            board.white_knight,
            board.white_bishop,
            board.white_rook, 
            board.white_queen,
            board.white_king,
        )
    } else {
        (
            board.black_knight,
            board.black_bishop,
            board.black_rook, 
            board.black_queen,
            board.black_king,
        )
    };
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

    moves
}

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
            let ply = Move {
                piece,
                from,
                to,
                kind,
            };
            let new_board = make_move(&board, &ply);
            if !in_check(&new_board, board.turn) {
                moves.push(ply);
            }
        }
    };
    let add_regular_move = |moves: &mut Vec<Move>, from: u8, to: u8, kind: u8| {
        let ply = Move {
            piece,
            from,
            to,
            kind,
        };
        let new_board = make_move(&board, &ply);
        if !in_check(&new_board, board.turn) {
            moves.push(ply);
        }
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

fn knight_moves(board: &Board, position: u8) -> Vec<Move> {
    debug_assert!(
        position <= 63,
        "knight_moves received invalid position: {}",
        position
    );
    //set piece and colors
    let (piece, color, other_color) = if board.turn {
        (WHITE_KNIGHT, board.white, board.black)
    } else {
        (BLACK_KNIGHT, board.black, board.white)
    };

    // All the destinations it could jump (ignoring leaving the king in check)
    let potentials = KNIGHT_MOVE_MASKS[position as usize] & !color;
    let quiets = potentials & !other_color;
    let captures = potentials & other_color;
    let quiet_pos = set_bit_positions(quiets);
    let capture_pos = set_bit_positions(captures);

    let mut moves: Vec<Move> = Vec::new();
    for dest in quiet_pos {
        let ply = Move {
            piece,
            from: position,
            to: dest,
            kind: QUIET_MOVE,
        };
        let new_board = make_move(&board, &ply);
        if !in_check(&new_board, board.turn) {
            moves.push(ply);
        }
    }
    for dest in capture_pos {
        let ply = Move {
            piece,
            from: position,
            to: dest,
            kind: CAPTURE,
        };
        let new_board = make_move(&board, &ply);
        if !in_check(&new_board, board.turn) {
            moves.push(ply);
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
    let (piece, color, other_color) = if board.turn {
        (WHITE_BISHOP, board.white, board.black)
    } else {
        (BLACK_BISHOP, board.black, board.white)
    };
    let mut moves: Vec<Move> = Vec::new();

    // TODO use bitboard and mask with board state
    let mut mask = BISHOP_MOVE_MASKS[position as usize];
    if position < 56 {
        mask &= !EIGHTH_RANK;
    }
    if position > 7 {
        mask &= !FIRST_RANK;
    }
    if position % 8 < 7 {
        mask &= !H_FILE;
    }
    if position % 8 > 0 {
        mask &= !A_FILE;
    }
    mask &= color | other_color;
    let index = mask.wrapping_mul(magic) >> (64 - BBITS[position as usize] as u64);
    let potentials = attacks[index as usize] & !color;

    let quiets = set_bit_positions(potentials & !other_color);
    let captures = set_bit_positions(potentials & other_color);

    for dest in quiets {
        let ply = Move {
            piece,
            from: position,
            to: dest,
            kind: QUIET_MOVE,
        };
        let new_board = make_move(&board, &ply);
        if !in_check(&new_board, board.turn) {
            moves.push(ply);
        }
    }
    for dest in captures {
        let ply = Move {
            piece,
            from: position,
            to: dest,
            kind: CAPTURE,
        };
        let new_board = make_move(&board, &ply);
        if !in_check(&new_board, board.turn) {
            moves.push(ply);
        }
    }

    moves
}

fn rook_moves(board: &Board, position: u8, attacks: &Vec<u64>, magic: u64) -> Vec<Move> {
    debug_assert!(
        position <= 63,
        "rook_moves received invalid position: {}",
        position
    );
    let (piece, color, other_color) = if board.turn {
        (WHITE_ROOK, board.white, board.black)
    } else {
        (BLACK_ROOK, board.black, board.white)
    };
    let mut moves: Vec<Move> = Vec::new();

    // TODO use bitboard and mask with board state
    let mut mask = ROOK_MOVE_MASKS[position as usize];
    if position < 56 {
        mask &= !EIGHTH_RANK;
    }
    if position > 7 {
        mask &= !FIRST_RANK;
    }
    if position % 8 < 7 {
        mask &= !H_FILE;
    }
    if position % 8 > 0 {
        mask &= !A_FILE;
    }
    mask &= color | other_color;
    let index = mask.wrapping_mul(magic) >> (64 - RBITS[position as usize] as u64);
    let potentials = attacks[index as usize] & !color;
   
    let quiets = set_bit_positions(potentials & !other_color);
    let captures = set_bit_positions(potentials & other_color);

    for dest in quiets {
        let ply = Move {
            piece,
            from: position,
            to: dest,
            kind: QUIET_MOVE,
        };
        let new_board = make_move(&board, &ply);
        if !in_check(&new_board, board.turn) {
            moves.push(ply);
        }
    }
    for dest in captures {
        let ply = Move {
            piece,
            from: position,
            to: dest,
            kind: CAPTURE,
        };
        let new_board = make_move(&board, &ply);
        if !in_check(&new_board, board.turn) {
            moves.push(ply);
        }
    }

    moves
}

fn queen_moves(board: &Board, position: u8) -> Vec<Move> {
    debug_assert!(
        position <= 63,
        "queen_moves received invalid position: {}",
        position
    );
    let bishop_attacks = &MAGIC_TABLES.bishop_attacks[position as usize];
    let bishop_magic = MAGIC_TABLES.bishop_magics[position as usize];
    let rook_attacks = &MAGIC_TABLES.rook_attacks[position as usize];
    let rook_magic = MAGIC_TABLES.rook_magics[position as usize];
    let mut moves = bishop_moves(board, position, bishop_attacks, bishop_magic);
    moves.extend(rook_moves(board, position, rook_attacks, rook_magic));
    moves
}

fn king_moves(board: &Board, position: u8) -> Vec<Move> {
    //set piece and colors
    let (piece, color, other_color) = if board.turn {
        (WHITE_KING, board.white, board.black)
    } else {
        (BLACK_KING, board.black, board.white)
    };

    // moves besides castling
    let potentials = KING_MOVE_MASKS[position as usize] & !color;
    let quiets = potentials & !other_color;
    let captures = potentials & other_color;
    let quiet_pos = set_bit_positions(quiets);
    let capture_pos = set_bit_positions(captures);

    let mut moves: Vec<Move> = Vec::new();
    for dest in quiet_pos {
        let ply = Move {
            piece,
            from: position,
            to: dest,
            kind: QUIET_MOVE,
        };
        let new_board = make_move(&board, &ply);
        if !in_check(&new_board, board.turn) {
            moves.push(ply);
        }
    }
    for dest in capture_pos {
        let ply = Move {
            piece,
            from: position,
            to: dest,
            kind: CAPTURE,
        };
        let new_board = make_move(&board, &ply);
        if !in_check(&new_board, board.turn) {
            moves.push(ply);
        }
    }

    // castling
    if board.turn {
        if position == 4 {
            // kingside castle
            if ((color | other_color) & (1 << 5 | 1 << 6)) == 0 && board.white_kingside_castle {
                // make sure king doesnt pass through check
                let check_test = Move {
                    piece,
                    from: position,
                    to: 5,
                    kind: QUIET_MOVE,
                };
                let check_test_board = make_move(&board, &check_test);
                if !in_check(&check_test_board, board.turn) {
                    let ply = Move {
                        piece,
                        from: position,
                        to: 6,
                        kind: KINGSIDE_CASTLE,
                    };
                    let new_board = make_move(&board, &ply);
                    if !in_check(&new_board, board.turn) {
                        moves.push(ply);
                    }
                }
            }
            // queenside castle
            if ((color | other_color) & (1 << 1 | 1 << 2 | 1 << 3)) == 0
                && board.white_queenside_castle
            {
                // make sure king doesnt pass through check
                let check_test = Move {
                    piece,
                    from: position,
                    to: 3,
                    kind: QUIET_MOVE,
                };
                let check_test_board = make_move(&board, &check_test);
                if !in_check(&check_test_board, board.turn) {
                    let ply = Move {
                        piece,
                        from: position,
                        to: 2,
                        kind: QUEENSIDE_CASTLE,
                    };
                    let new_board = make_move(&board, &ply);
                    if !in_check(&new_board, board.turn) {
                        moves.push(ply);
                    }
                }
            }
        }
    } else {
        if position == 60 {
            // kingside castle
            if ((color | other_color) & (1 << 61 | 1 << 62)) == 0 && board.black_kingside_castle {
                // make sure king doesnt pass through check
                let check_test = Move {
                    piece,
                    from: position,
                    to: 61,
                    kind: QUIET_MOVE,
                };
                let check_test_board = make_move(&board, &check_test);
                if !in_check(&check_test_board, board.turn) {
                    let ply = Move {
                        piece,
                        from: position,
                        to: 62,
                        kind: KINGSIDE_CASTLE,
                    };
                    let new_board = make_move(&board, &ply);
                    if !in_check(&new_board, board.turn) {
                        moves.push(ply);
                    }
                }
            }
            // queenside castle
            if ((color | other_color) & (1 << 57 | 1 << 58 | 1 << 59)) == 0
                && board.black_queenside_castle
            {
                // make sure king doesnt pass through check
                let check_test = Move {
                    piece,
                    from: position,
                    to: 59,
                    kind: QUIET_MOVE,
                };
                let check_test_board = make_move(&board, &check_test);
                if !in_check(&check_test_board, board.turn) {
                    let ply = Move {
                        piece,
                        from: position,
                        to: 58,
                        kind: QUEENSIDE_CASTLE,
                    };
                    let new_board = make_move(&board, &ply);
                    if !in_check(&new_board, board.turn) {
                        moves.push(ply);
                    }
                }
            }
        }
    }
    moves
}
// Checks if the king of the current player is in check
fn in_check(board: &Board, color: bool) -> bool {
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
    ) = if color {
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
            if color {
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

    let straight_encounters =
        MAGIC_TABLES.rook_attacks[king_pos_usize][index as usize] & other_color;

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
fn starting_position() -> Board {
    Board {
        white: FIRST_RANK | SECOND_RANK,
        black: SEVENTH_RANK | EIGHTH_RANK,
        white_pawn: SECOND_RANK,
        white_knight: 1 << 1 | 1 << 6,
        white_bishop: 1 << 2 | 1 << 5,
        white_rook: 1 << 0 | 1 << 7,
        white_queen: 1 << 3,
        white_king: 1 << 4,
        black_pawn: SEVENTH_RANK,
        black_knight: 1 << 57 | 1 << 62,
        black_bishop: 1 << 58 | 1 << 61,
        black_rook: 1 << 56 | 1 << 63,
        black_queen: 1 << 59,
        black_king: 1 << 60,
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

fn perft(board: &Board, depth: u8) -> u64 {
    if depth == 0 {
        return 1
    };
    let mut count = 0;

    let moves = legal_moves(board);
    for ply in moves.iter() {
        let new_board = make_move(board, &ply);    
        count += perft(&new_board, depth - 1);
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;
    use std::time::Instant;

    #[test]
    fn test_perft() {
     let board = starting_position();
    let perft = perft(&board, 5);
        println!("Perft at depth 5: {}", perft);
    }

    #[test]
    fn test_legal_moves() {

        let board = starting_position();
        let moves = legal_moves(&board);
        for ply in moves.iter() {
            println!("");
            println!("{:?}", ply);
            println!("");


        }
        assert_eq!(moves.len(), 20);


    }
    #[test]
    fn legal_king_moves() {
        let mut board = create_test_board();
        board.white_king = 1 << 4;
        board.white_rook = 1 << 0 | 1 << 7;
        //board.white_knight = 1 << 2;
        board.white |= board.white_king | board.white_rook | board.white_knight;
        board.black_queen = 1 << 61 | 1 << 39;
        board.black |= board.black_queen;
        let kingmoves = king_moves(&board, 4);
        println!("");
        println!("Queen moves: {:?}", kingmoves);
        println!("");
        assert_eq!(kingmoves.len(), 1);
    }
    #[test]
    fn legal_queen_moves() {
        let mut board = create_test_board();
        board.white_queen = 1 << 28;
        board.white_king = 1 << 12;
        board.white |= board.white_queen | board.white_king;

        board.black_pawn = 1 << 42;
        board.black |= board.black_pawn;

        let queenmoves = queen_moves(&board, 28);
        let mut bits: u64 = 0;
        for ply in queenmoves.iter() {
            bits |= 1 << ply.to;
        }
        println!("");
        println!("");
        print_binary_board(bits);
        println!("");
        println!("Queen moves: {:?}", queenmoves);
        println!("");
        assert_eq!(queenmoves.len(), 23);
    }
    #[test]
    fn legal_bishop_moves() {
        let mut board = create_test_board();
        board.white_bishop = 1 << 27;
        board.white_king = 1 << 6;
        board.white |= board.white_knight | board.white_king;
        board.black_bishop = 1 << 48;
        board.black |= board.black_bishop;

        let attacks = &MAGIC_TABLES.bishop_attacks[27];
        let magic = MAGIC_TABLES.bishop_magics[27];
        let now = Instant::now();

        let bishopmoves = bishop_moves(&board, 27, attacks, magic);
        println!("");
        println!("Bishop moves: {:?}", bishopmoves);
        println!("");
        assert_eq!(bishopmoves.len(), 5);
        println!("");
        println!("");
        println!("Elapsed time: {:?}", now.elapsed());
    }
    #[test]
    fn legal_knight_moves() {
        let mut board = create_test_board();
        board.white_knight = 1 << 27;
        board.white_king = 1 << 13;
        board.white |= board.white_knight | board.white_king;
        board.black_bishop = 1 << 48;
        board.black |= board.black_bishop;
        let knightmoves = knight_moves(&board, 27);
        println!("");
        println!("Knight moves: {:?}", knightmoves);
        println!("");
        assert_eq!(knightmoves.len(), 0);

        let now = Instant::now();

        let knightmoves = knight_moves(&board, 27);
        println!("Knight moves: {:?}", knightmoves);
        println!("");
        println!("");
        println!("Elapsed time: {:?}", now.elapsed());
        println!("");
        println!("");
    }

    #[test]
    fn legal_pawn_moves() {
        let mut board = create_test_board();
        // 12 pawn can't move foward bc pinned
        board.white_pawn = (1 << 9) | (1 << 52) | (1 << 12);
        board.white_king = 1 << 5;
        board.white |= board.white_pawn | board.white_king;
        board.black_queen = 1 << 19;
        board.black |= board.black_queen;

        let pawnmoves = pawn_moves(&board);
        println!("");
        println!("Pawn moves: {:?}", pawnmoves);
        println!("");
        assert_eq!(pawnmoves.len(), 7);

        // no en passant bc would be in check
        board.white_pawn = 1 << 36;
        board.white_king = 1 << 39;
        board.white = board.white_pawn | board.white_king;
        board.black_pawn = 1 << 35;
        board.black_rook = 1 << 32;
        board.black = board.black_pawn | board.black_rook;
        board.ep_target = Some(43);

        let pawnmoves = pawn_moves(&board);
        println!("");
        println!("Pawn moves: {:?}", pawnmoves);
        println!("");
        assert_eq!(pawnmoves.len(), 1);
    }

    #[test]
    fn make_black_move() {
        let mut board = create_test_board();
        board.turn = false;
        board.halfmove = 1;

        //quiet rook move
        board.black_rook |= 1 << 20;
        board.black |= board.black_rook;
        let ply = Move {
            piece: BLACK_ROOK,
            from: 20,
            to: 60,
            kind: QUIET_MOVE,
        };
        let after = make_move(&board, &ply);
        assert_eq!(after.ep_target, None);
        assert_eq!(after.black_rook, 1 << 60);
        assert_eq!(after.black, 1 << 60);

        // quiet rook move preventing castling
        board.black_rook = 1 << 63;
        board.black = board.black_rook;
        let ply = Move {
            piece: BLACK_ROOK,
            from: 63,
            to: 47,
            kind: QUIET_MOVE,
        };
        let after = make_move(&board, &ply);
        assert_eq!(after.black_kingside_castle, false);
        assert_eq!(after.black_queenside_castle, true);
        assert_eq!(after.black_rook, 1 << 47);
        assert_eq!(after.black, 1 << 47);

        // double pawn push
        board.black_pawn = 1 << 52;
        board.black = board.black_pawn;
        let ply = Move {
            piece: BLACK_PAWN,
            from: 52,
            to: 36,
            kind: DOUBLE_PAWN_PUSH,
        };
        let after = make_move(&board, &ply);
        assert_eq!(after.ep_target, Some(44));
        assert_eq!(after.black_pawn, 1 << 36);
        assert_eq!(after.black, 1 << 36);

        // kingside castle
        board.black_king = 1 << 60;
        board.black_rook = 1 << 63;
        board.black = board.black_king | board.black_rook;
        let ply = Move {
            piece: BLACK_KING,
            from: 60,
            to: 62,
            kind: KINGSIDE_CASTLE,
        };
        let after = make_move(&board, &ply);
        assert_eq!(after.black_kingside_castle, false);
        assert_eq!(after.black_queenside_castle, false);
        assert_eq!(after.black_king, 1 << 62);
        assert_eq!(after.black_rook, 1 << 61);
        assert_eq!(after.black, 1 << 61 | 1 << 62);

        // queenside castle
        board.black_king = 1 << 60;
        board.black_rook = 1 << 56;
        board.black = board.black_king | board.black_rook;
        let ply = Move {
            piece: BLACK_KING,
            from: 60,
            to: 58,
            kind: QUEENSIDE_CASTLE,
        };
        let after = make_move(&board, &ply);
        assert_eq!(after.black_kingside_castle, false);
        assert_eq!(after.black_queenside_castle, false);
        assert_eq!(after.black_king, 1 << 58);
        assert_eq!(after.black_rook, 1 << 59);
        assert_eq!(after.black, 1 << 58 | 1 << 59);

        // capture
        board.black_bishop = 1 << 45;
        board.black = board.black_bishop;
        board.white_rook = 1 << 0;
        board.white = board.white_rook;
        let ply = Move {
            piece: BLACK_BISHOP,
            from: 45,
            to: 0,
            kind: CAPTURE,
        };
        let after = make_move(&board, &ply);
        assert_eq!(after.black_bishop, 1 << 0);
        assert_eq!(after.black, 1 << 0);
        assert_eq!(after.white, 0);
        assert_eq!(after.white_rook, 0);
        assert_eq!(after.white_queenside_castle, false);
        assert_eq!(after.white_kingside_castle, true);

        // rook capture ruining castling
        board.black_rook = 1 << 63;
        board.black = board.black_rook;
        board.white_rook = 1 << 47;
        board.white = board.white_rook;
        let ply = Move {
            piece: BLACK_ROOK,
            from: 63,
            to: 47,
            kind: CAPTURE,
        };
        let after = make_move(&board, &ply);
        assert_eq!(after.black_rook, 1 << 47);
        assert_eq!(after.black, 1 << 47);
        assert_eq!(after.white, 0);
        assert_eq!(after.white_rook, 0);
        assert_eq!(after.black_kingside_castle, false);
        assert_eq!(after.black_queenside_castle, true);

        // en passant
        board.black_pawn = 1 << 24;
        board.black = board.black_pawn;
        board.white_pawn = 1 << 25;
        board.white = board.white_pawn;
        board.ep_target = Some(17);
        let ply = Move {
            piece: BLACK_PAWN,
            from: 24,
            to: 17,
            kind: EN_PASSANT,
        };
        let after = make_move(&board, &ply);
        assert_eq!(after.ep_target, None);
        assert_eq!(after.black_pawn, 1 << 17);
        assert_eq!(after.black, 1 << 17);
        assert_eq!(after.white_pawn, 0);
        assert_eq!(after.white, 0);

        // promotion
        board.black_pawn = 1 << 13;
        board.black = board.black_pawn;
        let ply = Move {
            piece: BLACK_PAWN,
            from: 13,
            to: 5,
            kind: BISHOP_PROMO,
        };
        let after = make_move(&board, &ply);
        assert_eq!(after.black_pawn, 0);
        assert_eq!(after.black_bishop, 1 << 5);
        assert_eq!(after.black, 1 << 5);

        // promo capture
        board.black_pawn = 1 << 9;
        board.black = board.black_pawn;
        board.white_queen = 1 << 2;
        board.white = board.white_queen;
        let ply = Move {
            piece: BLACK_PAWN,
            from: 9,
            to: 2,
            kind: QUEEN_PROMO_CAPTURE,
        };
        let after = make_move(&board, &ply);
        assert_eq!(after.black_pawn, 0);
        assert_eq!(after.black_queen, 1 << 2);
        assert_eq!(after.black, 1 << 2);
        assert_eq!(after.white_queen, 0);
        assert_eq!(after.white, 0);
    }

    #[test]
    fn make_white_move() {
        let mut board = create_test_board();

        //quiet rook move
        board.white_rook |= 1 << 20;
        board.white |= board.white_rook;
        let ply = Move {
            piece: WHITE_ROOK,
            from: 20,
            to: 60,
            kind: QUIET_MOVE,
        };
        let after = make_move(&board, &ply);
        assert_eq!(after.ep_target, None);
        assert_eq!(after.white_rook, 1 << 60);
        assert_eq!(after.white, 1 << 60);

        // quiet rook move preventing castling
        board.white_rook = 1 << 7;
        board.white = board.white_rook;
        let ply = Move {
            piece: WHITE_ROOK,
            from: 7,
            to: 47,
            kind: QUIET_MOVE,
        };
        let after = make_move(&board, &ply);
        assert_eq!(after.white_kingside_castle, false);
        assert_eq!(after.white_queenside_castle, true);
        assert_eq!(after.white_rook, 1 << 47);
        assert_eq!(after.white, 1 << 47);

        // double pawn push
        board.white_pawn = 1 << 10;
        board.white = board.white_pawn;
        let ply = Move {
            piece: WHITE_PAWN,
            from: 10,
            to: 26,
            kind: DOUBLE_PAWN_PUSH,
        };
        let after = make_move(&board, &ply);
        assert_eq!(after.ep_target, Some(18));
        assert_eq!(after.white_pawn, 1 << 26);
        assert_eq!(after.white, 1 << 26);

        // kingside castle
        board.white_king = 1 << 4;
        board.white_rook = 1 << 7;
        board.white = board.white_king | board.white_rook;
        let ply = Move {
            piece: WHITE_KING,
            from: 4,
            to: 6,
            kind: KINGSIDE_CASTLE,
        };
        let after = make_move(&board, &ply);
        assert_eq!(after.white_kingside_castle, false);
        assert_eq!(after.white_queenside_castle, false);
        assert_eq!(after.white_king, 1 << 6);
        assert_eq!(after.white_rook, 1 << 5);
        assert_eq!(after.white, 1 << 5 | 1 << 6);

        // queenside castle
        board.white_king = 1 << 4;
        board.white_rook = 1 << 0;
        board.white = board.white_king | board.white_rook;
        let ply = Move {
            piece: WHITE_KING,
            from: 4,
            to: 2,
            kind: QUEENSIDE_CASTLE,
        };
        let after = make_move(&board, &ply);
        assert_eq!(after.white_kingside_castle, false);
        assert_eq!(after.white_queenside_castle, false);
        assert_eq!(after.white_king, 1 << 2);
        assert_eq!(after.white_rook, 1 << 3);
        assert_eq!(after.white, 1 << 2 | 1 << 3);

        // capture
        board.white_bishop = 1 << 28;
        board.white = board.white_bishop;
        board.black_rook = 1 << 42;
        board.black = board.black_rook;
        let ply = Move {
            piece: WHITE_BISHOP,
            from: 28,
            to: 42,
            kind: CAPTURE,
        };
        let after = make_move(&board, &ply);
        assert_eq!(after.white_bishop, 1 << 42);
        assert_eq!(after.white, 1 << 42);
        assert_eq!(after.black, 0);
        assert_eq!(after.black_rook, 0);

        // rook capture ruining castling
        board.white_rook = 1 << 7;
        board.white = board.white_rook;
        board.black_rook = 1 << 47;
        board.black = board.black_rook;
        let ply = Move {
            piece: WHITE_ROOK,
            from: 7,
            to: 47,
            kind: CAPTURE,
        };
        let after = make_move(&board, &ply);
        assert_eq!(after.white_rook, 1 << 47);
        assert_eq!(after.white, 1 << 47);
        assert_eq!(after.black, 0);
        assert_eq!(after.black_rook, 0);
        assert_eq!(after.white_kingside_castle, false);
        assert_eq!(after.white_queenside_castle, true);

        // en passant
        board.white_pawn = 1 << 36;
        board.white = board.white_pawn;
        board.black_pawn = 1 << 35;
        board.black = board.black_pawn;
        board.ep_target = Some(43);
        let ply = Move {
            piece: WHITE_PAWN,
            from: 36,
            to: 43,
            kind: EN_PASSANT,
        };
        let after = make_move(&board, &ply);
        assert_eq!(after.ep_target, None);
        assert_eq!(after.white_pawn, 1 << 43);
        assert_eq!(after.white, 1 << 43);
        assert_eq!(after.black_pawn, 0);
        assert_eq!(after.black, 0);

        // promotion
        board.white_pawn = 1 << 52;
        board.white = board.white_pawn;
        let ply = Move {
            piece: WHITE_PAWN,
            from: 52,
            to: 60,
            kind: QUEEN_PROMO,
        };
        let after = make_move(&board, &ply);
        assert_eq!(after.white_pawn, 0);
        assert_eq!(after.white_queen, 1 << 60);
        assert_eq!(after.white, 1 << 60);

        // promo capture
        board.white_pawn = 1 << 52;
        board.white = board.white_pawn;
        board.black_queen = 1 << 59;
        board.black = board.black_queen;
        let ply = Move {
            piece: WHITE_PAWN,
            from: 52,
            to: 59,
            kind: KNIGHT_PROMO_CAPTURE,
        };
        let after = make_move(&board, &ply);
        assert_eq!(after.white_pawn, 0);
        assert_eq!(after.white_knight, 1 << 59);
        assert_eq!(after.white, 1 << 59);
        assert_eq!(after.black_queen, 0);
        assert_eq!(after.black, 0);
    }
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
        println!("In check: {}", in_check(&board, board.turn));
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
        println!("In check: {}", in_check(&board, board.turn));
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
