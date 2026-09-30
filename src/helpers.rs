pub fn set_bits(mut num: u64, start: u64, len: u64, bits: u64) -> u64 {
    if len == 0 {
        panic!("length equals zero")
    }
    if start + len > 64 {
        panic!("setted bits are out of bounds of u64");
    }
    let mut nullifier = u64::MAX;
    if len < 64 {
        nullifier = (1 << len) - 1;
    }
    if bits > nullifier {
        panic!("number of bits bigger than length");
    }
    num &= !(nullifier << start);
    num |= bits << start;
    num
}

pub fn set_position(board: u64, piece_index: u64, position: u64) -> u64 {
    if piece_index > 9 {
        panic!("piece index can't be bigger than 9");
    }
    set_bits(board, piece_index * 4, 4, position)
}

pub fn set_color(board: u64, piece_index: u64, color: u64) -> u64 {
    if piece_index > 9 {
        panic!("piece index can't be bigger than 9");
    }
    if piece_index < 2 {
        panic!("can't set colors of kings");
    }
    set_bits(board, (piece_index - 2) + 40, 1, color)
}

pub fn set_in_pocket(board: u64, piece_index: u64, in_pocket: u64) -> u64 {
    if piece_index > 9 {
        panic!("piece index can't be bigger than 9");
    }
    if piece_index < 2 {
        panic!("can't put kings into pocket");
    }
    set_bits(board, (piece_index - 2) + 48, 1, in_pocket)
}

pub fn set_pawn_type(board: u64, piece_index: u64, pawn_type: u64) -> u64 {
    if piece_index > 9 {
        panic!("piece index can't be bigger than 9");
    }
    if piece_index < 8 {
        panic!("can't set pawn type for non-pawn pieces");
    }
    set_bits(board, (piece_index - 8) * 2 + 56, 2, pawn_type)
}

pub fn get_bits(num: u64, start: u64, len: u64) -> u64 {
    if len == 0 {
        panic!("length equals zero")
    }
    if start + len > 64 {
        panic!("getted bits are out of bounds of u64");
    }
    let mut get_mask = u64::MAX;
    if len < 64 {
        get_mask = (1 << len) - 1;
    }
    num & (get_mask << start)
}

pub fn get_position(board: u64, piece_index: u64) -> u64 {
    if piece_index > 9 {
        panic!("piece index can't be bigger than 9");
    }
    get_bits(board, piece_index * 4, 4)
}

pub fn get_color(board: u64, piece_index: u64) -> u64 {
    if piece_index > 9 {
        panic!("piece index can't be bigger than 9");
    }
    if piece_index < 2 {
        panic!("can't set colors of kings");
    }
    get_bits(board, (piece_index - 2) + 40, 1)
}

pub fn get_in_pocket(board: u64, piece_index: u64) -> u64 {
    if piece_index > 9 {
        panic!("piece index can't be bigger than 9");
    }
    if piece_index < 2 {
        panic!("can't put kings into pocket");
    }
    get_bits(board, (piece_index - 2) + 48, 1)
}

pub fn get_pawn_type(board: u64, piece_index: u64) -> u64 {
    if piece_index > 9 {
        panic!("piece index can't be bigger than 9");
    }
    if piece_index < 8 {
        panic!("can't set pawn type for non-pawn pieces");
    }
    get_bits(board, (piece_index - 8) * 2 + 56, 2)
}

pub fn swap_pieces_pair(mut board: u64, pair_index: u64) -> u64 {
    if pair_index > 4 {
        panic!("piece pair index can't be bigger than 4");
    }
    let index1 = pair_index * 2;
    let index2 = pair_index * 2 + 1;
    let pos1 = get_position(board, index1);
    let pos2 = get_position(board, index2);
    board = set_position(board, index1, pos2);
    board = set_position(board, index2, pos1);

    if pair_index >= 1 {
        let color1 = get_color(board, index1);
        let color2 = get_color(board, index2);
        board = set_color(board, index1, color2);
        board = set_color(board, index2, color1);
        let in_pocket1 = get_in_pocket(board, index1);
        let in_pocket2 = get_in_pocket(board, index2);
        board = set_in_pocket(board, index1, in_pocket2);
        board = set_in_pocket(board, index2, in_pocket1);
    }

    if pair_index == 4 {
        let pawn_type1 = get_pawn_type(board, index1);
        let pawn_type2 = get_pawn_type(board, index2);
        board = set_pawn_type(board, index1, pawn_type2);
        board = set_pawn_type(board, index2, pawn_type1);
    }

    board
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    #[should_panic(expected = "length equals zero")]
    fn set_bits_panic_zero_length() {
        assert_eq!(0, set_bits(0, 0, 0, 0));
    }

    #[test]
    #[should_panic(expected = "setted bits are out of bounds of u64")]
    fn set_bits_panic_start_too_big() {
        set_bits(0, 64, 1, 0);
    }

    #[test]
    #[should_panic(expected = "setted bits are out of bounds of u64")]
    fn set_bits_panic_length_too_big() {
        set_bits(0, 0, 65, 0);
    }

    #[test]
    #[should_panic(expected = "setted bits are out of bounds of u64")]
    fn set_bits_panic_sum_too_big() {
        set_bits(0, 33, 32, 0);
    }

    #[test]
    #[should_panic(expected = "number of bits bigger than length")]
    fn set_bits_panic_bigger_than_length() {
        set_bits(0, 0, 1, 0b11);
    }

    proptest! {
        #[test]
        fn set_bits_proptest_check(
            (start, len, bits) in (1..=64u64).prop_flat_map(|len| {
                let max_start = 64 - len;
                let max_bits = if len == 64 {u64::MAX} else {(1 << len) - 1};
                (0..=max_start, Just(len), 0..=max_bits)
            }))
        {
            println!("start = {}, len = {}, bits = {}", start, len, bits);
            assert_eq!(bits << start, set_bits(0, start, len, bits));
        }
    }

    #[test]
    fn set_bits_full_replacement() {
        assert_eq!(u64::MAX, set_bits(0, 0, 64, u64::MAX));
    }

    #[test]
    fn set_bits_one_bit() {
        for i in 0..64u64 {
            let replaced: u64 = set_bits(0, i, 1, 1);
            let bit_replaced: u64 = 1 << i;
            assert_eq!(replaced, bit_replaced);
        }
    }

    #[test]
    fn set_bits_two_bits() {
        for i in 0..63u64 {
            let replaced: u64 = set_bits(0, i, 2, 0b11);
            let bit_replaced: u64 = 0b11 << i;
            assert_eq!(replaced, bit_replaced);
        }
    }
}
