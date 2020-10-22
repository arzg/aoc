use std::str::FromStr;

use crate::parsing::{extract_digits, tag};

const ROWS: usize = 1000;
const COLUMNS: usize = 1000;
const LIGHTS: usize = ROWS * COLUMNS;

pub struct Grid<L: Light> {
    lights: Vec<L>,
}

impl<L: Light> Default for Grid<L> {
    fn default() -> Self {
        Self {
            lights: vec![L::default(); LIGHTS],
        }
    }
}

impl<L: Light> Grid<L> {
    pub fn apply(&mut self, instruction: Instruction) {
        let rows = self.lights.chunks_mut(COLUMNS);

        // We need to add one because the instructions are inclusive.
        let affected_rows = rows.take(instruction.to.y + 1).skip(instruction.from.y);

        for row in affected_rows {
            let affected_lights = &mut row[instruction.from.x..=instruction.to.x];

            for light in affected_lights {
                light.apply(instruction.action);
            }
        }
    }

    pub fn total_brightness(&self) -> u32 {
        self.lights.iter().map(Light::brightness).sum()
    }
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Action {
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

pub trait Light: Clone + Default {
    fn apply(&mut self, action: Action);
    fn brightness(&self) -> u32;
}

#[derive(Clone, PartialEq)]
pub enum BinaryLight {
    On,
    Off,
}

impl Light for BinaryLight {
    fn apply(&mut self, action: Action) {
        match action {
            Action::TurnOn => *self = Self::On,
            Action::TurnOff => *self = Self::Off,
            Action::Toggle => match self {
                Self::On => *self = Self::Off,
                Self::Off => *self = Self::On,
            },
        }
    }

    fn brightness(&self) -> u32 {
        match self {
            Self::On => 1,
            Self::Off => 0,
        }
    }
}

impl Default for BinaryLight {
    fn default() -> Self {
        Self::Off
    }
}

#[derive(Clone)]
pub struct ScalarLight {
    brightness: u32,
}

impl Light for ScalarLight {
    fn apply(&mut self, action: Action) {
        match action {
            Action::TurnOn => self.brightness += 1,
            Action::TurnOff => self.brightness = self.brightness.saturating_sub(1),
            Action::Toggle => self.brightness += 2,
        }
    }

    fn brightness(&self) -> u32 {
        self.brightness
    }
}

impl Default for ScalarLight {
    fn default() -> Self {
        Self { brightness: 0 }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn new(s: &str) -> Result<(&str, Self), String> {
        let (s, x) = extract_digits(s)?;
        let s = tag(",", s)?;
        let (s, y) = extract_digits(s)?;

        let x = x.parse().unwrap();
        let y = y.parse().unwrap();

        Ok((s, Self { x, y }))
    }
}

#[cfg(test)]
mod binary_tests {
    use super::*;

    #[test]
    fn turn_on_all_lights() {
        let mut grid: Grid<BinaryLight> = Grid::default();

        grid.apply(Instruction {
            action: Action::TurnOn,
            from: Coordinate { x: 0, y: 0 },
            to: Coordinate {
                x: COLUMNS - 1,
                y: ROWS - 1,
            },
        });

        assert!(grid.lights.iter().all(|light| *light == BinaryLight::On));
    }

    #[test]
    fn toggle_first_row() {
        let mut grid: Grid<BinaryLight> = Grid::default();

        grid.apply(Instruction {
            action: Action::Toggle,
            from: Coordinate { x: 0, y: 0 },
            to: Coordinate {
                x: COLUMNS - 1,
                y: 0,
            },
        });

        let (first_row, all_others) = grid.lights.split_at(COLUMNS);

        assert!(first_row.iter().all(|light| *light == BinaryLight::On));
        assert!(all_others.iter().all(|light| *light == BinaryLight::Off));
    }
}

#[cfg(test)]
mod scalar_tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn turn_on_first() {
        let mut grid: Grid<ScalarLight> = Grid::default();

        grid.apply(Instruction {
            action: Action::TurnOn,
            from: Coordinate { x: 0, y: 0 },
            to: Coordinate { x: 0, y: 0 },
        });

        assert_eq!(grid.total_brightness(), 1);
    }

    #[test]
    fn do_not_overflow_if_turning_off_all_lights() {
        let mut grid: Grid<ScalarLight> = Grid::default();

        grid.apply(Instruction {
            action: Action::TurnOff,
            from: Coordinate { x: 0, y: 0 },
            to: Coordinate {
                x: COLUMNS - 1,
                y: ROWS - 1,
            },
        });

        assert_eq!(grid.total_brightness(), 0);
    }

    #[test]
    fn toggle_all_lights() {
        let mut grid: Grid<ScalarLight> = Grid::default();

        grid.apply(Instruction {
            action: Action::Toggle,
            from: Coordinate { x: 0, y: 0 },
            to: Coordinate {
                x: COLUMNS - 1,
                y: ROWS - 1,
            },
        });

        assert_eq!(grid.total_brightness(), u32::try_from(LIGHTS).unwrap() * 2);
    }
}

#[cfg(test)]
mod parsing_tests {
    use super::*;

    #[test]
    fn parse_instruction() {
        assert_eq!(
            "turn on 0,0 through 999,999".parse(),
            Ok(Instruction {
                action: Action::TurnOn,
                from: Coordinate { x: 0, y: 0 },
                to: Coordinate { x: 999, y: 999 },
            }),
        );
    }

    #[test]
    fn parse_action() {
        assert_eq!(Action::new("toggle"), Ok(("", Action::Toggle)));
    }

    #[test]
    fn parse_coordinate() {
        assert_eq!(
            Coordinate::new("123,456"),
            Ok(("", Coordinate { x: 123, y: 456 })),
        );
    }
}
