use aoc::light_grid::{BinaryLight, Grid, Instruction, Light, ScalarLight};
use std::str::FromStr;

const INPUT: &str = include_str!("light_grid_input");

fn main() {
    calculate_total_brightness::<BinaryLight>("With binary lights");
    calculate_total_brightness::<ScalarLight>("With scalar lights");
}

fn calculate_total_brightness<L: Light>(msg: &str) {
    let mut grid: Grid<L> = Grid::default();

    for instruction in INPUT.lines().map(Instruction::from_str) {
        grid.apply(instruction.unwrap());
    }

    println!("{}: {}", msg, grid.total_brightness());
}
