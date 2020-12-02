use crate::parsing::{extract_char, extract_digits, tag};
use std::convert::TryFrom;

#[derive(Debug, PartialEq)]
pub struct Password<'a> {
    requirements: Requirements,
    password: &'a str,
}

impl<'a> TryFrom<&'a str> for Password<'a> {
    type Error = String;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let (s, requirements) = Requirements::new(s)?;
        let s = tag(": ", s)?;

        Ok(Self {
            requirements,
            password: s,
        })
    }
}

impl Password<'_> {
    pub fn is_valid(&self, ruleset: Ruleset) -> bool {
        self.requirements.met_by_password(&self.password, ruleset)
    }
}

#[derive(Debug, PartialEq)]
struct Requirements {
    num1: usize,
    num2: usize,
    letter: char,
}

impl Requirements {
    fn new(s: &str) -> Result<(&str, Self), String> {
        let (s, num1) = extract_digits(s)?;
        let s = tag("-", s)?;
        let (s, num2) = extract_digits(s)?;
        let s = tag(" ", s)?;
        let (s, letter) = extract_char(s)?;

        Ok((
            s,
            Self {
                num1: num1.parse().unwrap(),
                num2: num2.parse().unwrap(),
                letter,
            },
        ))
    }

    fn met_by_password(&self, password: &str, ruleset: Ruleset) -> bool {
        match ruleset {
            Ruleset::Old => {
                let num_occurrences_of_letter =
                    password.chars().filter(|c| *c == self.letter).count();

                (self.num1..=self.num2).contains(&num_occurrences_of_letter)
            }
            Ruleset::New => {
                let is_first_position_correct =
                    password.chars().nth(self.num1 - 1).unwrap() == self.letter;

                let is_second_position_correct =
                    password.chars().nth(self.num2 - 1).unwrap() == self.letter;

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
            Password::try_from("1-3 a: abcde"),
            Ok(Password {
                requirements: Requirements {
                    num1: 1,
                    num2: 3,
                    letter: 'a',
                },
                password: "abcde",
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
                num1: 1,
                num2: 3,
                letter: 'a',
            },
            password: "abcde",
        };

        assert_eq!(password.is_valid(Ruleset::Old), true);
        assert_eq!(password.is_valid(Ruleset::New), true);
    }

    #[test]
    fn cdefg() {
        let password = Password {
            requirements: Requirements {
                num1: 1,
                num2: 3,
                letter: 'b',
            },
            password: "cdefg",
        };

        assert_eq!(password.is_valid(Ruleset::Old), false);
        assert_eq!(password.is_valid(Ruleset::New), false);
    }

    #[test]
    fn ccccccccc_old_ruleset() {
        let password = Password {
            requirements: Requirements {
                num1: 2,
                num2: 9,
                letter: 'c',
            },
            password: "ccccccccc",
        };

        assert_eq!(password.is_valid(Ruleset::Old), true);
        assert_eq!(password.is_valid(Ruleset::New), false);
    }
}
