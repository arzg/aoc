use aoc::xmas::Input;

const INPUT: &str = include_str!("xmas_input");

fn main() {
    let input = Input::new(INPUT);
    println!("{}", input.first_invalid_number());
}
