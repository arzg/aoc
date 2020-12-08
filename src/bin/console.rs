use aoc::console::Vm;

const INPUT: &str = include_str!("console_input");

fn main() {
    let instructions = INPUT.lines().map(|line| line.parse().unwrap());
    let mut vm = Vm::new(instructions);

    println!("{}", vm.accumulator_before_loop());
}
