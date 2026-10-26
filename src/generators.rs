use crate::helpers::*;
use std::ops::Neg;

pub fn for_each_generated_wazir_position(position: u64, mut f: impl FnMut(u64)) {
    let offsets: [(i8, i8); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    for x in apply_offsets(offsets, position) {
        f(x);
    }
}

pub fn for_each_generated_ferz_position(position: u64, mut f: impl FnMut(u64)) {
    let offsets: [(i8, i8); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
    for x in apply_offsets(offsets, position) {
        f(x);
    }
}

fn make_hors_offset_derivations(offset: (i8, i8)) -> [(i8, i8); 2] {
    let (x0, y0) = (offset.0 * 2, offset.1 * 2);
    let lam1 = |x| {
        if x == 0 { 1 } else { x }
    };
    let lam2 = |x| {
        if x == 0 { -1 } else { x }
    };
    let o1 = (lam1(x0), lam1(y0));
    let o2 = (lam2(x0), lam2(y0));
    [o1, o2]
}

pub fn for_each_generated_hors_position(
    position: u64,
    occupied_positions: &[u64],
    mut f: impl FnMut(u64),
) {
    const ADJACENT_OFFSETS: [(i8, i8); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    for offset in ADJACENT_OFFSETS {
        let adjacent = apply_offset(offset, position);
        if adjacent.is_none() || occupied_positions.contains(&adjacent.unwrap()) {
            continue;
        }
        for der in make_hors_offset_derivations(offset) {
            if let Some(x) = apply_offset(der, position) {
                f(x);
            }
        }
    }
}

pub fn for_each_generated_forward_pawn_position(
    position: u64,
    occupied_positions: &[u64],
    color: u64,
    mut f: impl FnMut(u64),
) {
    let move_direction = -1 + 2 * color as i8;
    let offsets: [(i8, i8); 1] = [(0, move_direction)];
    for pos in apply_offsets(offsets, position).filter(|pos| !occupied_positions.contains(pos)) {
        f(pos)
    }
}

pub fn for_each_generated_diagonal_pawn_position(
    position: u64,
    enemy_positions: &[u64],
    color: u64,
    mut f: impl FnMut(u64),
) {
    let move_direction = -1 + 2 * color as i8;
    let offsets: [(i8, i8); 2] = [(-1, move_direction), (1, move_direction)];
    for pos in apply_offsets(offsets, position).filter(|pos| enemy_positions.contains(pos)) {
        f(pos)
    }
}

pub fn for_each_generated_king_position(
    position: u64,
    blocked_positions: &[u64],
    mut f: impl FnMut(u64),
) {
    #[rustfmt::skip]
    let offsets: [(i8, i8); 8] = [
        (-1, -1), (-1, 0), (-1, 1),
        ( 0, -1),          ( 0, 1),
        ( 1, -1), ( 1, 0), ( 1, 1),
    ];
    for pos in apply_offsets(offsets, position).filter(|pos| !blocked_positions.contains(pos)) {
        f(pos)
    }
}

fn collect_pos_by(predicate: impl Fn(&ChessPiece) -> bool, pieces: [ChessPiece; 10]) -> Vec<u64> {
    pieces
        .iter()
        .filter(|piece| predicate(piece))
        .map(|&piece| piece.position)
        .collect()
}

pub fn for_each_non_king_position_on_board(
    board: u64,
    move_side: u64,
    mut f: impl FnMut((usize, u64)),
) {
    let pieces = get_pieces(board);
    let occupied = collect_pos_by(|&piece| is_on_board(piece), pieces);
    let enemy = collect_pos_by(
        |&piece| is_on_board(piece) && piece.color != move_side,
        pieces,
    );

    for (i, &piece) in pieces
        .iter()
        .enumerate()
        .filter(|(_, piece)| is_on_board(**piece) && piece.color == move_side)
    {
        let position = piece.position;
        let mut i_f = |pos| f((i, pos));
        match i {
            0 | 1 => {}
            2 | 3 => for_each_generated_wazir_position(position, i_f),
            4 | 5 => for_each_generated_hors_position(position, occupied.as_slice(), i_f),
            6 | 7 => for_each_generated_ferz_position(position, i_f),
            8 | 9 => match piece.pawn_type.unwrap() {
                0 => for_each_generated_wazir_position(position, i_f),
                1 => for_each_generated_hors_position(position, occupied.as_slice(), i_f),
                2 => for_each_generated_ferz_position(position, i_f),
                3 => {
                    for_each_generated_forward_pawn_position(
                        position,
                        occupied.as_slice(),
                        piece.color,
                        &mut i_f,
                    );
                    for_each_generated_diagonal_pawn_position(
                        position,
                        enemy.as_slice(),
                        piece.color,
                        &mut i_f,
                    );
                }
                _ => panic!("wrong pawn type"),
            },
            _ => panic!("index bigger than 9"),
        }
    }
}

pub fn for_each_position_in_pocket(board: u64, move_side: u64, mut f: impl FnMut((usize, u64))) {
    let pieces = get_pieces(board);
    for (i, _) in pieces
        .iter()
        .enumerate()
        .filter(|(_, piece)| !is_on_board(**piece) && piece.color == move_side)
    {
        let mut i_f = |pos| f((i, pos));
        if i < 8 {
            (0..16).into_iter().for_each(&mut i_f);
        }
        if i == 8 || i == 9 {
            (4..12).into_iter().for_each(i_f);
        }
    }
}

pub fn for_each_position(board: u64, move_side: u64, mut f: impl FnMut((usize, u64))) {
    let mut blocked: Vec<u64> = Vec::with_capacity(MAXIMUM_GENERATED_POSITIONS);
    for_each_non_king_position_on_board(board, move_side ^ 1, |(_, pos)| blocked.push(pos));
    let king_pos = get_position(board, move_side);
    for_each_generated_king_position(king_pos, &blocked, |pos| f((move_side as usize, pos)));
    for_each_non_king_position_on_board(board, move_side, &mut f);
    for_each_position_in_pocket(board, move_side, &mut f);
}
