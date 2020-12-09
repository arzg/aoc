use aoc::xmas::Input;

const INPUT: &str = include_str!("xmas_input");

fn main() {
    let input = Input::new(INPUT);

    println!("Smallest invalid number: {}", input.first_invalid_number());
    println!("Encryption weakness: {}", input.determine_weakness());
}
