use aoc::expense_report;
use std::str::FromStr;

const INPUT: &str = include_str!("expense_report_input");

fn main() {
    let expenses: Vec<_> = INPUT
        .lines()
        .map(|line| u32::from_str(line).unwrap())
        .collect();

    let format_solution = |s: Option<u32>| s.map_or("No solution".to_string(), |n| n.to_string());

    println!(
        "{}",
        format_solution(expense_report::two_add_to_2020(&expenses)),
    );

    println!(
        "{}",
        format_solution(expense_report::three_add_to_2020(&expenses)),
    );
}
