const INPUT: &str = include_str!("customs_input");

fn main() {
    let num_yes_answers_per_group: usize = INPUT
        .split("\n\n")
        .map(|group| {
            let mut answers_in_group: Vec<_> = group.lines().flat_map(str::chars).collect();
            answers_in_group.sort();
            answers_in_group.dedup();

            answers_in_group.len()
        })
        .sum();

    println!("{}", num_yes_answers_per_group);
}
