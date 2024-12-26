use crate::{board, Board, BLACK_BISHOP, BLACK_KNIGHT, WHITE_BISHOP};
use std::fs::OpenOptions;
use std::io::Write;
use std::collections::HashMap;
use std::collections::HashSet;

// a-file             0x0101010101010101
// h-file             0x8080808080808080
// 1st rank           0x00000000000000FF
// 8th rank           0xFF00000000000000
// a1-h8 diagonal     0x8040201008040201
// h1-a8 antidiagonal 0x0102040810204080
// light squares      0x55AA55AA55AA55AA
// dark squares       0xAA55AA55AA55AA55

//define the edge
const EDGE_MASK: u64 = 18411139144890810879;

//rank and file masks
//file masks
const A_FILE: u64 = 0x0101010101010101;
const B_FILE: u64 = 0x0202020202020202;
const C_FILE: u64 = 0x0404040404040404;
const D_FILE: u64 = 0x0808080808080808;
const E_FILE: u64 = 0x1010101010101010;
const F_FILE: u64 = 0x2020202020202020;
const G_FILE: u64 = 0x4040404040404040;
const H_FILE: u64 = 0x8080808080808080;

//rank masks
const FIRST_RANK: u64 = 0x00000000000000FF;
const SECOND_RANK: u64 = 0x000000000000FF00;
const THIRD_RANK: u64 = 0x0000000000FF0000;
const FOURTH_RANK: u64 = 0x00000000FF000000;
const FIFTH_RANK: u64 = 0x000000FF00000000;
const SIXTH_RANK: u64 = 0x0000FF0000000000;
const SEVENTH_RANK: u64 = 0x00FF000000000000;
const EIGHTH_RANK: u64 = 0xFF00000000000000;

