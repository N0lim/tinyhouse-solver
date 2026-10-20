use crate::helpers::*;
use std::ops::Neg;

pub fn generate_wazir_positions(position: u64) -> impl Iterator<Item = u64> {
    let offsets: [(i8, i8); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    apply_offsets(offsets, position)
}

pub fn generate_ferz_positions(position: u64) -> impl Iterator<Item = u64> {
    let offsets: [(i8, i8); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
    apply_offsets(offsets, position)
}

fn make_hors_offset_derivations(offset: (i8, i8)) -> [(i8, i8); 2] {
    let (x0, y0) = (offset.0 * 2, offset.1 * 2);
    let lam1 = |x| i8::from(x == 0);
    let lam2 = |x| i8::from(x == 0).neg();
    let o1 = (lam1(x0), lam1(y0));
    let o2 = (lam2(x0), lam2(y0));
    [o1, o2]
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

pub fn extend_by_generated_positions_non_kings<E: Extend<(usize, u64)>>(
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

    // I'd prefer lambda function with index and friendly capture, but don't want to use Box and can't use nested impl for different iterators
    fn helper(
        index: usize,
        friendly: &[u64],
        iter: impl Iterator<Item = u64>,
    ) -> impl Iterator<Item = (usize, u64)> {
        iter.filter(|x| !friendly.contains(x))
            .map(move |x| (index, x))
    }

    for (i, &piece) in pieces
        .iter()
        .enumerate()
        .filter(|(_, piece)| is_on_board(**piece) && piece.color == move_side)
    {
        let position = piece.position;
        if i == 2 || i == 3 {
            ext.extend(helper(
                i,
                friendly.as_slice(),
                generate_wazir_positions(position),
            ));
        }
        if i == 4 || i == 5 {
            ext.extend(helper(
                i,
                friendly.as_slice(),
                generate_hors_positions(position, occupied.as_slice()),
            ));
        }
        if i == 6 || i == 7 {
            ext.extend(helper(
                i,
                friendly.as_slice(),
                generate_ferz_positions(position),
            ));
        }
        if i == 8 || i == 9 {
            let pawn_type = piece.pawn_type.unwrap();
            if pawn_type == 0 {
                ext.extend(helper(
                    i,
                    friendly.as_slice(),
                    generate_wazir_positions(position),
                ));
            }
            if pawn_type == 1 {
                ext.extend(helper(
                    i,
                    friendly.as_slice(),
                    generate_hors_positions(position, occupied.as_slice()),
                ));
            }
            if pawn_type == 2 {
                ext.extend(helper(
                    i,
                    friendly.as_slice(),
                    generate_ferz_positions(position),
                ));
            }
            if pawn_type == 3 {
                ext.extend(helper(
                    i,
                    friendly.as_slice(),
                    generate_sideways_pawn_positions(position, enemy.as_slice(), piece.color),
                ));
                ext.extend(helper(
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

    for (i, _) in pieces
        .iter()
        .enumerate()
        .filter(|(_, piece)| !is_on_board(**piece) && piece.color == move_side)
    {
        if i < 8 {
            ext.extend(helper(i, friendly.as_slice(), 0..16));
        }
        if i == 8 || i == 9 {
            ext.extend(helper(i, friendly.as_slice(), 4..12));
        }
    }
}

pub fn collect_generated_positions(board: u64) -> Vec<u64> {
    let result = Vec::new();

    todo!();
    result
}
