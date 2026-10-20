pub mod generators;
pub mod helpers;

use crate::generators::*;
use crate::helpers::*;
use comfy_table::Cell;
use comfy_table::Color;
use comfy_table::Table;
use comfy_table::presets::NOTHING;
use core::panic;
use proptest::bits::usize;
use std::array::from_fn;
use std::mem;
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

fn board_to_symbols(board: u64) -> [[Option<char>; 9]; 4] {
    let mut symbols = [[None; 9]; 4];
    let mut pocket0: Vec<char> = Vec::with_capacity(8);
    let mut pocket1: Vec<char> = Vec::with_capacity(8);
    for (index, &piece) in get_pieces(board).iter().enumerate() {
        let (x, y) = position_as_offset(piece.position);
        let (x, y) = (x as usize, y as usize);
        let symbol = visualize_piece(index as u64, piece);
        if is_on_board(piece) {
            symbols[y][x] = Some(symbol);
        } else if piece.color == 0 {
            pocket0.push(symbol);
        } else {
            pocket1.push(symbol);
        }
    }
    for (index, &symbol) in pocket1.iter().enumerate() {
        symbols[(index % 4) + 6][index / 4] = Some(symbol);
    }
    for (index, &symbol) in pocket1.iter().enumerate() {
        symbols[(index % 4) + 6][(index / 4) + 2] = Some(symbol);
    }
    symbols
}

fn init_empty_cells(board: u64) -> [[Cell; 9]; 4] {
    let symbols = board_to_symbols(board);
    std::array::from_fn(|y| {
        std::array::from_fn(|x| {
            let mut cell = Cell::new("");
            if let Some(symbol) = symbols[y][x] {
                cell = Cell::new(symbol)
            }
            if x < 4 {
                let color = if ((x ^ y) & 1) == 0 {
                    Color::White
                } else {
                    Color::Black
                };
                cell = cell.bg(color);
            }
            if x > 4 && y < 2 {
                cell = cell.bg(Color::White)
            }
            if x > 4 && y >= 2 {
                cell = cell.bg(Color::Black)
            }
            cell
        })
    })
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
    table.add_rows(init_empty_cells(STARTING_POSITION));
    print!("{table}")
}
