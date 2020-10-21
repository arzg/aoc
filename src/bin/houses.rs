use aoc::houses::Moves;

const INPUT: &str = include_str!("houses_input");

fn main() {
    let moves = Moves::new(INPUT.trim()).unwrap();

    println!(
        "without Robo-Santa: {}",
        moves.num_houses_with_presents(false),
    );
    println!("with Robo-Santa: {}", moves.num_houses_with_presents(true));
}
