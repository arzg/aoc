use aoc::presents::Present;
use std::str::FromStr;

const INPUT: &str = include_str!("presents_input");

fn main() {
    let presents = INPUT
        .lines()
        .map(|line| Present::from_str(line.trim()).unwrap());

    let total_wrapping_paper = presents
        .map(|present| present.wrapping_paper_area())
        .sum::<u32>();

    println!("{}", total_wrapping_paper);
}
