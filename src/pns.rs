use crate::generators::*;
use crate::helpers::*;
use crate::pns::TerminalState::Lose;
use crate::pns::TerminalState::Win;
use identity_hash::IntMap;
use petgraph::prelude::*;

const S_WIN: u32 = 0;
const G_LOSE: u32 = 0;

/*
     G S
Win  x 0
Draw x x
Lose 0 x
*/

#[derive(Clone)]
pub struct BoardPosition {
    board: u64,
    g_win: u32,
    g_draw: u32,
    s_draw: u32,
    s_lose: u32,
}

impl BoardPosition {
    pub fn new(board: u64) -> Self {
        Self {
            board,
            g_win: 1,
            g_draw: 1,
            s_draw: 1,
            s_lose: 1,
        }
    }
}

type SolutionGraph = DiGraph<BoardPosition, ()>;
type SolutionTable = IntMap<u64, NodeIndex>;
//type SolutionStack = Vec<u64>;
pub struct SolutionTuple {
    graph: SolutionGraph,
    table: SolutionTable,
}

// win/lose/draw from perspective of white
pub enum TerminalState {
    Win,
    Draw,
    Lose,
}

pub fn try_check_terminal_state(board: u64) -> Option<TerminalState> {
    let move_side = get_move_side(board);
    assert!(move_side <= 1);
    let mut has_move = false;
    for_each_position(board, move_side, |_| has_move = true);
    if has_move {
        return None;
    }

    let mut is_king_attacked = false;
    for_each_position(board, move_side ^ 1, |(_, pos)| {
        if pos == get_position(board, move_side) {
            is_king_attacked = true
        }
    });

    Some(match (is_king_attacked, move_side) {
        (true, 0) => Lose,
        (true, 1) => Win,
        (false, 0) => Win,
        (false, 1) => Lose,
        _ => unreachable!(),
    })
}

pub fn baord_position_to_arrays(b: &BoardPosition) -> ([u32; 3], [u32; 3]) {
    ([b.g_win, b.g_draw, G_LOSE], [S_WIN, b.s_draw, b.s_lose])
}

pub fn update_pns_numbers(
    board_pos: &mut BoardPosition,
    iter: impl Iterator<Item = BoardPosition>,
) {
    for child in iter {
        if get_move_side(board_pos.board) == 0 {
            todo!()
        } else {
            todo!()
        }
    }
}

pub fn expand_node(index: NodeIndex, SolutionTuple { graph, table }: &mut SolutionTuple) {
    let board_pos = graph.node_weight(index).unwrap();
    let board = board_pos.board;
    for new_board in generate_derivative_moves(board) {
        let new_node = graph.add_node(BoardPosition::new(new_board));
        graph.add_edge(index, new_node, ());
        match table.get(&new_board) {
            Some(&existing) => {
                graph.add_edge(new_node, existing, ());
            }
            None => {
                table.insert(new_board, new_node);
            }
        };
    }
}
