use std::{default, fs::TryLockError::Error};

fn main() {
    println!("Hello, world!");
}

/*
1 bit move side
4 bits white king position
4 bits black king position
5 * 8 = 40 bits for other pieces position (board and pocket)
3 * 2 = 6 bits for pawns promotion types

sum 55 bits
*/

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Color {
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
    board: [Option<BoardPiece>; 16],
    pocket: [Option<PocketPiece>; 8],
}

impl TryFrom<u64> for Position {
    type Error = &'static str;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value >> 55 != 0 {
            return Err("Unexpected data appears");
        }
        let position: Position = Default::default();
        Ok(position)
    }
}

// fn board_converter()

// fn visualizer(board:u64) -> String { }
