use aoc::apartment::Directions;

const INPUT: &str = include_str!("apartment_input");

fn main() {
    let directions = Directions::new(INPUT).unwrap();

    println!("{}", directions.final_floor());

    println!(
        "{}",
        directions.first_basement_pos().map_or_else(
            || "Santa was never instructed to go into the basement.".to_string(),
            |n| n.to_string(),
        ),
    );
}
