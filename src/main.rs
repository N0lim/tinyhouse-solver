pub mod generators;
pub mod helpers;
pub mod visulization;

use crate::generators::*;
use crate::helpers::*;
use crate::visulization::*;
use std::{
    iter::{Enumerate, Map},
    print, todo,
};

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

fn main() {
    print!("{}", init_table(STARTING_POSITION))
}
