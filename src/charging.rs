use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
pub struct Adapters {
    adapters: Vec<Adapter>,
}

impl Adapters {
    pub fn differences_to_outlet(&mut self) -> Vec<Difference> {
        self.adapters.sort();

        self.adapters
            .windows(2)
            .map(|adapters| {
                let last_adapter = &adapters[0];
                let current_adapter = &adapters[1];

                match current_adapter.jolts - last_adapter.jolts {
                    1 => Difference::One,
                    3 => Difference::Three,
                    _ => unreachable!(),
                }
            })
            .collect()
    }
}

impl FromStr for Adapters {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut adapters: Vec<_> = std::iter::once(Ok(Adapter { jolts: 0 }))
            .chain(s.lines().map(|line| {
                Ok(Adapter {
                    jolts: line.parse()?,
                })
            }))
            .collect::<Result<_, _>>()?;

        adapters.push(Adapter {
            jolts: adapters.iter().max().unwrap().jolts + 3,
        });

        Ok(Self { adapters })
    }
}

#[derive(Debug, PartialEq)]
pub enum Difference {
    One,
    Three,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Adapter {
    jolts: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small() {
        let mut adapters = Adapters::from_str(
            "16
10
15
5
1
11
7
19
6
12
4",
        )
        .unwrap();

        assert_eq!(
            adapters.differences_to_outlet(),
            vec![
                Difference::One,
                Difference::Three,
                Difference::One,
                Difference::One,
                Difference::One,
                Difference::Three,
                Difference::One,
                Difference::One,
                Difference::Three,
                Difference::One,
                Difference::Three,
                Difference::Three,
            ]
        );
    }
}
