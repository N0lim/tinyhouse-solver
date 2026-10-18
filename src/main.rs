pub mod generators;
pub mod helpers;

use crate::generators::*;
use crate::helpers::*;
use core::panic;
use std::{
    iter::{Enumerate, Map},
    ops::Neg,
    print, todo,
};

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
0b01 = Horse
0b10 = Ferz
0b11 = Pawn
*/
const STARTING_POSITION: u64 = 0b_1100_0011;

// notice this function do not check if there is multiple pieces in one position
pub fn try_capture_piece(mut board: u64, position: u64) -> u64 {
    let q = get_pieces(board);
    for (i, piece) in (0..10u64).zip(q.iter()) {
        if piece.position == position
            && piece.in_pocket == Some(0)
            && piece.color != get_move_side(board)
        {
            if i < 2 {
                panic!("can't capture kings");
            }

            board = set_position(board, i, 0);
            board = set_color(board, i, get_color(board, i) ^ 1);
            board = set_in_pocket(board, i, 1);

            if piece.pawn_type.is_some_and(|x| x != 0b11) {
                board = set_pawn_type(board, i, 0b11);
            }
        }
    }
    board = set_move_side(board, get_move_side(board) ^ 1);
    sort_board(board)
}

fn visualizer(board: u64) -> String {
    todo!()
}

fn main() {}
