use crate::constants::*;

// Number of on bits in the blocker mask per square (excludes current square and edges)
const RBITS: [u8; 64] = [
    12, 11, 11, 11, 11, 11, 11, 12, 11, 10, 10, 10, 10, 10, 10, 11, 11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11, 11, 10, 10, 10, 10, 10, 10, 11, 11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11, 12, 11, 11, 11, 11, 11, 11, 12,
];

const BBITS: [u8; 64] = [
    6, 5, 5, 5, 5, 5, 5, 6, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 7, 7, 7, 7, 5, 5, 5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5, 5, 5, 7, 7, 7, 7, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 6, 5, 5, 5, 5, 5, 5, 6,
];

// Returns rook and bishop attack tables and magics. rook_magics[i] is used to map rook blockers
// when rook is at position i to the appropriate attack mask in the rook_attacks[i] set of attack
// boards
fn init_bitboards() -> ((Vec<Vec<u64>>, Vec<u64>), (Vec<Vec<u64>>, Vec<u64>)) {
    let mut rook_attacks = Vec::new();
    let mut rook_magics = Vec::new();
    let mut bishop_attacks = Vec::new();
    let mut bishop_magics = Vec::new();

    for i in 0..64 {
        let (table, magic) = find_magics(WHITE_ROOK, i);
        rook_attacks.push(table);
        rook_magics.push(magic);
    }

    for i in 0..64 {
        let (table, magic) = find_magics(WHITE_BISHOP, i);
        bishop_attacks.push(table);
        bishop_magics.push(magic);
    }

    ((rook_attacks, rook_magics), (bishop_attacks, bishop_magics))

}

// Returns an attack table and associated magic number for one piece and position
fn find_magics(piece: u8, position: u8) -> (Vec<u64>, u64) {
    // Gets list of blockers and attack maps. blockers[i] maps to attacks[i]
    let (blockers, attacks) = match piece {
        WHITE_ROOK => rook_moves(position),
        WHITE_BISHOP => bishop_moves(position),
        _ => panic!("Can't find magics for non-sliding pieces"),
    };
    // Number of on bits in the piece's occupancy mask determines how big to make the attack mask
    // table and therefore how much to shift (blocker * magic). The table is 2^n where n is
    // on_bits.
    let on_bits = match piece {
        WHITE_ROOK => RBITS[position as usize],
        WHITE_BISHOP => BBITS[position as usize],
        _ => panic!("Can't find magics for non-sliding pieces"),
    };

    // Try to build a valid table with a random magic (three combined so we have fewer on bits)
    // until we find a magic that works
    loop {
        let mut magic = fastrand::u64(..) & fastrand::u64(..) & fastrand::u64(..);
        if let Ok(table) = build_table(magic, on_bits, &blockers, &attacks) {
            return (table, magic);
        }
    }
}

struct TableError;

// Try to return a hashtable given a magic
fn build_table(
    magic: u64,
    on_bits: u8,
    blockers: &Vec<u64>,
    attacks: &Vec<u64>,
) -> Result<Vec<u64>, TableError> {
    let mut table = vec![0; 1 << on_bits];
    let shifts: u64 = 64 - (on_bits as u64);
    let blocker_len = blockers.len();

    for i in 0..blocker_len {
        let product = blockers[i].wrapping_mul(magic);
        let index = (product >> shifts) as usize;

        if table[index] == 0 {
            table[index] = attacks[i];
        } else if table[index] != attacks[i] {
            return Err(TableError);
        }
    }
    Ok(table)
}

// This function returns a vec of all the on bit positions. e.g. 9 -> [0, 3]
// It takes in a u64 and outputs a Vec of the indices of the on bits
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
        WHITE_BISHOP | BLACK_BISHOP => {
            mask = BISHOP_MOVE_MASKS[position];
        }

        WHITE_ROOK | BLACK_ROOK => {
            mask = ROOK_MOVE_MASKS[position];
        }

              _ => panic!("blockers didn't receive sliding piece"),
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

// Generates all legal rook moves for all blocker configs given a position, returning blockers too
// for testing
fn rook_moves(position: u8) -> (Vec<u64>, Vec<u64>) {
    let mut moves: Vec<u64> = Vec::new();
    let mut mask: u64;
    let blockers = blockers(WHITE_ROOK, position as usize);

    for blocked in blockers.iter() {
        mask = 0;

        // north loop
        let mut obstructed = false;
        let mut distance: i8 = 8;
        if position < 56 {
            //check that it doesn't get too high or hit anything
            while position as i8 + distance <= 63 && !obstructed {
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
                && (((position as i8 + distance) % 8) < 7 && !obstructed)
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
                mask |= 1 << (position as i8 + distance);
                if blocked & (1 << (position as i8 + distance)) != 0 {
                    obstructed = true;
                }
                distance -= 8;
            }
        }
        moves.push(mask);
    }
    (blockers, moves)
}

