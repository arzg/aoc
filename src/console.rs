use std::mem;
use std::str::FromStr;

use crate::parsing::{extract_digits, tag};

#[derive(Debug)]
pub struct Vm {
    instructions: Vec<(Instruction, u32)>,
    instruction_pointer: i32,
    accumulator: i32,
}

impl Vm {
    pub fn new(instructions: impl Iterator<Item = Instruction>) -> Self {
        Self {
            instructions: instructions.map(|instruction| (instruction, 0)).collect(),
            instruction_pointer: 0,
            accumulator: 0,
        }
    }

    pub fn accumulator_after_fixing_program(&mut self) -> i32 {
        let mut fix_attempt_idx = 0;

        loop {
            let new_operation = match self.instructions[fix_attempt_idx].0.operation {
                Operation::Accumulate => {
                    fix_attempt_idx += 1;
                    continue;
                }
                Operation::Jump => Operation::NoOp,
                Operation::NoOp => Operation::Jump,
            };

            let old_operation = mem::replace(
                &mut self.instructions[fix_attempt_idx].0.operation,
                new_operation,
            );

            loop {
                if self.will_loop() {
                    break;
                }

                if self.at_end() {
                    return self.accumulator;
                }
            }

            // Restore old operation before attempt.
            let _ = mem::replace(
                &mut self.instructions[fix_attempt_idx].0.operation,
                old_operation,
            );

            self.reset();
            fix_attempt_idx += 1;
        }
    }

    pub fn accumulator_before_loop(&mut self) -> i32 {
        while !self.will_loop() {}

        self.accumulator
    }

    fn will_loop(&mut self) -> bool {
        let (
            Instruction {
                operation,
                argument,
            },
            ref mut num_times_already_evaled,
        ) = self.instructions[self.instruction_pointer as usize];

        if *num_times_already_evaled > 0 {
            return true;
        }

        match operation {
            Operation::Accumulate => {
                self.accumulator += argument;
                self.instruction_pointer += 1;
            }
            Operation::Jump => self.instruction_pointer += argument,
            Operation::NoOp => self.instruction_pointer += 1,
        }

        *num_times_already_evaled += 1;

        false
    }

    fn at_end(&self) -> bool {
        self.instruction_pointer == self.instructions.len() as i32
    }

    pub fn reset(&mut self) {
        self.instruction_pointer = 0;
        self.accumulator = 0;

        for (_, instruction_count) in &mut self.instructions {
            *instruction_count = 0;
        }
    }
}

#[derive(Debug)]
pub struct Instruction {
    operation: Operation,
    argument: i32,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s, operation) = Operation::new(s)?;
        let s = tag(" ", s)?;

        let (s, sign) = tag("+", s)
            .map(|s| (s, 1))
            .or_else(|_| tag("-", s).map(|s| (s, -1)))?;

        let (_s, number) = extract_digits(s)?;
        let number: i32 = number.parse().unwrap();

        Ok(Self {
            operation,
            argument: sign * number,
        })
    }
}

#[derive(Debug, Copy, Clone)]
enum Operation {
    Accumulate,
    Jump,
    NoOp,
}

impl Operation {
    fn new(s: &str) -> Result<(&str, Self), String> {
        Self::new_accumulate(s)
            .or_else(|_| Self::new_jump(s))
            .or_else(|_| Self::new_no_op(s))
    }

    fn new_accumulate(s: &str) -> Result<(&str, Self), String> {
        let s = tag("acc", s)?;
        Ok((s, Self::Accumulate))
    }

    fn new_jump(s: &str) -> Result<(&str, Self), String> {
        let s = tag("jmp", s)?;
        Ok((s, Self::Jump))
    }

    fn new_no_op(s: &str) -> Result<(&str, Self), String> {
        let s = tag("nop", s)?;
        Ok((s, Self::NoOp))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let instructions = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"
            .lines()
            .map(|line| line.parse().unwrap());

        let mut vm = Vm::new(instructions);

        // assert_eq!(vm.accumulator_before_loop(), 5);
        // vm.reset();
        assert_eq!(vm.accumulator_after_fixing_program(), 8);
    }
}
