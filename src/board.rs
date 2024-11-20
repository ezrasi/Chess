//TODO: revisit whether or not Move needs a color field. if not, clean up all the code that relies on it.


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

//true = white and false = black. This enables the ! operator for opposite color.
#[derive(Debug)]
struct Move {
    piece: u8,
    from: u8,
    to: u8,
    color: bool,
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
fn knight_moves (board: &Board, position: u8, color: bool) ->Vec<Move>{

    let mut moves: Vec<Move> = Vec::new();

    if position > 63 {
        dbg!("knight_moves received invalid position");
        return moves
    }
    
    //north jumps
    if position < 48 {
        //noEa jumps
        if position % 8 < 7 {
            let dest = position + 17;
            let shifted_dest = 1u64 << dest;
            let piece = if color {WHITE_KNIGHT} else {BLACK_KNIGHT};
            let board_color = if color {&board.white} else {&board.black};

                if shifted_dest & board_color == 0 {
                    let new_move = Move {
                        piece,
                        from: position,
                        to: dest,
                        color,
                    };

                    moves.push(new_move);
                }
            
        }
        //noWe jumps. Bound check, make sure no piece of same color is on destination square. Add move if all good.
        if position % 8 > 0 {
            let dest = position + 15;
            let shifted_dest = 1u64 << dest;
            let piece = if color {WHITE_KNIGHT} else {BLACK_KNIGHT};
            let board_color = if color {&board.white} else {&board.black};

                if shifted_dest & board_color == 0 {
                    let new_move = Move {
                        piece,
                        from: position,
                        to: dest,
                        color,
                    };

                    moves.push(new_move);
                }
            }
        }
    
    moves
}


#[cfg(test)]


    #[test]
    fn noEa_knight_jump() {
        let board =  Board {
            mailbox: [0; 64],  // Optionally, initialize mailbox with empty values
            white: 0,
            black: 0x0000000000000000,
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

        let mut new_moves = knight_moves(&board, 1, true);
        // new_moves.extend(knight_moves(&board, 5, true));
        
        dbg!({new_moves});

        assert_eq!(true, true);
    }
