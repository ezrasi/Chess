
// This function returns a vec of all the on bit positions. e.g. 9 -> [0, 3]
// It takes in a u64 and outputs a Vec of the indices of the on bits
pub fn set_bit_positions(mut number: u64) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    while number != 0 {
        result.push(number.trailing_zeros() as u8);
        number &= number - 1; // Clear the least significant set bit
    }
    result
}

pub fn print_binary_board(value: u64) {
    let binary_string = format!("{:064b}", value); // Convert to a 64-bit binary string
    let reversed_binary_string: String = binary_string.chars().rev().collect(); // Reverse the entire string

    // Print chunks in reverse order directly
    reversed_binary_string
        .as_bytes()
        .chunks(8)
        .rev()
        .for_each(|chunk| {
            println!("{}", std::str::from_utf8(chunk).unwrap());
        });
}

fn blank_board() -> Board {
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

pub fn starting_position() -> Board {
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
        fullmove: 1,
    }
}

// expects valid fen
pub fn fen_to_board(fen: &str) -> Board {
    let mut board = blank_board();

    let parts: Vec<&str> = fen.split(' ').collect();

    // piece placement
    let ranks = parts[0].split('/').rev();
    let mut file: u8;
    let mut rank_no: u8 = 0;

    for rank in ranks {
        file = 0;
        for square in rank.chars() {
            match square {
                '1'..='8' => file += square.to_digit(10).unwrap() as u8,
                'p' => {
                    board.black |= 1 << (8 * rank_no + file);
                    board.black_pawn |= 1 << (8 * rank_no + file);
                    file += 1;
                }
                'n' => {
                    board.black |= 1 << (8 * rank_no + file);
                    board.black_knight |= 1 << (8 * rank_no + file);
                    file += 1;
                }
                'b' => {
                    board.black |= 1 << (8 * rank_no + file);
                    board.black_bishop |= 1 << (8 * rank_no + file);
                    file += 1;
                }
                'r' => {
                    board.black |= 1 << (8 * rank_no + file);
                    board.black_rook |= 1 << (8 * rank_no + file);
                    file += 1;
                }
                'q' => {
                    board.black |= 1 << (8 * rank_no + file);
                    board.black_queen |= 1 << (8 * rank_no + file);
                    file += 1;
                }
                'k' => {
                    board.black |= 1 << (8 * rank_no + file);
                    board.black_king |= 1 << (8 * rank_no + file);
                    file += 1;
                }
                'P' => {
                    board.white |= 1 << (8 * rank_no + file);
                    board.white_pawn |= 1 << (8 * rank_no + file);
                    file += 1;
                }

                'N' => {
                    board.white |= 1 << (8 * rank_no + file);
                    board.white_knight |= 1 << (8 * rank_no + file);
                    file += 1;
                }
                'B' => {
                    board.white |= 1 << (8 * rank_no + file);
                    board.white_bishop |= 1 << (8 * rank_no + file);
                    file += 1;
                }
                'R' => {
                    board.white |= 1 << (8 * rank_no + file);
                    board.white_rook |= 1 << (8 * rank_no + file);
                    file += 1;
                }
                'Q' => {
                    board.white |= 1 << (8 * rank_no + file);
                    board.white_queen |= 1 << (8 * rank_no + file);
                    file += 1;
                }
                'K' => {
                    board.white |= 1 << (8 * rank_no + file);
                    board.white_king |= 1 << (8 * rank_no + file);
                    file += 1;
                }

                _ => panic!("Invalid fen"),
            }
        }

        rank_no += 1;
    }

    // side to move
    let side = parts[1];
    board.turn = side == "w";

    // castling rights
    board.white_kingside_castle = false;
    board.white_queenside_castle = false;
    board.black_kingside_castle = false;
    board.black_queenside_castle = false;

    let rights = parts[2];
    for right in rights.chars() {
        match right {
            '-' => {}
            'K' => {
                board.white_kingside_castle = true;
            }
            'Q' => {
                board.white_queenside_castle = true;
            }
            'k' => {
                board.black_kingside_castle = true;
            }
            'q' => {
                board.black_queenside_castle = true;
            }
            _ => panic!("Invalid fen"),
        }
    }

    // ep target
    let target = parts[3];

    if target != "-" {
        let file = target.chars().next().unwrap() as u8 - 'a' as u8;
        let rank = target.chars().next().unwrap().to_digit(10).unwrap();
        board.ep_target = Some((8 * rank) as u8 + file);
    }

    board.halfmove = parts[4].chars().next().unwrap() as u16;
    board.fullmove = parts[5].chars().next().unwrap() as u16;

    board
}

