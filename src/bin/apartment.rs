use aoc::apartment::Directions;

const INPUT: &str = include_str!("apartment_input");

fn main() {
    let directions = Directions::new(INPUT).unwrap();

    println!("{}", directions.final_floor());
    println!("{}", directions.first_basement_pos());
}
