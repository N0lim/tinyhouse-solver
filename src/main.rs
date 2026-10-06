pub mod helpers;

use crate::helpers::*;

use core::panic;

use std::{
    collections::btree_map::Range,
    fmt::DebugTuple,
    iter::{Enumerate, Map},
    print, todo,
};

// notice this function do not check if there is multiple pieces in one position
pub fn try_capture_piece(mut board: u64, position: u64) -> u64 {
    let q = get_chess_pieces(board);
    for (i, piece) in (0..10u64).zip(q.iter()) {
        if piece.position == position
            && piece.in_pocket == Some(0)
            && piece.color != Some(get_move_side(board))
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

pub fn get_blocked_positions(board: u64) -> Vec<u64> {
    let mut result = Vec::new();
    let pieces = get_chess_pieces(board);
    for piece in pieces {
        if piece.in_pocket != Some(1) && piece.color == Some(get_move_side(board)) {
            result.push(piece.position);
        }
    }
    result
}

pub fn generate_new_wazir_positions(position: u64) -> Vec<u64> {
    let mut result = Vec::new();
    let offsets = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
    let mut x = position as i8 % 0b11;
    let mut y = position as i8 / 0b11;
    for (x_offset, y_offset) in offsets {
        x += x_offset;
        y += y_offset;

        if !(0..=0b11).contains(&x) || !(0..=0b11).contains(&y) {
            continue;
        }
        result.push(((y as u64) << 2) & (x as u64));
    }
    result
}

pub fn generate_new_ferz_positions(position: u64) -> Vec<u64> {
    let mut result = Vec::new();
    let offsets = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut x = position as i8 % 0b11;
    let mut y = position as i8 / 0b11;
    for (x_offset, y_offset) in offsets {
        x += x_offset;
        y += y_offset;

        if !(0..=0b11).contains(&x) || !(0..=0b11).contains(&y) {
            continue;
        }
        result.push(((y as u64) << 2) & (x as u64));
    }
    result
}

pub fn generate_wazir_moves(board: u64) -> Vec<u64> {
    let result = Vec::new();
    let original_board = board;

    todo!();
    result
}

fn visualizer(board: u64) -> String {
    todo!()
}

fn main() {}