//piece move masks
const KNIGHT_MOVE_MASKS: [u64; 64] = [
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
const BISHOP_MOVE_MASKS: [u64; 64] = [
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
const ROOK_MOVE_MASKS: [u64; 64] = [
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
const QUEEN_MOVE_MASKS: [u64; 64] = [
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
const KING_MOVE_MASKS: [u64; 64] = [
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



// Create magics for one piece on one square
fn one_magic(map: &HashMap<u64, Vec<u64>>) -> (Vec<u64>, u64) {
    let keys: Vec<u64> = map.keys().cloned().collect();

    let mut found = false;

    while (!found) {
       found = true; 





    }



}
// Create <blocker, legal> hashmap
fn create_first_map(blockers: &Vec<u64>, legals: &Vec<u64>) -> HashMap<u64, u64> {
    let mut result = HashMap::new();
    for i in 0..blockers.len() {
       result.insert(blockers[i], legals[i]); 
    }
    result
}

// Create <legal, Vec<blocker>> hashmap from <blocker, legal> map
fn create_second_map(map: &HashMap<u64, u64>) -> HashMap<u64, Vec<u64>> {
    let mut result: HashMap<u64, Vec<u64>> = HashMap::new();
    for (blocker, legal) in map {
        if result.contains_key(&legal) {
            result.get_mut(&legal).unwrap().push(*blocker);
        } 
        else {
            result.insert(*legal, vec![*blocker]);
        }
    }
    result
}

// This function takes in a vector and appends it to a specified file (currently hardcoded)
fn append_vector_to_file(vector: &[Vec<u64>]) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true) // Open the file with write access
        .append(true) // Append to the file instead of overwriting
        .create(true) // Create the file if it doesn't exist
        .open("src/attacks.rs")?; // Open the file

    writeln!(file, "pub static DATA: &[u64] = &{:?};", vector)?;
    Ok(())
}

// Saves to a file a list of all the blocking configs for a piece and position. each of these * a
// magic number will map to one of the actual attack maps
fn sliding_attacks(piece: u8, position: usize) -> () {
    let mut result: Vec<Vec<u64>> = Vec::new();

    //loop through each of the positions
    // for i in 0..64 {

    // }
    let mut blocks = blockers(crate::WHITE_ROOK, 0);

    result.push(blocks);
    blocks = blockers(crate::WHITE_ROOK, 1);
    result.push(blocks);
    append_vector_to_file(result.as_slice());
}

// fn rook_moves(board: &Board, position: u8, color: bool) -> Vec<Move> {
//     debug_assert!(
//         position <= 63,
//         "rook_moves received invalid position: {}",
//         position
//     );
//     let mut moves: Vec<Move> = Vec::new();

//     //set piece and colors
//     let piece = if color { WHITE_ROOK } else { BLACK_ROOK };
//     let board_color = if color { &board.white } else { &board.black };
//     let other_color = if color { &board.black } else { &board.white };

//     let mut obstructed = false;
//     //north loop
//     let mut distance: i8 = 8;
//     if position < 56 {
//         //check that it doesn't get too high or hit anything
//         while (position as i8 + distance <= 63) && !obstructed {
//             let (new_move, is_blocked) =
//                 move_helper(position, color, distance, piece, board_color, other_color);
//             if new_move.is_some() {
//                 moves.push(new_move.unwrap());
//             }
//             obstructed = is_blocked;
//             distance += 8;
//         }
//     }
//     obstructed = false;
//     //west loop
//     if position % 8 > 0 {
//         distance = -1;
//         //check that it doesn't wrap around or hit anything. >= 0 check needed because % is remainder NOT modulus
//         while ((position as i8 + distance) >= 0)
//             && (((position as i8 + distance) % 8) < 7)
//             && !obstructed
//         {
//             let (new_move, is_blocked) =
//                 move_helper(position, color, distance, piece, board_color, other_color);
//             if new_move.is_some() {
//                 moves.push(new_move.unwrap());
//             }
//             obstructed = is_blocked;
//             distance -= 1;
//         }
//     }
//     obstructed = false;
//     //east loop
//     if position % 8 < 7 {
//         distance = 1;
//         //check that it doesn't get too high, wrap around, or hit anything
//         while ((position as i8 + distance) % 8 > 0) && !obstructed {
//             let (new_move, is_blocked) =
//                 move_helper(position, color, distance, piece, board_color, other_color);
//             if new_move.is_some() {
//                 moves.push(new_move.unwrap());
//             }
//             obstructed = is_blocked;
//             distance += 1;
//         }
//     }
//     obstructed = false;
//     //south loop
//     if position > 7 {
//         distance = -8;
//         //check that it doesn't get too low or hit anything
//         while (0 <= position as i8 + distance) && !obstructed {
//             let (new_move, is_blocked) =
//                 move_helper(position, color, distance, piece, board_color, other_color);
//             if new_move.is_some() {
//                 moves.push(new_move.unwrap());
//             }
//             obstructed = is_blocked;
//             distance -= 8;
//         }
//     }

//     moves
// }

// This function returns a vec of all the on bit positions. e.g. 9 -> [0, 3]

// This function takes in a u64 and outputs a Vec of the indeces of the on bits
fn set_bit_positions(mut number: u64) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    for i in 0..63 {
        if (number & 1) == 1 {
            result.push(i);
        }
        number = number >> 1;
    }
    result
}

// This function generates a big list of all the possible blocker configurations for a given piece and square
fn blockers(piece: u8, position: usize) -> Vec<u64> {
    let mut mask: u64;
    match piece {

        crate::WHITE_BISHOP | crate::BLACK_BISHOP => {
            mask = BISHOP_MOVE_MASKS[position];
        }

        crate::WHITE_ROOK | crate::BLACK_ROOK => { mask = ROOK_MOVE_MASKS[position]; }

        crate::WHITE_QUEEN | crate::BLACK_QUEEN => { mask = QUEEN_MOVE_MASKS[position]; }

        _ => panic!("blockers didn't receive sliding piece")
    }

    // Get rid of the appropriate edges for the blocker mask
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

    // A list of the on-bit positions in the move mask
    let on_bits = set_bit_positions(mask);

    println!("{:b}", mask);

    // This helper recursively generates all the combinations of blockers.
    // For each potential blocker square we include it in one recursive call and exclude it in the other
    fn blockers_helper(list: Vec<u8>, acc: u64) -> Vec<u64> {
        match list.as_slice() {
            [] => {
                vec![acc]
            }
            _ => {
                let bit_set = 1 << list[0];
                let with_my_contribution = acc | bit_set;
                let (_, tail) = list.split_at(1);
                let mut result = blockers_helper(tail.to_vec(), with_my_contribution);
                result.extend(blockers_helper(tail.to_vec(), acc));
                result
            }
        }
    }

    blockers_helper(on_bits, 0)
}
 
// fn king_moves() -> Vec<u64> {
//     let mut moves: Vec<u64> = Vec::new();
//     for position in 0..64 {
//         let mut mask: u64 = 0;
//         //north moves
//         if position < 56 {
//             mask |= 1 << (position + 8);
//             if position % 8 > 0 {
//                 mask |= 1 << (position + 7);
//             }
//             if position % 8 < 7 {
//                 mask |= 1 << (position + 9);
//             }
//         }
//         //lateral moves
//         if position % 8 > 0 {
//             mask |= 1 << (position - 1);
//         }
//         if position % 8 < 7 {
//             mask |= 1 << (position + 1);
//         }
//         //south moves
//         if position > 7 {
//             mask |= 1 << (position - 8);
//             if position % 8 > 0 {
//                 mask |= 1 << (position - 9);
//             }
//             if position % 8 < 7 {
//                 mask |= 1 << (position - 7);
//             }
//         }
//         moves.push(mask);
//     }
//     moves
// }
//move mask generators
// fn queen_moves() -> Vec<u64> {
//     let mut moves: Vec<u64> = Vec::new();
//     for i in 0..64 {
//         let mask = bishop_move_masks[i] | rook_move_masks[i];
//         moves.push(mask);
//     }
//     moves
// }



// Generates all legal rook moves for all blocker configs given a position, returning blockers too
// for testing
fn rook_moves(position: u8) -> (Vec<u64>, Vec<u64>) {
    let mut moves: Vec<u64> = Vec::new();
    let mut mask: u64;
    let blockers = blockers(crate::WHITE_ROOK, position as usize);
    let blockers_clone = blockers.clone(); // FOR TESTING ONLY

    for blocked in blockers {
        mask = 0;

        // north loop
        let mut obstructed = false;
        let mut distance: i8 = 8;
        if position < 56 {
            //check that it doesn't get too high or hit anything
            while position as i8 + distance <= 63 && !obstructed{
                mask |= 1 << (position as i8 + distance);
                if blocked & (1 << (position as i8 + distance)) != 0 {
                    obstructed = true;
                }

                distance += 8;
            }
        }

        // west loop
        obstructed = false;
        if position % 8 > 0 {
            distance = -1;
            //check that it doesn't wrap around or hit anything. >= 0 check needed because % is remainder NOT modulus
            while ((position as i8 + distance) >= 0)
                && (((position as i8 + distance) % 8) < 7 
                && !obstructed)
            {
                mask |= 1 << (position as i8 + distance);
                if blocked & (1 << (position as i8 + distance)) != 0 {
                    obstructed = true;
                }
                distance -= 1;
            }
        }

        // east loop
        obstructed = false;
        if position % 8 < 7 {
            distance = 1;
            //check that it doesn't get too high, wrap around, or hit anything
            while (position as i8 + distance) % 8 > 0 && !obstructed {
                mask |= 1 << (position as i8 + distance);
                if blocked & (1 << (position as i8 + distance)) != 0 {
                    obstructed = true;
                }
                distance += 1;
            }
        }

        // south loop
        obstructed = false;
        if position > 7 {
            distance = -8;
            //check that it doesn't get too low or hit anything
            while 0 <= position as i8 + distance && !obstructed {
                mask |= 1 << (position as i8+ distance);
                if blocked & (1 << (position as i8 + distance)) != 0 {
                    obstructed = true;
                }
                distance -= 8;
            }
        }
        moves.push(mask);
    }
(blockers_clone,moves)
}

// fn bishop_moves() -> Vec<u64> {
//         let mut moves: Vec<u64> = Vec::new();
//         let mut mask: u64;
//     for position in 0..64 {
//         mask = 0;
//         let mut distance: i8 = 7;
//         //noWe loop
//         if (position < 56) && (position % 8 > 0) {
//             //check that it doesn't get too high, wrap around, or hit anything
//             while (position as i8 + distance <= 63)
//                 && ((position as i8 + distance) % 8 < 7)
//             {
//                 mask |= 1 << (position + distance);
//                 distance += 7;
//             }
//         }
//         //noEa loop
//         if (position < 56) && (position % 8 < 7) {
//             distance = 9;
//             //check that it doesn't get too high, wrap around, or hit anything
//             while (position as i8 + distance <= 63)
//                 && ((position as i8 + distance) % 8 > 0)
//             {
//                 mask |= 1 << (position + distance);
//                 distance += 9;
//             }
//         }
//         //soWe loop
//         if (position > 7) && (position % 8 > 0) {
//             distance = -9;
//             //check that it doesn't get too high, wrap around, or hit anything
//             while (0 <= position as i8 + distance)
//                 && ((position as i8 + distance) % 8 < 7)
//             {
//                 mask |= 1 << (position + distance);
//                 distance -= 9;
//             }
//         }
//         //soEa loop
//         if (position > 7) && (position % 8 < 7) {
//             distance = -7;
//             //check that it doesn't get too high, wrap around, or hit anything
//             while (0 <= position as i8 + distance)
//                 && ((position as i8 + distance) % 8 > 0)
//             {
//                 mask |= 1 << (position + distance);
//                 distance -= 7;
//             }
//         }
//         moves.push(mask);
//     }
//     moves
// }
 
// fn knight_moves_generator() ->Vec<u64> {
//     let mut moves: Vec<u64> = Vec::new();
//     let mut mask: u64;
//     for position in 0..64 {
//         mask = 0;
//         if position < 56 {
//             //north long jumps
//             if position < 48 {
//                 //noWe long jumps. Bound check, make sure no piece of same color is on destination square. Add move if all good.
//                 if position % 8 > 0 {
//                     mask |= 1 << (position + 15);
//                 }
//                 //noEa long jumps
//                 if position % 8 < 7 {
//                    mask |= 1 << (position + 17);
//                 }
//             }
//             //noWe wide jumps
//             if position % 8 > 1 {
//                 mask |= 1 << (position + 6);
//             }
//             //noEA wide jumps
//             if position % 8 < 6 {
//                 mask |= 1 << (position + 10);
//             }
//         }
//         //south jumps
//         if position > 7 {
//             //soWe wide jumps
//             if position % 8 > 1 {
//                 mask |= 1 << (position - 10);
//             }
//             //soEa wide jumps
//             if position % 8 < 6 {
//                 mask |= 1 << (position - 6);
//             }
//             //south long jumps
//             if position > 15 {
//                 //soWe long jumps
//                 if position % 8 > 0 {
//                     mask |= 1 << (position - 17);
//                 }
//                 //soEa long jumps
//                 if position % 8 < 7 {
//                     mask |= 1 << (position - 15);
//                 }
//             }
//         }
//         moves.push(mask);
//     }
//     moves
// }




fn print_binary_board(value: u64) {
     let binary_string = format!("{:064b}", value); // Convert to a 64-bit binary string
    let reversed_binary_string: String = binary_string.chars().rev().collect(); // Reverse the entire string

    // Print chunks in reverse order directly
    reversed_binary_string.as_bytes().chunks(8).rev().for_each(|chunk| {
        println!("{}", std::str::from_utf8(chunk).unwrap());
    });
}




#[cfg(test)]

#[test]
fn legalvecmap() {

    let (blockers, legals) = rook_moves(0);
    let map1 = create_first_map(&blockers, &legals);
    let map2 = create_second_map(&map1);
    
    let mut duplicates = HashSet::new();
    let mut no_duplicates = true;

    let key = map2.keys().next();
    let values = map2.get(key.unwrap());
    println!("Key: ");
    print_binary_board(*key.unwrap());
    println!("Values: ");
    for legal in values.unwrap() {
        print_binary_board(*legal);
        println!("");
        
        if duplicates.contains(legal) {
            no_duplicates = false;
        } else {
            duplicates.insert(*legal);
        }

    }

    assert!(no_duplicates);

}

#[test]
fn rook_legal_bitboards() {
    // For position 0
    let (blockers, legals) = rook_moves(0);

    println!("POSITION 0: ");

    println!("Blocker 0: ");
    print_binary_board(blockers[0]);
    println!("Legal 0:");
    print_binary_board(legals[0]);

    println!("Blocker 300: ");
    print_binary_board(blockers[300]);
    println!("Legal 300:");
    print_binary_board(legals[300]);
    
    println!("Blocker 2051: ");
    print_binary_board(blockers[2051]);
    println!("Legal 2051:");
    print_binary_board(legals[2051]);

    println!("Blocker 4003: ");
    print_binary_board(blockers[4003]);
    println!("Legal 4003:");
    print_binary_board(legals[4003]);

    // For position 27
    let (blockers, legals) = rook_moves(27);
    println!("POSITION 27: ");

    println!("Blocker 1: ");
    print_binary_board(blockers[1]);
    println!("Legal 1:");
    print_binary_board(legals[1]);

    println!("Blocker 451: ");
    print_binary_board(blockers[451]);
    println!("Legal 451:");
    print_binary_board(legals[451]);

    println!("Blocker 713: ");
    print_binary_board(blockers[713]);
    println!("Legal 713:");
    print_binary_board(legals[713]);
    
    println!("Blocker 1010: ");
    print_binary_board(blockers[1010]);
    println!("Legal 1010:");
    print_binary_board(legals[1010]);
}



#[test]
fn mask() {
    blockers(crate::WHITE_ROOK, 0);
}

#[test]
fn generate_attacks() {
    use crate::WHITE_ROOK;

    sliding_attacks(WHITE_ROOK, 0);
}

#[test]
fn blocker_list() {
    use crate::{WHITE_KNIGHT, WHITE_ROOK};
    let mut blocks = blockers(WHITE_ROOK, 1);

    for elem in &blocks {
        if elem & !ROOK_MOVE_MASKS[1] != 0 {
            assert!(
                false,
                "Some blocker had 1 bits outside of the rook's move mask"
            );
        }
    }

    // println!("blockers: {:#?}", blockers);
    // println!("the numbuh of blockuhs: {}", blocks.len());
    assert_eq!(blocks.len() as u32, 2u32.pow(11));

    blocks = blockers(WHITE_BISHOP, 20);

    for elem in &blocks {
        if elem & !BISHOP_MOVE_MASKS[20] != 0 {
            assert!(
                false,
                "Some blocker had 1 bits outside of the bishop's move mask"
            );
        }
    }
    println!("blockers: {:#?}", blocks);
    println!("the numbuh of blockuhs: {}", blocks.len());
    assert_eq!(blocks.len() as u32, 2u32.pow(7));
}
// #[test]
// fn positions() {
//     let list = set_bit_positions(9,);
//     println!("{:#?}", list);
//     assert_eq!(1, 1);
// }

// #[test]
// fn masks() {
//     let list = knight_moves();
//     println!("{:#?}", list);
//     assert_eq!(1, 1);
// }
// #[test]
// fn bishop_masks() {
//     let list = bishop_moves();
//     println!("{:#?}", list);
//     assert_eq!(1, 0);
// }
// #[test]
// fn rook_masks() {
//     let list = rook_moves();
//     println!("{:#?}", list);
//     assert_eq!(1, 0);
// }
// #[test]
// fn queen_masks() {
//     let list = queen_moves();
//     println!("{:#?}", list);
//     assert_eq!(1, 0);
// }
// #[test]
// fn king_masks() {
//     let list = king_moves();
//     println!("{:#?}", list);
//     assert_eq!(1, 0);
// }

//TODO: when generating attack masks from occupancy masks, reuse board.rs code but make all obstructions the other color.
