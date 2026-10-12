pub mod helpers;

use proptest::result;

use crate::helpers::*;

use core::panic;

use std::{
    collections::btree_map::Range,
    fmt::DebugTuple,
    iter::{Enumerate, Map},
    ops::Neg,
    print, todo,
};

// notice this function do not check if there is multiple pieces in one position
pub fn try_capture_piece(mut board: u64, position: u64) -> u64 {
    let q = get_pieces(board);
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

fn is_available_to_move(board: u64, from: u64, to: u64) -> bool {
    let from_piece_option = try_get_piece_by_position(board, from);
    let to_piece_option = try_get_piece_by_position(board, to);
    match (from_piece_option, to_piece_option) {
        (None, _) => false,
        (Some(_), None) => true,
        (Some(from_piece), Some(to_piece)) => from_piece.color != to_piece.color,
    }
}

pub fn get_blocked_positions(board: u64) -> Vec<u64> {
    let mut result = Vec::new();
    let pieces = get_pieces(board);
    for piece in pieces {
        if piece.in_pocket != Some(1) && piece.color == Some(get_move_side(board)) {
            result.push(piece.position);
        }
    }
    result
}

fn apply_offsets(offsets: &Vec<(i8, i8)>, position: u64) -> Vec<u64> {
    let mut result = Vec::with_capacity(offsets.capacity());
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

pub fn generate_new_wazir_positions(board: u64, position: u64) -> Vec<u64> {
    let offsets: Vec<(i8, i8)> = vec![(-1, -1), (-1, 1), (1, -1), (1, 1)];
    let mut result = apply_offsets(&offsets, position);
    result.retain(|x| is_available_to_move(board, position, *x));
    result
}

pub fn generate_new_ferz_positions(board: u64, position: u64) -> Vec<u64> {
    let offsets: Vec<(i8, i8)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut result = apply_offsets(&offsets, position);
    result.retain(|x| is_available_to_move(board, position, *x));
    result
}

fn make_horse_offset_derivations(offset: (i8, i8)) -> [(i8, i8); 2] {
    let (x0, y0) = (offset.0 * 2, offset.1 * 2);
    let lam1 = |x| i8::from(x == 0);
    let lam2 = |x| i8::from(x == 0).neg();
    let o1 = (lam1(x0), lam1(y0));
    let o2 = (lam2(x0), lam2(y0));
    [o1, o2]
}

pub fn generate_new_horse_positions(board: u64, position: u64) -> Vec<u64> {
    let pre_offsets = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut offsets = Vec::with_capacity(8);
    for (i, pos) in apply_offsets(&pre_offsets, position)
        .into_iter()
        .enumerate()
    {
        if try_get_piece_by_position(board, pos).is_some() {
            continue;
        }
        for i in make_horse_offset_derivations(pre_offsets[i]) {
            offsets.push(i);
        }
    }
    let mut result = apply_offsets(&offsets, position);
    result.retain(|x| is_available_to_move(board, position, *x));
    result
}

pub fn generate_new_pawn_positions(board: u64, position: u64) -> Vec<u64> {
    let mut result = Vec::with_capacity(3);
    let piece = try_get_piece_by_position(board, position).unwrap();
    if let Some(c) = piece.color {
        let move_direction = -1 + 2 * c as i8;
        for i in apply_offsets(&vec![(-1, move_direction), (1, move_direction)], position) {
            if try_get_piece_by_position(board, i).is_some_and(|p| p.color != piece.color) {
                result.push(i);
            }
        }

        let forward_pos = apply_offsets(&vec![(0, move_direction)], position)[0];
        if try_get_piece_by_position(board, forward_pos).is_none() {
            result.push(forward_pos);
        }
    }
    result
}

pub fn collect_generated_positions(board: u64) -> Vec<u64> {
    let mut result = Vec::new();
    for i in get_pieces(board) {
        todo!()
    }
    result
}

fn visualizer(board: u64) -> String {
    todo!()
}

fn main() {}
