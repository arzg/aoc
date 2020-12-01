use aoc::circuit::Circuit;

const INPUT: &str = include_str!("circuit_input");

fn main() {
    let instructions = INPUT.lines().map(|line| line.parse().unwrap());

    let circuit = Circuit {
        instructions: instructions.collect(),
    };

    let wire_signals = circuit.emulate();

    println!(
        "The signal being provided to wire a is {}",
        wire_signals.get("a").unwrap(),
    );
}
