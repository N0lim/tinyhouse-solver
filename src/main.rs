use std::todo;

use pastey::paste;

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

#[derive(Specifier, PartialEq, Eq, PartialOrd, Ord)]
pub enum PieceType {
    Wazir = 0,
    Horse = 1,
    Ferz = 2,
    Pawn = 3,
}

// white = 0|False, black = 1|True
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

fn generate_moves(board: Board) -> Board {
    todo!()
}

fn visualizer(board: Board) -> String {
    todo!()
}
