use aoc::password::{Password, Ruleset};
use std::str::FromStr;

const INPUT: &str = include_str!("password_input");

fn main() {
    let passwords = INPUT.lines().map(|line| Password::from_str(line).unwrap());

    let num_valid_with_old_ruleset = passwords
        .clone()
        .filter(|password| password.is_valid(Ruleset::Old))
        .count();

    let num_valid_with_new_ruleset = passwords
        .filter(|password| password.is_valid(Ruleset::New))
        .count();

    println!(
        "{} passwords are valid with the old ruleset",
        num_valid_with_old_ruleset
    );
    println!(
        "{} passwords are valid with the new ruleset",
        num_valid_with_new_ruleset
    );
}
