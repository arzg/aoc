use std::str::FromStr;

use crate::parsing::{extract_digits, tag};

const ROWS: usize = 1000;
const COLUMNS: usize = 1000;
const LIGHTS: usize = ROWS * COLUMNS;

pub struct Grid {
    lights: [Light; LIGHTS],
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            lights: [Light::Off; LIGHTS],
        }
    }
}

impl Grid {
    pub fn apply(&mut self, instruction: Instruction) {
        for (idx, light) in self.lights.iter_mut().enumerate() {
            let current_coordinate = Coordinate(idx % COLUMNS, idx / COLUMNS);

            if current_coordinate.contained_in(instruction.from, instruction.to) {
                light.apply(instruction.action);
            }
        }
    }

    pub fn lit_lights(&self) -> usize {
        self.lights
            .iter()
            .filter(|&&light| light == Light::On)
            .count()
    }
}

pub struct Instruction {
    action: Action,
    from: Coordinate,
    to: Coordinate,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s, action) = Action::new(s)?;
        let s = tag(" ", s)?;

        let (s, from) = Coordinate::new(s)?;

        let s = tag(" through ", s)?;

        let (s, to) = Coordinate::new(s)?;

        if !s.is_empty() {
            return Err("parser did not consume entire input".to_string());
        }

        Ok(Self { action, from, to })
    }
}

#[derive(Copy, Clone)]
enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

impl Action {
    fn new(s: &str) -> Result<(&str, Self), String> {
        Self::new_turn_on(s)
            .or_else(|_| Self::new_turn_off(s))
            .or_else(|_| Self::new_toggle(s))
    }

    fn new_turn_on(s: &str) -> Result<(&str, Self), String> {
        let s = tag("turn on", s)?;
        Ok((s, Self::TurnOn))
    }

    fn new_turn_off(s: &str) -> Result<(&str, Self), String> {
        let s = tag("turn off", s)?;
        Ok((s, Self::TurnOff))
    }

    fn new_toggle(s: &str) -> Result<(&str, Self), String> {
        let s = tag("toggle", s)?;
        Ok((s, Self::Toggle))
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Light {
    On,
    Off,
}

impl Light {
    fn apply(&mut self, instruction: Action) {
        match instruction {
            Action::TurnOn => *self = Self::On,
            Action::TurnOff => *self = Self::Off,
            Action::Toggle => match self {
                Self::On => *self = Self::Off,
                Self::Off => *self = Self::On,
            },
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Coordinate(usize, usize);

impl Coordinate {
    fn new(s: &str) -> Result<(&str, Self), String> {
        let (s, x) = extract_digits(s)?;
        let s = tag(",", s)?;
        let (s, y) = extract_digits(s)?;

        let x = x.parse().unwrap();
        let y = y.parse().unwrap();

        Ok((s, Self(x, y)))
    }

    fn contained_in(&self, from: Self, to: Self) -> bool {
        from.0 <= self.0 && to.0 >= self.0 && from.1 <= self.1 && to.1 >= self.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn turn_on_all_lights() {
        let mut grid = Grid::default();

        grid.apply(Instruction {
            action: Action::TurnOn,
            from: Coordinate(0, 0),
            to: Coordinate(COLUMNS - 1, ROWS - 1),
        });

        assert!(grid.lights.iter().all(|light| *light == Light::On));
    }

    #[test]
    fn toggle_first_row() {
        let mut grid = Grid::default();

        grid.apply(Instruction {
            action: Action::Toggle,
            from: Coordinate(0, 0),
            to: Coordinate(COLUMNS - 1, 0),
        });

        let (first_row, all_others) = grid.lights.split_at(COLUMNS);

        assert!(first_row.iter().all(|light| *light == Light::On));
        assert!(all_others.iter().all(|light| *light == Light::Off));
    }
}
