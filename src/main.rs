use std::todo;
use tinyhouse_solver::generators::*;
use tinyhouse_solver::helpers::*;
use tinyhouse_solver::visulization::*;

// notice this function do not check if there is multiple pieces in one position
pub fn try_capture_piece(mut board: u64, position: u64) -> u64 {
    let q = get_pieces(board);
    for (i, &piece) in (0..10u64).zip(q.iter()) {
        if piece.position == position && is_on_board(piece) {
            assert!(
                i >= 2,
                "can't capture kings, board: {}, position: {}",
                board,
                position
            );

            board = set_position(board, i, 0);
            board = set_color(board, i, get_color(board, i) ^ 1);
            board = set_in_pocket(board, i, 1);

            if piece.pawn_type.is_some() {
                board = set_pawn_type(board, i, 0b11);
            }
        }
    }
    board
}

pub fn move_piece(board: u64, index: u64, position: u64) -> u64 {
    let board = try_capture_piece(board, position);
    let board = set_position(board, index, position);
    let board = set_move_side(board, get_move_side(board) ^ 1);
    sort_board(board)
}

pub fn generate_derivative_positions(board: u64) -> impl Iterator<Item = u64> {
    let mut vec = Vec::new();
    extend_by_generated_positions_non_kings_on_board(&mut vec, board, get_move_side(board));
    vec.into_iter()
        .map(move |(index, pos)| move_piece(board, index as u64, pos))
}

fn main() {
    println!("{}\n", init_table(STARTING_POSITION));
    for i in generate_derivative_positions(STARTING_POSITION) {
        println!("\n{}", init_table(i));
        let pieces = get_pieces(i);
        println!("{:?}", pieces);
    }
}
