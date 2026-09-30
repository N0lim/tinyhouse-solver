use crate::helpers::*;

use core::panic;

use std::{
    collections::btree_map::Range,
    fmt::DebugTuple,
    iter::{Enumerate, Map},
    print, todo,
};

use pastey::paste;

use modular_bitfield::{Specifier, bitfield, prelude::*, specifiers::B4};

use num_enum::TryFromPrimitive;

pub mod helpers;

#[derive(Specifier, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, TryFromPrimitive)]
#[repr(u8)]
enum PieceType {
    Wazir = 0,
    Horse = 1,
    Ferz = 2,
    Pawn = 3,
}

/*
position encoding

0000 0001 0010 0011
0100 0101 0110 0111
1000 1001 1010 1011
1100 1101 1110 1111
*/

// white = 0|False, black = 1|True
#[bitfield]
#[repr(u64)]
#[derive(Clone, Copy)]
struct Board {
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
    repeated: bool,
    #[skip]
    __: B2,
}

#[derive(Clone, Copy)]
struct ChessPiece {
    position: u8,
    color: Option<bool>,
    in_pocket: Option<bool>,
    pawn_type: Option<PieceType>,
}

fn sort_board(board: Board) -> Board {
    sort_pawns(sort_ferzes(sort_horses(sort_wazirs(board))))
}

macro_rules! swap_piece_fields {
    ($board:ident, $piece:ident, $field:ident) => {
        paste! {
            let [<$field 1>] = $board.[<$piece _ $field 1>]();
            let [<$field 2>] = $board.[<$piece _ $field 2>]();
            $board.[<set_ $piece _ $field 1>]([<$field 2>]);
            $board.[<set_ $piece _ $field 2>]([<$field 1>]);
        }
    };
}

macro_rules! define_sort {
    ($fn_name:ident, $piece:ident $(, $field:ident)+ $(,)?) => {
        paste! {
            fn $fn_name(mut board: Board) -> Board {
                let key1 = ( $( board.[<$piece _ $field 1>](), )+ );
                let key2 = ( $( board.[<$piece _ $field 2>](), )+ );

                if key1 > key2 {
                    $(swap_piece_fields!(board, $piece, $field);)+
                }

                board
            }
        }
    };
}

define_sort!(sort_wazirs, wazir, color, in_pocket, pos);
define_sort!(sort_horses, horse, color, in_pocket, pos);
define_sort!(sort_ferzes, ferz, color, in_pocket, pos);
define_sort!(sort_pawns, pawn, color, in_pocket, pos, type);

fn main() {
    print!("{:#08b} ", set_bits(0, 32, 32, 1));
}

fn generate_blocked_positions(board: Board) -> Vec<u8> {
    let mut blocked_positions = Vec::with_capacity(10);
    let num: u64 = board.into();
    for (b, i) in (40..48).zip(0u64..) {
        if (((num >> b) & 1) == 1) == board.move_side() {
            let pos = (num >> (i * 4)) & 0b1111;
            blocked_positions.push(pos as u8);
        }
    }
    blocked_positions
}

fn generate_moves(board: Board) -> Board {
    let blocked_positions = generate_blocked_positions(board);
    todo!()
}

fn generate_wazir_moves(board: Board) {
    let offsets = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
    if board.wazir_color1() == board.move_side() {
        let x = (board.wazir_pos1() % 4) as i8;
        let y = (board.wazir_pos1() / 4) as i8;
    }
    todo!()
}

// note this function do not change who currently move, but checks who move now in calculation
fn capture_piece(board: Board, position: u8) -> Board {
    let q = get_quadruples(board);
    let mut num: u64 = board.into();
    for (i, piece) in q.iter().enumerate().skip(2) {
        if piece.position == position
            && piece.in_pocket == Some(false)
            && piece.color != Some(board.move_side())
        {
            let pos_shift = 4 * i;
            let color_shift = 40 + (i - 2);
            let pocket_shift = 48 + (i - 2);

            num &= !(0b1111_u64 << pos_shift); // nullify position
            num ^= 1 << color_shift; // swap color and pocket flag of piece to other
            num ^= 1 << pocket_shift;

            if piece.pawn_type.is_some_and(|x| x != PieceType::Pawn) {
                let type_shift = 56 + 2 * (i - 8);
                num &= !(0b11 << type_shift); // set promoted pawn to pawn again after capture
                num |= 0b11 << type_shift;
            }
        }
    }
    Board::from(num)
}

fn get_quadruples(board: Board) -> [ChessPiece; 10] {
    let mut arr = [ChessPiece {
        position: 0,
        color: None,
        in_pocket: None,
        pawn_type: None,
    }; 10];
    let num: u64 = board.into();

    for (i, piece) in arr.iter_mut().enumerate() {
        let pos_shift = 4 * i;
        let new_pos = ((num >> pos_shift) & 0b1111) as u8;
        piece.position = new_pos;

        if i == 0 {
            piece.color = Some(false);
        }

        if i == 1 {
            piece.color = Some(true);
        }

        if i >= 2 {
            let color_shift = 40 + (i - 2);
            let pocket_shift = 48 + (i - 2);
            let new_color = ((num >> color_shift) & 1) == 1;
            let new_pocket = ((num >> pocket_shift) & 1) == 1;
            piece.color = Some(new_color);
            piece.in_pocket = Some(new_pocket);
        }

        if i >= 8 {
            let type_shift = 56 + 2 * (i - 8);
            let new_type = ((num >> type_shift) & 0b11) as u8;
            piece.pawn_type = Some(new_type.try_into().unwrap());
        }
    }

    arr
}

fn visualizer(board: Board) -> String {
    todo!()
}
