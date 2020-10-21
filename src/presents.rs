use crate::parsing::{extract_digits, tag};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Present {
    l: u32,
    w: u32,
    h: u32,
}

impl Present {
    pub fn wrapping_paper_area(&self) -> u32 {
        let side_areas = self.side_areas();
        let smalled_side_area = side_areas.iter().min().unwrap();
        let total_area_of_all_sides: u32 = side_areas.iter().map(|side| side * 2).sum();

        total_area_of_all_sides + smalled_side_area
    }

    fn side_areas(&self) -> [u32; 3] {
        [self.l * self.w, self.w * self.h, self.h * self.l]
    }
}

impl FromStr for Present {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s, l) = extract_digits(s)?;
        let s = tag("x", s)?;
        let (s, w) = extract_digits(s)?;
        let s = tag("x", s)?;
        let (s, h) = extract_digits(s)?;

        if !s.is_empty() {
            return Err("parser did not consume entire input".to_string());
        }

        let l = l.parse().unwrap();
        let w = w.parse().unwrap();
        let h = h.parse().unwrap();

        Ok(Self { l, w, h })
    }
}

#[cfg(test)]
mod parsing_tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            "10x20x30".parse(),
            Ok(Present {
                l: 10,
                w: 20,
                h: 30,
            }),
        );
    }
}

#[cfg(test)]
mod wrapping_paper_tests {
    use super::*;

    #[test]
    fn two_by_three_by_four() {
        let present = Present { l: 2, w: 3, h: 4 };
        assert_eq!(present.wrapping_paper_area(), 58);
    }

    #[test]
    fn one_by_one_by_ten() {
        let present = Present { l: 1, w: 1, h: 10 };
        assert_eq!(present.wrapping_paper_area(), 43);
    }
}
