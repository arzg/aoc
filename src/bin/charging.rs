use aoc::charging::{Adapters, Difference};
use std::str::FromStr;

const INPUT: &str = include_str!("charging_input");

fn main() {
    let mut adapters = Adapters::from_str(INPUT).unwrap();
    let differences = adapters.differences_to_outlet();

    let num_one_jolt_differences = differences
        .iter()
        .filter(|diff| **diff == Difference::One)
        .count();

    let num_three_jolt_differences = differences
        .iter()
        .filter(|diff| **diff == Difference::Three)
        .count();

    println!("{}", num_one_jolt_differences * num_three_jolt_differences);
}
