use aoc::presents::Present;
use std::str::FromStr;

const INPUT: &str = include_str!("presents_input");

fn main() {
    let presents = INPUT
        .lines()
        .map(|line| Present::from_str(line.trim()).unwrap());

    let total_wrapping_paper: u32 = presents
        .clone()
        .map(|present| present.wrapping_paper_area())
        .sum();

    let total_ribbon_length: u32 = presents.map(|present| present.ribbon_length()).sum();

    println!("{}", total_wrapping_paper);
    println!("{}", total_ribbon_length);
}
