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



