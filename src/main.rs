use std::ops::{BitAnd, Shl, Shr, Sub};

fn main() {
    println!("{:b}", get_bits(0b101010101, 0, 4));
}

/*
1 bit move side
4 bits white king position
4 bits black king position
5 * 8 bits for other pieces position (board and pocket)
1 * 8 bits for color
3 * 2 = 6 bits for pawns promotion types

sum 63 bits
*/

/*
position encoding

0000 0001 0010 0011
0100 0101 0110 0111
1000 1001 1010 1011
1100 1101 1110 1111

*/

// do not use numbers >= 64 for start and length
fn get_bits<T1, T2, T3>(number: T1, start: T2, length: T3) -> T1
where
    T1: Shr<T2, Output = T1>
        + Shl<T3, Output = T1>
        + BitAnd<T1, Output = T1>
        + Sub<T1, Output = T1>
        + From<u8>,
{
    (number >> start) & ((T1::from(1) << length) - T1::from(1))
}

/*
#[derive(PartialEq, Eq, PartialOrd, Ord, Default)]
enum Color {
    #[default]
    White = 0,
    Black = 1,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum PieceType {
    King = 0,
    Wazir = 1,
    Horse = 2,
    Ferz = 3,
    Pawn = 4,
}

struct BoardPiece {
    color: Color,
    cell_type: PieceType,
}

struct PocketPiece {
    color: Color,
    cell_type: PieceType,
    was_pawn: bool,
}

#[derive(Default)]
struct Position {
    current_move: Color,
    board: [Option<BoardPiece>; 16],
    pocket: [Option<PocketPiece>; 8],
}

impl TryFrom<u64> for Position {
    type Error = &'static str;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value >> 55 != 0 {
            return Err("Unexpected data, more than 55 bits");
        }

        let mut position: Position = Default::default();

        match value | 1 {
            0 => position.current_move = White,
            _ => position.current_move = Black,
        }
        value >>= 1;

        let wk: BoardPiece = BoardPiece {
            color: White,
            cell_type: King,
        };

        position.board[(value | 0b1111) as usize] = Some(wk);

        Ok(position)
    }
} */

// fn board_converter()

// fn visualizer(board:u64) -> String { }
