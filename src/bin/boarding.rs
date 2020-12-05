use aoc::boarding::{find_seat, BoardingPass};

const INPUT: &str = include_str!("boarding_input");

fn main() {
    let seat_locations: Vec<_> = INPUT
        .lines()
        .map(BoardingPass::from)
        .map(|pass| pass.location())
        .collect();

    let highest_id = seat_locations.iter().map(|location| location.id()).max();

    if let Some(id) = highest_id {
        println!("{}", id);
    } else {
        println!("No seat ID was found");
    }

    let seat_location = find_seat(&seat_locations);
    println!("Your seat location’s ID is {}", seat_location.id());
}
