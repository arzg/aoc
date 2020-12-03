use crate::parsing::tag;
use std::str::FromStr;

pub struct Map {
    data: Vec<Datum>,
    width: usize,
    height: usize,
}

impl Map {
    pub fn num_trees_in_path(&self, offset: Offset) -> usize {
        self.gen_path(offset)
            .filter(|datum| *datum == Datum::Tree)
            .count()
    }

    fn gen_path(&self, offset: Offset) -> impl Iterator<Item = Datum> + '_ {
        PathIter {
            map: self,
            current_pos: Coordinate { x: 0, y: 0 },
            offset,
        }
    }

    fn get(&self, coordinate: Coordinate) -> Option<Datum> {
        if coordinate.y > self.height {
            return None;
        }

        let x = coordinate.x % self.width;

        self.data.get(coordinate.y * self.width + x).copied()
    }
}

impl FromStr for Map {
    type Err = String;

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        let width = s
            .lines()
            .next()
            .ok_or_else(|| "expected at least one line".to_string())?
            .len();

        let height = s.lines().count();

        let mut data = Vec::new();

        loop {
            s = s.strip_prefix("\n").unwrap_or(s);

            if s.is_empty() {
                break;
            }

            let (new_s, datum) = Datum::new(s)?;
            s = new_s;

            data.push(datum);
        }

        Ok(Self {
            data,
            width,
            height,
        })
    }
}

struct PathIter<'m> {
    map: &'m Map,
    current_pos: Coordinate,
    offset: Offset,
}

impl Iterator for PathIter<'_> {
    type Item = Datum;

    fn next(&mut self) -> Option<Self::Item> {
        let datum = self.map.get(self.current_pos)?;
        self.current_pos.offset_by(self.offset);

        Some(datum)
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Datum {
    Open,
    Tree,
}

impl Datum {
    fn new(s: &str) -> Result<(&str, Self), String> {
        tag(".", s)
            .map(|s| (s, Self::Open))
            .or_else(|_| tag("#", s).map(|s| (s, Self::Tree)))
    }
}

#[derive(Debug, Copy, Clone)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn offset_by(&mut self, offset: Offset) {
        self.x += offset.right;
        self.y += offset.down;
    }
}

#[derive(Copy, Clone)]
pub struct Offset {
    pub right: usize,
    pub down: usize,
}
