use aoc::luggage::{Bag, Rules};
use std::str::FromStr;

const INPUT: &str = include_str!("luggage_input");

fn main() {
    let rules = Rules::from_str(INPUT).unwrap();

    println!(
        "{}",
        rules.num_bags_that_can_contain(Bag {
            modifier: "shiny".to_string(),
            color: "gold".to_string()
        })
    );
}
