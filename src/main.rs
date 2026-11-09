use std::todo;
use tinyhouse_solver::generators::*;
use tinyhouse_solver::helpers::*;
use tinyhouse_solver::visulization::*;

fn main() {
    println!("{}\n", init_table(STARTING_POSITION));
    for i in generate_derivative_moves(STARTING_POSITION) {
        println!("\n{}", init_table(i));
        let pieces = get_pieces(i);
        println!("{:?}", pieces);
    }
}