// The Board representation.
#[derive(Clone, Debug)]
pub struct Board {
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
    //en-passant
    pub ep_target: Option<u8>,
    pub halfmove: u16,
    pub fullmove: u16,
}

// The Move representation
#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct Move {
    pub piece: u8,
    pub from: u8,
    pub to: u8,
    pub kind: u8,
}

// a-file             0x0101010101010101
// h-file             0x8080808080808080
// 1st rank           0x00000000000000FF
// 8th rank           0xFF00000000000000
// a1-h8 diagonal     0x8040201008040201
// h1-a8 antidiagonal 0x0102040810204080
// light squares      0x55AA55AA55AA55AA
// dark squares       0xAA55AA55AA55AA55

// Number of on bits in the blocker mask per square (excludes current square and edges)
pub const RBITS: [u8; 64] = [
    12, 11, 11, 11, 11, 11, 11, 12, 11, 10, 10, 10, 10, 10, 10, 11, 11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11, 11, 10, 10, 10, 10, 10, 10, 11, 11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11, 12, 11, 11, 11, 11, 11, 11, 12,
];

pub const BBITS: [u8; 64] = [
    6, 5, 5, 5, 5, 5, 5, 6, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 7, 7, 7, 7, 5, 5, 5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5, 5, 5, 7, 7, 7, 7, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 6, 5, 5, 5, 5, 5, 5, 6,
];

pub const SQUARES: [&str; 64] = [
    "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1", // 0-7
    "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2", // 8-15
    "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3", // 16-23
    "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4", // 24-31
    "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5", // 32-39
    "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6", // 40-47
    "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7", // 48-55
    "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8", // 56-63
];

//piece codes
pub const WHITE_PAWN: u8 = 0b00000101;
pub const WHITE_KNIGHT: u8 = 0b00001001;
pub const WHITE_BISHOP: u8 = 0b00010001;
pub const WHITE_ROOK: u8 = 0b00100001;
pub const WHITE_QUEEN: u8 = 0b01000001;
pub const WHITE_KING: u8 = 0b10000001;
pub const BLACK_PAWN: u8 = 0b00000110;
pub const BLACK_KNIGHT: u8 = 0b00001010;
pub const BLACK_BISHOP: u8 = 0b00010010;
pub const BLACK_ROOK: u8 = 0b00100010;
pub const BLACK_QUEEN: u8 = 0b01000010;
pub const BLACK_KING: u8 = 0b10000010;

//special move codes
pub const QUIET_MOVE: u8 = 0b0000;
pub const DOUBLE_PAWN_PUSH: u8 = 0b0001;
pub const KINGSIDE_CASTLE: u8 = 0b0010;
pub const QUEENSIDE_CASTLE: u8 = 0b0011;
pub const CAPTURE: u8 = 0b0100;
pub const EN_PASSANT: u8 = 0b0101;
pub const KNIGHT_PROMO: u8 = 0b1000;
pub const BISHOP_PROMO: u8 = 0b1001;
pub const ROOK_PROMO: u8 = 0b1010;
pub const QUEEN_PROMO: u8 = 0b1011;
pub const KNIGHT_PROMO_CAPTURE: u8 = 0b1100;
pub const BISHOP_PROMO_CAPTURE: u8 = 0b1101;
pub const ROOK_PROMO_CAPTURE: u8 = 0b1110;
pub const QUEEN_PROMO_CAPTURE: u8 = 0b1111;

//define the edge
pub const EDGE_MASK: u64 = 18411139144890810879;

