const INPUT: &str = include_str!("customs_input");

fn main() {
    let num_yes_anyone_in_group: usize = INPUT
        .split("\n\n")
        .map(|group| {
            let mut answers_in_group: Vec<_> = group.lines().flat_map(str::chars).collect();
            answers_in_group.sort();
            answers_in_group.dedup();

            answers_in_group.len()
        })
        .sum();

    println!(
        "Number of questions to which anyone in a group answered ‘yes’: {}",
        num_yes_anyone_in_group,
    );

    let num_yes_everyone_in_group: usize = INPUT
        .split("\n\n")
        .map(|group| {
            let mut answers_in_group: Vec<_> = group.lines().flat_map(str::chars).collect();
            answers_in_group.sort();
            answers_in_group.dedup();

            let group_answers: Vec<Vec<_>> =
                group.lines().map(str::chars).map(|c| c.collect()).collect();

            let mut num_yes_everyone_in_group = 0;
            for answer in answers_in_group {
                if group_answers
                    .iter()
                    .all(|answers| answers.contains(&answer))
                {
                    num_yes_everyone_in_group += 1;
                }
            }

            num_yes_everyone_in_group
        })
        .sum();

    println!(
        "Number of questions to which everyone in a group answered ‘yes’: {}",
        num_yes_everyone_in_group,
    );
}