// Generates all legal bishop moves for all blocker configs given a position, returning blockers too
// for testing
fn bishop_moves(position: u8) -> (Vec<u64>, Vec<u64>) {
    let mut moves: Vec<u64> = Vec::new();
    let mut mask: u64;
    let blockers = blockers(WHITE_BISHOP, position as usize);

    for blocked in blockers.iter() {
        mask = 0;

        // northwest loop
        let mut obstructed = false;
        let mut distance: i8 = 7;
        if (position < 56) && (position % 8) > 0 {
            //check that it doesn't get too high or hit anything
            while (position as i8 + distance < 63)
                && ((position as i8 + distance) % 8 < 7)
                && !obstructed
            {
                mask |= 1 << (position as i8 + distance);
                if blocked & (1 << (position as i8 + distance)) != 0 {
                    obstructed = true;
                }

                distance += 7;
            }
        }

        // northeast loop
        obstructed = false;
        if (position < 56) && (position % 8 < 7) {
            distance = 9;
            while ((position as i8 + distance) <= 63)
                && ((position as i8 + distance) % 8 > 0)
                && !obstructed
            {
                mask |= 1 << (position as i8 + distance);
                if blocked & (1 << (position as i8 + distance)) != 0 {
                    obstructed = true;
                }
                distance += 9;
            }
        }

        // southwest loop
        obstructed = false;
        if (position > 7) && (position % 8 > 0) {
            distance = -9;
            //check that it doesn't get too high, wrap around, or hit anything
            while ((position as i8 + distance) >= 0)
                && ((position as i8 + distance) % 8 < 7)
                && !obstructed
            {
                mask |= 1 << (position as i8 + distance);
                if blocked & (1 << (position as i8 + distance)) != 0 {
                    obstructed = true;
                }
                distance -= 9;
            }
        }

        // southeast loop
        obstructed = false;
        if (position > 7) && (position % 8 < 7) {
            distance = -7;
            //check that it doesn't get too low or hit anything
            while ((position as i8 + distance) >= 0)
                && ((position as i8 + distance) % 8 > 0)
                && !obstructed
            {
                mask |= 1 << (position as i8 + distance);
                if blocked & (1 << (position as i8 + distance)) != 0 {
                    obstructed = true;
                }
                distance -= 7;
            }
        }
        moves.push(mask);
    }
    (blockers, moves)
}

fn print_binary_board(value: u64) {
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


#[cfg(test)]

#[test]
fn timed() {
    use std::hint::black_box;
    use std::time::Instant;
    let now = Instant::now();
    for i in 0..64 {
        let l = find_magics(WHITE_ROOK, i);
        black_box(l);
    }
    for i in 0..64 {
        let l = find_magics(WHITE_BISHOP, i);
        black_box(l);
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

}
#[test]
fn init() {
let ((rook_attacks, rook_magics), (bishop_attacks, bishop_magics)) = init_bitboards();

let blocker = blockers(WHITE_ROOK, 20);

println!("");
println!("Rook Blocker: ");
println!("");
print_binary_board(blocker[1000]);
println!("");

    let index = (blocker[1000].wrapping_mul(rook_magics[20])) >> (64 - RBITS[20]);
    println!("Rook Attacks: ");
    println!("");
    print_binary_board(rook_attacks[20][index as usize]);
println!("");
    
let blocker = blockers(WHITE_BISHOP, 21);

println!("");
println!("Bishop Blocker: ");
println!("");
print_binary_board(blocker[100]);
println!("");

    let index = (blocker[100].wrapping_mul(bishop_magics[21])) >> (64 - BBITS[21]);
    println!("Bishop Attacks: ");
    println!("");
    print_binary_board(bishop_attacks[21][index as usize]);
println!("");


}

#[test]
fn better_magic() {

    for i in 0..64 {
    let (table, magic) = find_magics(WHITE_ROOK, i);
    }
    for i in 0..64 {
    let (table, magic) = find_magics(WHITE_BISHOP, i);
    }
let (blockers, _legals) = rook_moves(0);
let (table, magic) = find_magics(WHITE_ROOK, 0);

println!("");
println!("Blocker: ");
println!("");
print_binary_board(blockers[0]);
println!("");

    let index = (blockers[0].wrapping_mul(magic)) >> (64 - RBITS[0]);
    println!("Attacks: ");
    println!("");
    print_binary_board(table[index as usize]);
println!("");

let (blockers, _legals) = rook_moves(26);
let (table, magic) = find_magics(WHITE_ROOK, 26);

println!("");
println!("Blocker: ");
println!("");
print_binary_board(blockers[26]);
println!("");

    let index = (blockers[26].wrapping_mul(magic)) >> (64 - RBITS[26]);
    println!("Attacks: ");
    println!("");
    print_binary_board(table[index as usize]);
println!("");




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

 
