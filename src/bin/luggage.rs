use aoc::luggage::{Bag, Rules};
use std::str::FromStr;

const INPUT: &str = include_str!("luggage_input");

fn main() {
    let bag = Bag {
        modifier: "shiny".to_string(),
        color: "gold".to_string(),
    };

    let rules = Rules::from_str(INPUT).unwrap();

    println!(
        "Number of bags that can contain a shiny gold bag: {}",
        rules.num_bags_that_can_contain(&bag),
    );

    println!(
        "Number of bags that are contained by a shiny gold bag: {}",
        rules.num_bags_contained_by(&bag),
    );
}
