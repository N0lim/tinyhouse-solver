pub mod generators;
pub mod helpers;

use crate::generators::*;
use crate::helpers::*;
use comfy_table::Table;
use comfy_table::presets::NOTHING;
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
0b01 = Hors
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

fn visualize_king(color: u64) -> char {
    match color {
        0 => '♔',
        1 => '♚',
        _ => panic!("wrong color"),
    }
}

fn visualize_wazir(color: u64) -> char {
    match color {
        0 => '🨠',
        1 => '🨦',
        _ => panic!("wrong color"),
    }
}

fn visualize_hors(color: u64) -> char {
    match color {
        0 => '🩣',
        1 => '🩪',
        _ => panic!("wrong color"),
    }
}

fn visualize_ferz(color: u64) -> char {
    match color {
        0 => '🨡',
        1 => '🨧',
        _ => panic!("wrong color"),
    }
}

fn visualize_pawn(color: u64) -> char {
    match color {
        0 => '♙',
        1 => '♟',
        _ => panic!("wrong color"),
    }
}

fn visualize_piece(piece_index: u64, piece: ChessPiece) -> char {
    let color = piece.color;
    match piece_index {
        0 | 1 => visualize_king(color),
        2 | 3 => visualize_wazir(color),
        4 | 5 => visualize_hors(color),
        6 | 7 => visualize_ferz(color),
        8 | 9 if is_on_board(piece) => match piece.pawn_type.unwrap() {
            0 => visualize_wazir(color),
            1 => visualize_hors(color),
            2 => visualize_ferz(color),
            3 => visualize_pawn(color),
            _ => panic!("wrong pawn type"),
        },
        8 | 9 => visualize_pawn(color),
        _ => panic!("wrong piece index"),
    }
}

fn visualizer(board: u64) -> Table {
    let mut table = Table::new();
    table.load_style(NOTHING);
    let pieces = get_pieces(board);
    todo!()
}

fn main() {
    let mut table = Table::new();
    table.load_style(NOTHING);
    table.add_row(vec!["", "", "", "♔", "", "abcdefgh"]);
    table.add_row(vec![""]);
    table.add_row(vec![""]);
    table.add_row(vec!["♚"]);
    print!("{table}")
}
