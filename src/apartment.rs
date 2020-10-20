pub struct Directions {
    instructions: Vec<Instruction>,
}

impl Directions {
    pub fn new(s: &str) -> Option<Self> {
        let mut instructions = Vec::new();

        for c in s.trim().chars() {
            match c {
                '(' => instructions.push(Instruction::Up),
                ')' => instructions.push(Instruction::Down),
                _ => return None,
            }
        }

        Some(Self { instructions })
    }

    pub fn final_floor(&self) -> i32 {
        let num_floors_up = self
            .instructions
            .iter()
            .filter(|instruction| instruction == &&Instruction::Up)
            .count();

        let num_floors_down = self
            .instructions
            .iter()
            .filter(|instruction| instruction == &&Instruction::Down)
            .count();

        i32::from(num_floors_up as u16) - i32::from(num_floors_down as u16)
    }
}

#[derive(PartialEq)]
enum Instruction {
    Up,
    Down,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(input: &str, expected_final_floor: i32) {
        let directions = Directions::new(input).unwrap();
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
