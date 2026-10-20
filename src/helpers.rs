/*
position encoding

0000 0001 0010 0011
0100 0101 0110 0111
1000 1001 1010 1011
1100 1101 1110 1111

color encoding
white = 0|False, black = 1|True

pawn type encoding
0b00 = Wazir
0b01 = Hors
0b10 = Ferz
0b11 = Pawn
*/
pub const STARTING_POSITION: u64 =
    0b_11_11_00000000_10101010_0111_1000_0000_1111_0001_1110_0010_1101_1100_0011;

// pieces should be sorted in lexicographic order of fields, so they placed strangely
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChessPiece {
    pub color: u64,
    pub in_pocket: Option<u64>,
    pub position: u64,
    pub pawn_type: Option<u64>,
}

pub fn set_bits(mut num: u64, start: u64, len: u64, bits: u64) -> u64 {
    assert!(len != 0, "length equals zero");
    assert!(start + len <= 64, "setted bits are out of bounds of u64");
    let mut nullifier = u64::MAX;
    if len < 64 {
        nullifier = (1 << len) - 1;
    }
    assert!(bits <= nullifier, "number of bits bigger than length");
    num &= !(nullifier << start);
    num |= bits << start;
    num
}

pub fn set_position(board: u64, piece_index: u64, position: u64) -> u64 {
    assert!(piece_index <= 9, "piece index can't be bigger than 9");
    set_bits(board, piece_index * 4, 4, position)
}

pub fn set_color(board: u64, piece_index: u64, color: u64) -> u64 {
    assert!(piece_index <= 9, "piece index can't be bigger than 9");
    assert!(piece_index >= 2, "can't set colors of kings");
    set_bits(board, (piece_index - 2) + 40, 1, color)
}

pub fn set_in_pocket(board: u64, piece_index: u64, in_pocket: u64) -> u64 {
    assert!(piece_index <= 9, "piece index can't be bigger than 9");
    assert!(piece_index >= 2, "can't put kings into pocket");
    set_bits(board, (piece_index - 2) + 48, 1, in_pocket)
}

pub fn set_pawn_type(board: u64, piece_index: u64, pawn_type: u64) -> u64 {
    assert!(piece_index <= 9, "piece index can't be bigger than 9");
    assert!(piece_index >= 8, "can't set pawn type for non-pawn pieces");
    set_bits(board, (piece_index - 8) * 2 + 56, 2, pawn_type)
}

pub fn set_move_side(board: u64, move_side: u64) -> u64 {
    assert!(
        move_side <= 1,
        "move side can't be bigger than 1 (black moves)"
    );
    set_bits(board, 60, 1, move_side)
}

pub fn get_bits(mut num: u64, start: u64, len: u64) -> u64 {
    assert!(len != 0, "length equals zero");
    assert!(start + len <= 64, "getted bits are out of bounds of u64");
    let mut get_mask = u64::MAX;
    if len < 64 {
        get_mask = (1 << len) - 1;
    }
    num &= (get_mask << start);
    num >> start
}

pub fn get_position(board: u64, piece_index: u64) -> u64 {
    assert!(piece_index <= 9, "piece index can't be bigger than 9");
    get_bits(board, piece_index * 4, 4)
}

pub fn get_color(board: u64, piece_index: u64) -> u64 {
    assert!(piece_index <= 9, "piece index can't be bigger than 9");
    if piece_index < 2 {
        return (piece_index == 1) as u64;
    }
    get_bits(board, (piece_index - 2) + 40, 1)
}

pub fn get_in_pocket(board: u64, piece_index: u64) -> u64 {
    assert!(piece_index <= 9, "piece index can't be bigger than 9");
    assert!(piece_index >= 2, "can't get kings from pocket");
    get_bits(board, (piece_index - 2) + 48, 1)
}

pub fn get_pawn_type(board: u64, piece_index: u64) -> u64 {
    assert!(piece_index <= 9, "piece index can't be bigger than 9");
    assert!(piece_index >= 8, "can't get pawn type for non-pawn pieces");
    get_bits(board, (piece_index - 8) * 2 + 56, 2)
}

pub fn get_move_side(board: u64) -> u64 {
    get_bits(board, 60, 1)
}

pub fn get_pieces(board: u64) -> [ChessPiece; 10] {
    let mut arr = [ChessPiece {
        position: 999,
        color: 999,
        in_pocket: None,
        pawn_type: None,
    }; 10];

    for (i, piece) in (0..10u64).zip(arr.iter_mut()) {
        piece.position = get_position(board, i);

        piece.color = get_color(board, i);

        if i >= 2 {
            piece.in_pocket = Some(get_in_pocket(board, i));
        }

        if i >= 8 {
            piece.pawn_type = Some(get_pawn_type(board, i));
        }
    }

    arr
}

pub fn is_on_board(piece: ChessPiece) -> bool {
    piece.in_pocket != Some(1)
}

// returns piece by position if piece is not in pocket
pub fn try_get_piece_by_position(board: u64, position: u64) -> Option<ChessPiece> {
    let pieces = get_pieces(board);
    pieces
        .into_iter()
        .find(|piece| piece.position == position && piece.in_pocket != Some(1))
}

pub fn swap_pieces_pair(mut board: u64, pair_index: u64) -> u64 {
    assert!(pair_index <= 4, "piece pair index can't be bigger than 4");
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

pub fn sort_board(mut board: u64) -> u64 {
    let pieces = get_pieces(board);
    for pair_index in 0..5u64 {
        let index1 = pair_index * 2;
        let index2 = pair_index * 2 + 1;
        if pieces[index1 as usize] > pieces[index2 as usize] {
            board = swap_pieces_pair(board, pair_index);
        }
    }
    board
}

pub fn position_as_offset(position: u64) -> (i8, i8) {
    assert!(position <= 0b1111, "position is too big: {:b}", position);
    let x = (position % 4) as i8;
    let y = (position >> 2) as i8;
    (x, y)
}

pub fn apply_offset((x_offset, y_offset): (i8, i8), position: u64) -> Option<u64> {
    let (x, y) = position_as_offset(position);
    let offsetted_x = x + x_offset;
    let offsetted_y = y + y_offset;
    if !(0..=0b11).contains(&offsetted_x) || !(0..=0b11).contains(&offsetted_y) {
        return None;
    }
    Some(((offsetted_y as u64) << 2) | (offsetted_x as u64))
}

pub fn apply_offsets<I>(offsets: I, position: u64) -> impl Iterator<Item = u64>
where
    I: IntoIterator<Item = (i8, i8)>,
{
    offsets
        .into_iter()
        .filter_map(move |offset| apply_offset(offset, position))
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

    #[test]
    fn sort_starting_position_equal() {
        assert_eq!(sort_board(STARTING_POSITION), STARTING_POSITION)
    }

    #[test]
    fn position_as_offset_max() {
        assert_eq!(position_as_offset(0b1111), (0b11, 0b11))
    }

    #[test]
    fn position_as_offset_min() {
        assert_eq!(position_as_offset(0b0000), (0b00, 0b00))
    }

    #[test]
    fn apply_offset_plus_one_x() {
        assert_eq!(Some(0b1110 + 1), apply_offset((1, 0), 0b1110));
    }

    #[test]
    fn apply_offset_plus_one_y() {
        assert_eq!(Some(0b1011 + 0b0100), apply_offset((0, 1), 0b1011))
    }
}
