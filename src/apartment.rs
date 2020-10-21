use std::str::Chars;

#[derive(Clone)]
pub struct Directions<'s> {
    chars: Chars<'s>,
}

impl<'s> Directions<'s> {
    pub fn new(s: &'s str) -> Self {
        Self { chars: s.chars() }
    }

    pub fn final_floor(self) -> i32 {
        self.fold(0, |floor, instruction| instruction.offset_floor(floor))
    }

    pub fn first_basement_pos(self) -> Option<usize> {
        self.enumerate()
            .try_fold(0, |current_floor, (idx, instruction)| {
                let new_floor = instruction.offset_floor(current_floor);

                if new_floor == -1 {
                    Err(idx + 1) // Instruction indices are one-based.
                } else {
                    Ok(new_floor)
                }
            })
            .err()
    }
}

impl Iterator for Directions<'_> {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        match self.chars.next()? {
            '(' => Some(Instruction::Up),
            ')' => Some(Instruction::Down),
            _ => None,
        }
    }
}

#[derive(PartialEq)]
pub enum Instruction {
    Up,
    Down,
}

impl Instruction {
    fn offset_floor(&self, floor: i32) -> i32 {
        match self {
            Self::Up => floor + 1,
            Self::Down => floor - 1,
        }
    }
}

#[cfg(test)]
mod final_floor_tests {
    use super::*;

    fn check(input: &str, expected_final_floor: i32) {
        let directions = Directions::new(input);
        assert_eq!(directions.final_floor(), expected_final_floor);
    }

    #[test]
    fn up_up_down_down() {
        check("(())", 0);
    }

    #[test]
    fn up_down_up_down() {
        check("()()", 0);
    }

    #[test]
    fn up_up_up() {
        check("(((", 3);
    }

    #[test]
    fn down_down_up_up_up_up_up() {
        check("))(((((", 3);
    }

    #[test]
    fn up_down_down() {
        check("())", -1);
    }

    #[test]
    fn down_down_up() {
        check("))(", -1);
    }

    #[test]
    fn down_down_down() {
        check(")))", -3);
    }

    #[test]
    fn down_up_down_down_up_down_down() {
        check(")())())", -3);
    }
}

#[cfg(test)]
mod first_basement_pos_tests {
    use super::*;

    fn check(input: &str, expected_pos_to_enter_basement: Option<usize>) {
        let directions = Directions::new(input);

        assert_eq!(
            directions.first_basement_pos(),
            expected_pos_to_enter_basement,
        );
    }

    #[test]
    fn down() {
        check(")", Some(1));
    }

    #[test]
    fn up_down_up_down_down() {
        check("()())", Some(5));
    }

    #[test]
    fn down_up_down() {
        check(")()", Some(1));
    }

    #[test]
    fn up_up_up() {
        check("(((", None);
    }
}
