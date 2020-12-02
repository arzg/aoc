use crate::parsing::{extract_char, extract_digits, tag};
use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
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
    pub fn is_valid(&self, ruleset: Ruleset) -> bool {
        self.requirements.met_by_password(&self.password, ruleset)
    }
}

#[derive(Debug, PartialEq)]
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

    fn met_by_password(&self, password: &str, ruleset: Ruleset) -> bool {
        match ruleset {
            Ruleset::Old => {
                let num_occurrences_of_letter =
                    password.chars().filter(|c| *c == self.letter).count();

                self.num_appearances.contains(&num_occurrences_of_letter)
            }
            Ruleset::New => {
                let is_first_position_correct = password
                    .chars()
                    .nth(self.num_appearances.start() - 1)
                    .unwrap()
                    == self.letter;

                let is_second_position_correct = password
                    .chars()
                    .nth(self.num_appearances.end() - 1)
                    .unwrap()
                    == self.letter;

                is_first_position_correct != is_second_position_correct
            }
        }
    }
}

pub enum Ruleset {
    New,
    Old,
}

#[cfg(test)]
mod parsing_tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Password::from_str("1-3 a: abcde"),
            Ok(Password {
                requirements: Requirements {
                    num_appearances: 1..=3,
                    letter: 'a',
                },
                password: "abcde".to_string(),
            }),
        );
    }
}

#[cfg(test)]
mod validation_tests {
    use super::*;

    #[test]
    fn abcde() {
        let password = Password {
            requirements: Requirements {
                num_appearances: 1..=3,
                letter: 'a',
            },
            password: "abcde".to_string(),
        };

        assert_eq!(password.is_valid(Ruleset::Old), true);
        assert_eq!(password.is_valid(Ruleset::New), true);
    }

    #[test]
    fn cdefg() {
        let password = Password {
            requirements: Requirements {
                num_appearances: 1..=3,
                letter: 'b',
            },
            password: "cdefg".to_string(),
        };

        assert_eq!(password.is_valid(Ruleset::Old), false);
        assert_eq!(password.is_valid(Ruleset::New), false);
    }

    #[test]
    fn ccccccccc_old_ruleset() {
        let password = Password {
            requirements: Requirements {
                num_appearances: 2..=9,
                letter: 'c',
            },
            password: "ccccccccc".to_string(),
        };

        assert_eq!(password.is_valid(Ruleset::Old), true);
        assert_eq!(password.is_valid(Ruleset::New), false);
    }
}
