use aoc::customs::Group;

const INPUT: &str = include_str!("customs_input");

fn main() {
    let groups: Vec<_> = INPUT.split("\n\n").map(Group::from).collect();

    let num_questions: usize = groups.iter().map(Group::num_questions).sum();
    let num_questions_all_have: usize = groups.iter().map(Group::num_questions_all_have).sum();

    println!(
        "Number of questions to which anyone in a group answered ‘yes’: {}",
        num_questions,
    );

    println!(
        "Number of questions to which everyone in a group answered ‘yes’: {}",
        num_questions_all_have,
    );
}
