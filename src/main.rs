mod parsing;
mod days {
    pub mod d1;
    pub mod d2;
    pub mod d3;
    pub mod d4;
}
mod graph;

use crate::d2::*;
use crate::d3::*;
use crate::d4::*;
use crate::days::*;
use crate::graph::*;
use crate::parsing::*;

fn test_graph() {
    let graph = Graph::new(42);
}

fn main() {
    let input = get_input_as_vec("./inputs/d4.txt");

    let mut inputvec = match input {
        Ok(result) => result,
        Err(err) => panic!("Error getting input: {}", err),
    };

    test_graph();

    println!("Result: {}", d4p2(&inputvec));
    return ();
}
