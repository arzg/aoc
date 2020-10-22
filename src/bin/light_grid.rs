use aoc::light_grid::{Grid, Instruction};
use std::str::FromStr;

const INPUT: &str = include_str!("light_grid_input");

fn main() {
    let mut grid = Grid::default();

    for instruction in INPUT.lines().map(Instruction::from_str) {
        grid.apply(instruction.unwrap());
    }

    println!("{}", grid.lit_lights())
}
