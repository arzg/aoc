use aoc::naughty_or_nice_strings::{Judgement, Ruleset};

const INPUT: &str = include_str!("naughty_or_nice_strings_input");

fn main() {
    println!(
        "With old ruleset: {}",
        num_nice_lines(INPUT.trim().lines(), Ruleset::Old),
    );

    println!(
        "With new ruleset: {}",
        num_nice_lines(INPUT.trim().lines(), Ruleset::New),
    );
}

fn num_nice_lines<'a>(lines: impl Iterator<Item = &'a str>, ruleset: Ruleset) -> usize {
    lines
        .filter(|line| Judgement::of(line, ruleset) == Judgement::Nice)
        .count()
}
