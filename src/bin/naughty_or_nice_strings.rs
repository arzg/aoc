use aoc::naughty_or_nice_strings::Judgement;

const INPUT: &str = include_str!("naughty_or_nice_strings_input");

fn main() {
    let num_nice_lines = INPUT
        .lines()
        .filter(|line| Judgement::of(line) == Judgement::Nice)
        .count();

    println!("{}", num_nice_lines);
}
