use aoc::password::Password;
use std::str::FromStr;

const INPUT: &str = include_str!("password_input");

fn main() {
    let passwords = INPUT.lines().map(|line| Password::from_str(line).unwrap());

    let num_valid = passwords.filter(Password::is_valid).count();

    println!("{}", num_valid);
}
