use std::str::FromStr;

const INPUT: &str = include_str!("expense_report_input");

fn main() {
    let expenses: Vec<_> = INPUT
        .lines()
        .map(|line| u32::from_str(line).unwrap())
        .collect();

    println!(
        "{}",
        two_add_to_2020(&expenses).map_or("No solution".to_string(), |n| n.to_string()),
    );

    println!(
        "{}",
        three_add_to_2020(&expenses).map_or("No solution".to_string(), |n| n.to_string()),
    );
}

fn two_add_to_2020(expenses: &[u32]) -> Option<u32> {
    for x in expenses {
        for y in expenses {
            if x + y == 2020 {
                return Some(x * y);
            }
        }
    }

    None
}

fn three_add_to_2020(expenses: &[u32]) -> Option<u32> {
    for x in expenses {
        for y in expenses {
            for z in expenses {
                if x + y + z == 2020 {
                    return Some(x * y * z);
                }
            }
        }
    }

    None
}
