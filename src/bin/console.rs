use aoc::console::Vm;

const INPUT: &str = include_str!("console_input");

fn main() {
    let instructions = INPUT.lines().map(|line| line.parse().unwrap());
    let mut vm = Vm::new(instructions);

    println!(
        "Accumulator of the VM before looping: {}",
        vm.accumulator_before_loop(),
    );

    vm.reset();

    println!(
        "Accumulator of the VM after fixing the program: {}",
        vm.accumulator_after_fixing_program(),
    );
}
