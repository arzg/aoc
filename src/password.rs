use crate::parsing::{extract_char, extract_digits, tag};
use std::ops::RangeInclusive;
use std::str::FromStr;

pub struct Password {
    requirements: Requirements,
    password: String,
}

impl FromStr for Password {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s, requirements) = Requirements::new(s)?;
        let s = tag(": ", s)?;

        Ok(Self {
            requirements,
            password: s.to_string(),
        })
    }
}

impl Password {
    pub fn is_valid(&self) -> bool {
        self.requirements.met_by_password(&self.password)
    }
}

#[derive(Debug)]
struct Requirements {
    num_appearances: RangeInclusive<usize>,
    letter: char,
}

impl Requirements {
    fn new(s: &str) -> Result<(&str, Self), String> {
        let (s, min) = extract_digits(s)?;
        let s = tag("-", s)?;
        let (s, max) = extract_digits(s)?;
        let s = tag(" ", s)?;
        let (s, letter) = extract_char(s)?;

        Ok((
            s,
            Self {
                num_appearances: min.parse().unwrap()..=max.parse().unwrap(),
                letter,
            },
        ))
    }

    fn met_by_password(&self, password: &str) -> bool {
        let num_occurrences_of_letter = password.chars().filter(|c| *c == self.letter).count();

        self.num_appearances.contains(&num_occurrences_of_letter)
    }
}