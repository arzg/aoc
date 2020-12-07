use crate::parsing::{extract_digits, extract_lowercase, sequence1, tag};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
pub struct Rules {
    rules: HashMap<Bag, Specification>,
}

impl Rules {
    pub fn num_bags_that_can_contain(&self, bag: &Bag) -> usize {
        fn can_contain(rules: &Rules, bag: &Bag, spec: &Specification) -> bool {
            let bags_with_quantities = match spec {
                Specification::MultipleBags(bags_with_quantities) => bags_with_quantities,
                Specification::NoBags => return false,
            };

            bags_with_quantities
                .iter()
                .any(|BagWithQuantity { bag: spec_bag, .. }| {
                    spec_bag == bag || can_contain(rules, bag, rules.rules.get(spec_bag).unwrap())
                })
        }

        self.rules
            .values()
            .filter(|contains| can_contain(self, bag, contains))
            .count()
    }

    pub fn num_bags_contained_by(&self, bag: &Bag) -> usize {
        match self.rules.get(bag).unwrap() {
            Specification::MultipleBags(bags_with_quantities) => bags_with_quantities
                .iter()
                .map(|BagWithQuantity { bag, quantity }| {
                    quantity + self.num_bags_contained_by(bag) * quantity
                })
                .sum(),

            Specification::NoBags => 0,
        }
    }
}

impl FromStr for Rules {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_s, rules) = sequence1(Rule::new, |s| tag("\n", s), s)?;

        let rules = rules
            .into_iter()
            .map(|Rule { subject, spec }| (subject, spec))
            .collect();

        Ok(Self { rules })
    }
}

#[derive(Debug)]
struct Rule {
    subject: Bag,
    spec: Specification,
}

impl Rule {
    fn new(s: &str) -> Result<(&str, Self), String> {
        let (s, subject) = Bag::new(s)?;
        let s = tag(" bags contain ", s)?;

        let (s, spec) = Specification::new(s)?;

        let s = tag(".", s)?;

        Ok((s, Self { subject, spec }))
    }
}

#[derive(Debug, PartialEq)]
enum Specification {
    MultipleBags(Vec<BagWithQuantity>),
    NoBags,
}

impl Specification {
    fn new(s: &str) -> Result<(&str, Self), String> {
        Self::new_multiple(s).or_else(|_| Self::new_no_bags(s))
    }

    fn new_multiple(s: &str) -> Result<(&str, Self), String> {
        sequence1(BagWithQuantity::new, |s| tag(", ", s), s)
            .map(|(s, bags_with_quantities)| (s, Self::MultipleBags(bags_with_quantities)))
    }

    fn new_no_bags(s: &str) -> Result<(&str, Self), String> {
        let s = tag("no other bags", s)?;

        Ok((s, Self::NoBags))
    }
}

#[derive(Debug, PartialEq)]
struct BagWithQuantity {
    bag: Bag,
    quantity: usize,
}

impl BagWithQuantity {
    fn new(s: &str) -> Result<(&str, Self), String> {
        let (s, num_bags) = extract_digits(s)?;
        let quantity = num_bags.parse().unwrap();
        let s = tag(" ", s)?;

        let (s, bag) = Bag::new(s)?;

        let s = tag(" bag", s)?;
        // Handle plural.
        let s = tag("s", s).unwrap_or(s);

        Ok((s, Self { bag, quantity }))
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Bag {
    pub modifier: String,
    pub color: String,
}

impl Bag {
    fn new(s: &str) -> Result<(&str, Self), String> {
        let (s, modifier) = extract_lowercase(s)?;
        let s = tag(" ", s)?;
        let (s, color) = extract_lowercase(s)?;

        Ok((
            s,
            Self {
                modifier: modifier.to_string(),
                color: color.to_string(),
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_level_one_bag() {
        let rules = Rules::from_str("light red bags contain 1 shiny gold bag.").unwrap();

        let bag = Bag {
            modifier: "shiny".to_string(),
            color: "gold".to_string(),
        };

        assert_eq!(rules.num_bags_that_can_contain(&bag), 1);
    }

    #[test]
    fn one_level_two_bags() {
        let rules = Rules::from_str(
            "light red bags contain 1 shiny gold bag.
dark green bags contain 1 shiny gold bag.",
        )
        .unwrap();

        let bag = Bag {
            modifier: "shiny".to_string(),
            color: "gold".to_string(),
        };

        assert_eq!(rules.num_bags_that_can_contain(&bag), 2);
    }

    #[test]
    fn two_levels_two_bags() {
        let rules = Rules::from_str(
            "light red bags contain 1 dark green bag.
dark green bags contain 1 shiny gold bag.",
        )
        .unwrap();

        let bag = Bag {
            modifier: "shiny".to_string(),
            color: "gold".to_string(),
        };

        assert_eq!(rules.num_bags_that_can_contain(&bag), 2);
    }

    #[test]
    fn top_level_contains_bag_that_is_being_looked_for() {
        let rules = Rules::from_str(
            "light red bags contain 1 dark green bag.
shiny gold bags contain no other bags.
dark green bags contain 1 shiny gold bag.",
        )
        .unwrap();

        let bag = Bag {
            modifier: "shiny".to_string(),
            color: "gold".to_string(),
        };

        assert_eq!(rules.num_bags_that_can_contain(&bag), 2);
    }
}
