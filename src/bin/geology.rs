use aoc::geology::{Map, Slope};
use std::str::FromStr;

const INPUT: &str = include_str!("geology_input");

const SLOPES: &[Slope] = &[
    Slope { right: 1, down: 1 },
    Slope { right: 3, down: 1 },
    Slope { right: 5, down: 1 },
    Slope { right: 7, down: 1 },
    Slope { right: 1, down: 2 },
];

fn main() {
    let map = Map::from_str(INPUT).unwrap();

    println!(
        "Number of trees in path with slope of ‘right 3, down 1’: {}",
        map.num_trees_in_path(Slope { right: 3, down: 1 })
    );

    let num_trees_product: usize = SLOPES
        .iter()
        .map(|slope| map.num_trees_in_path(*slope))
        .product();

    println!(
        "Product of number of trees in path of slopes: {}",
        num_trees_product
    );
}
