use std::str::FromStr;

const INPUT: &str = include_str!("expense_report_input");

fn main() {
    let expenses: Vec<_> = INPUT
        .lines()
        .map(|line| u32::from_str(line).unwrap())
        .collect();

    for x in &expenses {
        for y in &expenses {
            if x + y == 2020 {
                println!("{}", x * y);
                return;
            }
        }
    }
}
