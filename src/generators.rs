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

pub fn for_each_non_king_position_on_board(board: u64, move_side: u64, mut f: impl FnMut(u64)) {
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
        match i {
            0 | 1 => {}
            2 | 3 => for_each_generated_wazir_position(position, &mut f),
            4 | 5 => for_each_generated_hors_position(position, occupied.as_slice(), &mut f),
            6 | 7 => for_each_generated_ferz_position(position, &mut f),
            8 | 9 => match piece.pawn_type.unwrap() {
                0 => for_each_generated_wazir_position(position, &mut f),
                1 => for_each_generated_hors_position(position, occupied.as_slice(), &mut f),
                2 => for_each_generated_ferz_position(position, &mut f),
                3 => {
                    for_each_generated_forward_pawn_position(
                        position,
                        occupied.as_slice(),
                        piece.color,
                        &mut f,
                    );
                    for_each_generated_diagonal_pawn_position(
                        position,
                        enemy.as_slice(),
                        piece.color,
                        &mut f,
                    );
                }
                _ => panic!("wrong pawn type"),
            },
            _ => panic!("index bigger than 9"),
        }
    }
}

pub fn for_each_position_in_pocket(board: u64, move_side: u64, mut f: impl FnMut(u64)) {
    let pieces = get_pieces(board);
    for (i, _) in pieces
        .iter()
        .enumerate()
        .filter(|(_, piece)| !is_on_board(**piece) && piece.color == move_side)
    {
        if i < 8 {
            (0..16).into_iter().for_each(&mut f);
        }
        if i == 8 || i == 9 {
            (4..12).into_iter().for_each(&mut f);
        }
    }
}

pub fn for_each_position(board: u64, move_side: u64, mut f: impl FnMut(u64)) {
    let mut blocked: Vec<u64> = Vec::with_capacity(MAXIMUM_GENERATED_POSITIONS);
    for_each_non_king_position_on_board(board, move_side ^ 1, |x| blocked.push(x));
    let king_pos = get_position(board, move_side);
    for_each_generated_king_position(king_pos, &blocked, &mut f);
    for_each_non_king_position_on_board(board, move_side, &mut f);
    for_each_position_in_pocket(board, move_side, &mut f);
}

