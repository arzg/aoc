use aoc::geology::{Map, Offset};
use std::str::FromStr;

const INPUT: &str = include_str!("geology_input");

fn main() {
    let map = Map::from_str(INPUT).unwrap();
    println!("{}", map.num_trees_in_path(Offset { right: 3, down: 1 }))
}
