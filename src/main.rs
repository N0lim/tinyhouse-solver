use std::todo;

use modular_bitfield::{Specifier, bitfield, prelude::*, specifiers::B4};

fn main() {
    println!("test");
}

/*
position encoding

0000 0001 0010 0011
0100 0101 0110 0111
1000 1001 1010 1011
1100 1101 1110 1111
*/

#[derive(Specifier)]
enum PieceType {
    Wazir = 0,
    Horse = 1,
    Ferz = 2,
    Pawn = 3,
}

#[repr(u64)]
#[bitfield]
pub struct Board {
    white_king_pos: B4,
    black_king_pos: B4,
    wazir_pos1: B4,
    wazir_pos2: B4,
    horse_pos1: B4,
    horse_pos2: B4,
    ferz_pos1: B4,
    ferz_pos2: B4,
    pawn_pos1: B4,
    pawn_pos2: B4,
    wazir_color1: bool,
    wazir_color2: bool,
    horse_color1: bool,
    horse_color2: bool,
    ferz_color1: bool,
    ferz_color2: bool,
    pawn_color1: bool,
    pawn_color2: bool,
    wazir_in_pocket1: bool,
    wazir_in_pocket2: bool,
    horse_in_pocket1: bool,
    horse_in_pocket2: bool,
    ferz_in_pocket1: bool,
    ferz_in_pocket2: bool,
    pawn_in_pocket1: bool,
    pawn_in_pocket2: bool,
    #[bits = 2]
    pawn_type1: PieceType,
    #[bits = 2]
    pawn_type2: PieceType,
    move_side: bool,
    #[skip]
    __: B3,
}

fn sort_board(board: Board) -> Board {
    todo!()
}

fn sort_wazirs(board: Board) -> Board {
    todo!()
}

fn sort_horses(board: Board) -> Board {
    todo!()
}

fn sort_ferzes(board: Board) -> Board {
    todo!()
}

fn sort_pawns(board: Board) -> Board {
    todo!()
}

fn generate_moves(board: Board) -> Board {
    todo!()
}

fn visualizer(board: Board) -> String {
    todo!()
}