pub fn generate_wazir_positions(position: u64) -> impl Iterator<Item = u64> {
    let offsets: [(i8, i8); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    apply_offsets(offsets, position)
}

pub fn generate_ferz_positions(position: u64) -> impl Iterator<Item = u64> {
    let offsets: [(i8, i8); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
    apply_offsets(offsets, position)
}

pub fn generate_hors_positions(
    position: u64,
    occupied_positions: &[u64],
) -> impl Iterator<Item = u64> {
    const PRE_OFFSETS: [(i8, i8); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    apply_offsets(PRE_OFFSETS, position)
        .enumerate()
        .filter(|(_, pos)| !occupied_positions.contains(pos))
        .flat_map(|(i, _)| make_hors_offset_derivations(PRE_OFFSETS[i]))
        .filter_map(move |offset| apply_offset(offset, position))
}

pub fn generate_forward_pawn_positions(
    position: u64,
    occupied_positions: &[u64],
    color: u64,
) -> impl Iterator<Item = u64> {
    let move_direction = -1 + 2 * color as i8;
    let offsets: [(i8, i8); 1] = [(0, move_direction)];
    apply_offsets(offsets, position).filter(|pos| !occupied_positions.contains(pos))
}

pub fn generate_sideways_pawn_positions(
    position: u64,
    enemy_positions: &[u64],
    color: u64,
) -> impl Iterator<Item = u64> {
    let move_direction = -1 + 2 * color as i8;
    let offsets: [(i8, i8); 2] = [(-1, move_direction), (1, move_direction)];
    apply_offsets(offsets, position).filter(|pos| enemy_positions.contains(pos))
}

pub fn generate_king_positions(
    position: u64,
    blocked_positions: &[u64],
) -> impl Iterator<Item = u64> {
    #[rustfmt::skip]
    let offsets: [(i8, i8); 8] = [
        (-1, -1), (-1, 0), (-1, 1),
        ( 0, -1),          ( 0, 1),
        ( 1, -1), ( 1, 0), ( 1, 1),
    ];
    apply_offsets(offsets, position).filter(|pos| !blocked_positions.contains(pos))
}

fn collect_pos_by(predicate: impl Fn(&ChessPiece) -> bool, pieces: [ChessPiece; 10]) -> Vec<u64> {
    pieces
        .iter()
        .filter(|piece| predicate(piece))
        .map(|&piece| piece.position)
        .collect()
}

// I'd prefer lambda function with index and friendly capture, but don't want to use Box and can't use nested impl for different iterators
fn filter_friendly(
    index: usize,
    friendly: &[u64],
    iter: impl Iterator<Item = u64>,
) -> impl Iterator<Item = (usize, u64)> {
    iter.filter(|x| !friendly.contains(x))
        .map(move |x| (index, x))
}

pub fn extend_by_generated_positions_non_kings_on_board<E: Extend<(usize, u64)>>(
    ext: &mut E,
    board: u64,
    move_side: u64,
) {
    let pieces = get_pieces(board);
    let occupied = collect_pos_by(|&piece| is_on_board(piece), pieces);
    let friendly = collect_pos_by(
        |&piece| is_on_board(piece) && piece.color == move_side,
        pieces,
    );
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
        if i == 2 || i == 3 {
            ext.extend(filter_friendly(
                i,
                friendly.as_slice(),
                generate_wazir_positions(position),
            ));
        }
        if i == 4 || i == 5 {
            ext.extend(filter_friendly(
                i,
                friendly.as_slice(),
                generate_hors_positions(position, occupied.as_slice()),
            ));
        }
        if i == 6 || i == 7 {
            ext.extend(filter_friendly(
                i,
                friendly.as_slice(),
                generate_ferz_positions(position),
            ));
        }
        if i == 8 || i == 9 {
            let pawn_type = piece.pawn_type.unwrap();
            if pawn_type == 0 {
                ext.extend(filter_friendly(
                    i,
                    friendly.as_slice(),
                    generate_wazir_positions(position),
                ));
            }
            if pawn_type == 1 {
                ext.extend(filter_friendly(
                    i,
                    friendly.as_slice(),
                    generate_hors_positions(position, occupied.as_slice()),
                ));
            }
            if pawn_type == 2 {
                ext.extend(filter_friendly(
                    i,
                    friendly.as_slice(),
                    generate_ferz_positions(position),
                ));
            }
            if pawn_type == 3 {
                ext.extend(filter_friendly(
                    i,
                    friendly.as_slice(),
                    generate_sideways_pawn_positions(position, enemy.as_slice(), piece.color),
                ));
                ext.extend(filter_friendly(
                    i,
                    friendly.as_slice(),
                    generate_forward_pawn_positions(position, occupied.as_slice(), piece.color),
                ));
            }
            if pawn_type >= 4 {
                panic!("wrong pawn type")
            }
        }
    }
}

pub fn extend_by_generated_positions_in_pocket<E: Extend<(usize, u64)>>(
    ext: &mut E,
    board: u64,
    move_side: u64,
) {
    let pieces = get_pieces(board);
    let friendly = collect_pos_by(
        |&piece| is_on_board(piece) && piece.color == move_side,
        pieces,
    );
    for (i, _) in pieces
        .iter()
        .enumerate()
        .filter(|(_, piece)| !is_on_board(**piece) && piece.color == move_side)
    {
        if i < 8 {
            ext.extend(filter_friendly(i, friendly.as_slice(), 0..16));
        }
        if i == 8 || i == 9 {
            ext.extend(filter_friendly(i, friendly.as_slice(), 4..12));
        }
    }
}

pub fn extend_by_king<E: Extend<(usize, u64)>>(ext: &mut E, board: u64, move_side: u64) {
    let pieces = get_pieces(board);
    let friendly = collect_pos_by(
        |&piece| is_on_board(piece) && piece.color == move_side,
        pieces,
    );
    let mut enemy_move_positions = Vec::new();
    extend_by_generated_positions_non_kings_on_board(
        &mut enemy_move_positions,
        board,
        move_side ^ 1,
    );
    let blocked: Vec<u64> = enemy_move_positions
        .into_iter()
        .map(|(_, pos)| pos)
        .chain(friendly)
        .chain(generate_king_positions(
            get_position(board, move_side ^ 1),
            &[],
        ))
        .collect();
    let generated = generate_king_positions(move_side, blocked.as_slice());

    ext.extend(((move_side as usize)..).zip(generated));
}

pub fn extend_by_generated_positions<E: Extend<(usize, u64)>>(
    ext: &mut E,
    board: u64,
    move_side: u64,
) {
    extend_by_generated_positions_non_kings_on_board(ext, board, move_side);
    extend_by_king(ext, board, move_side);
    extend_by_generated_positions_in_pocket(ext, board, move_side);
}
