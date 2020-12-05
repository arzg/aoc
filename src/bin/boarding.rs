use aoc::boarding::BoardingPass;

const INPUT: &str = include_str!("boarding_input");

fn main() {
    let highest_id = INPUT
        .lines()
        .map(BoardingPass::from)
        .map(|pass| pass.location())
        .map(|location| location.id())
        .max();

    if let Some(id) = highest_id {
        println!("{}", id);
    } else {
        println!("No seat ID was found");
    }
}