//rank and file masks
//file masks
pub const A_FILE: u64 = 0x0101010101010101;
pub const B_FILE: u64 = 0x0202020202020202;
pub const C_FILE: u64 = 0x0404040404040404;
pub const D_FILE: u64 = 0x0808080808080808;
pub const E_FILE: u64 = 0x1010101010101010;
pub const F_FILE: u64 = 0x2020202020202020;
pub const G_FILE: u64 = 0x4040404040404040;
pub const H_FILE: u64 = 0x8080808080808080;

//rank masks
pub const FIRST_RANK: u64 = 0x00000000000000FF;
pub const SECOND_RANK: u64 = 0x000000000000FF00;
pub const THIRD_RANK: u64 = 0x0000000000FF0000;
pub const FOURTH_RANK: u64 = 0x00000000FF000000;
pub const FIFTH_RANK: u64 = 0x000000FF00000000;
pub const SIXTH_RANK: u64 = 0x0000FF0000000000;
pub const SEVENTH_RANK: u64 = 0x00FF000000000000;
pub const EIGHTH_RANK: u64 = 0xFF00000000000000;

//piece move masks
pub const KNIGHT_MOVE_MASKS: [u64; 64] = [
    132096,
    329728,
    659712,
    1319424,
    2638848,
    5277696,
    10489856,
    4202496,
    33816580,
    84410376,
    168886289,
    337772578,
    675545156,
    1351090312,
    2685403152,
    1075839008,
    8657044482,
    21609056261,
    43234889994,
    86469779988,
    172939559976,
    345879119952,
    687463207072,
    275414786112,
    2216203387392,
    5531918402816,
    11068131838464,
    22136263676928,
    44272527353856,
    88545054707712,
    175990581010432,
    70506185244672,
    567348067172352,
    1416171111120896,
    2833441750646784,
    5666883501293568,
    11333767002587136,
    22667534005174272,
    45053588738670592,
    18049583422636032,
    145241105196122112,
    362539804446949376,
    725361088165576704,
    1450722176331153408,
    2901444352662306816,
    5802888705324613632,
    11533718717099671552,
    4620693356194824192,
    288234782788157440,
    576469569871282176,
    1224997833292120064,
    2449995666584240128,
    4899991333168480256,
    9799982666336960512,
    1152939783987658752,
    2305878468463689728,
    1128098930098176,
    2257297371824128,
    4796069720358912,
    9592139440717824,
    19184278881435648,
    38368557762871296,
    4679521487814656,
    9077567998918656,
];
pub const BISHOP_MOVE_MASKS: [u64; 64] = [
    9241421688590303744,
    36099303471056128,
    141012904249856,
    550848566272,
    6480472064,
    1108177604608,
    283691315142656,
    72624976668147712,
    4620710844295151618,
    9241421688590368773,
    36099303487963146,
    141017232965652,
    1659000848424,
    283693466779728,
    72624976676520096,
    145249953336262720,
    2310355422147510788,
    4620710844311799048,
    9241421692918565393,
    36100411639206946,
    424704217196612,
    72625527495610504,
    145249955479592976,
    290499906664153120,
    1155177711057110024,
    2310355426409252880,
    4620711952330133792,
    9241705379636978241,
    108724279602332802,
    145390965166737412,
    290500455356698632,
    580999811184992272,
    577588851267340304,
    1155178802063085600,
    2310639079102947392,
    4693335752243822976,
    9386671504487645697,
    326598935265674242,
    581140276476643332,
    1161999073681608712,
    288793334762704928,
    577868148797087808,
    1227793891648880768,
    2455587783297826816,
    4911175566595588352,
    9822351133174399489,
    1197958188344280066,
    2323857683139004420,
    144117404414255168,
    360293502378066048,
    720587009051099136,
    1441174018118909952,
    2882348036221108224,
    5764696068147249408,
    11529391036782871041,
    4611756524879479810,
    567382630219904,
    1416240237150208,
    2833579985862656,
    5667164249915392,
    11334324221640704,
    22667548931719168,
    45053622886727936,
    18049651735527937,
];
pub const ROOK_MOVE_MASKS: [u64; 64] = [
    72340172838076926,
    144680345676153597,
    289360691352306939,
    578721382704613623,
    1157442765409226991,
    2314885530818453727,
    4629771061636907199,
    9259542123273814143,
    72340172838141441,
    144680345676217602,
    289360691352369924,
    578721382704674568,
    1157442765409283856,
    2314885530818502432,
    4629771061636939584,
    9259542123273813888,
    72340172854657281,
    144680345692602882,
    289360691368494084,
    578721382720276488,
    1157442765423841296,
    2314885530830970912,
    4629771061645230144,
    9259542123273748608,
    72340177082712321,
    144680349887234562,
    289360695496279044,
    578721386714368008,
    1157442769150545936,
    2314885534022901792,
    4629771063767613504,
    9259542123257036928,
    72341259464802561,
    144681423712944642,
    289361752209228804,
    578722409201797128,
    1157443723186933776,
    2314886351157207072,
    4629771607097753664,
    9259542118978846848,
    72618349279904001,
    144956323094725122,
    289632270724367364,
    578984165983651848,
    1157687956502220816,
    2315095537539358752,
    4629910699613634624,
    9259541023762186368,
    143553341945872641,
    215330564830528002,
    358885010599838724,
    645993902138460168,
    1220211685215703056,
    2368647251370188832,
    4665518383679160384,
    9259260648297103488,
    18302911464433844481,
    18231136449196065282,
    18087586418720506884,
    17800486357769390088,
    17226286235867156496,
    16077885992062689312,
    13781085504453754944,
    9187484529235886208,
];
pub const QUEEN_MOVE_MASKS: [u64; 64] = [
    9313761861428380670,
    180779649147209725,
    289501704256556795,
    578721933553179895,
    1157442771889699055,
    2314886638996058335,
    4630054752952049855,
    9332167099941961855,
    4693051017133293059,
    9386102034266586375,
    325459994840333070,
    578862399937640220,
    1157444424410132280,
    2315169224285282160,
    4702396038313459680,
    9404792076610076608,
    2382695595002168069,
    4765391190004401930,
    9530782384287059477,
    614821794359483434,
    1157867469641037908,
    2387511058326581416,
    4775021017124823120,
    9550042029937901728,
    1227517888139822345,
    2455035776296487442,
    4910072647826412836,
    9820426766351346249,
    1266167048752878738,
    2460276499189639204,
    4920271519124312136,
    9840541934442029200,
    649930110732142865,
    1299860225776030242,
    2600000831312176196,
    5272058161445620104,
    10544115227674579473,
    2641485286422881314,
    5210911883574396996,
    10421541192660455560,
    361411684042608929,
    722824471891812930,
    1517426162373248132,
    3034571949281478664,
    6068863523097809168,
    12137446670713758241,
    5827868887957914690,
    11583398706901190788,
    287670746360127809,
    575624067208594050,
    1079472019650937860,
    2087167920257370120,
    4102559721436811280,
    8133343319517438240,
    16194909420462031425,
    13871017173176583298,
    18303478847064064385,
    18232552689433215490,
    18090419998706369540,
    17806153522019305480,
    17237620560088797200,
    16100553540994408480,
    13826139127340482880,
    9205534180971414145,
];
pub const KING_MOVE_MASKS: [u64; 64] = [
    770,
    1797,
    3594,
    7188,
    14376,
    28752,
    57504,
    49216,
    197123,
    460039,
    920078,
    1840156,
    3680312,
    7360624,
    14721248,
    12599488,
    50463488,
    117769984,
    235539968,
    471079936,
    942159872,
    1884319744,
    3768639488,
    3225468928,
    12918652928,
    30149115904,
    60298231808,
    120596463616,
    241192927232,
    482385854464,
    964771708928,
    825720045568,
    3307175149568,
    7718173671424,
    15436347342848,
    30872694685696,
    61745389371392,
    123490778742784,
    246981557485568,
    211384331665408,
    846636838289408,
    1975852459884544,
    3951704919769088,
    7903409839538176,
    15806819679076352,
    31613639358152704,
    63227278716305408,
    54114388906344448,
    216739030602088448,
    505818229730443264,
    1011636459460886528,
    2023272918921773056,
    4046545837843546112,
    8093091675687092224,
    16186183351374184448,
    13853283560024178688,
    144959613005987840,
    362258295026614272,
    724516590053228544,
    1449033180106457088,
    2898066360212914176,
    5796132720425828352,
    11592265440851656704,
    4665729213955833856,
];
