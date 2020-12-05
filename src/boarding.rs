use std::ops::Range;

const NUM_ROWS: u32 = 127;
const NUM_COLUMNS: u32 = 7;

pub fn find_seat(seat_locations: &[SeatLocation]) -> SeatLocation {
    let mut all_possible_locations: Vec<_> = (1..NUM_ROWS)
        .flat_map(|row| (1..NUM_COLUMNS).map(move |column| SeatLocation { row, column }))
        .collect();

    all_possible_locations.retain(|location| {
        let have_current_location = seat_locations.contains(location);

        let have_location_with_id_plus_one = seat_locations
            .iter()
            .map(SeatLocation::id)
            .any(|id| id + 1 == location.id());

        let have_location_with_id_minus_one = seat_locations
            .iter()
            .map(SeatLocation::id)
            .any(|id| id - 1 == location.id());

        !have_current_location && have_location_with_id_plus_one && have_location_with_id_minus_one
    });

    assert_eq!(all_possible_locations.len(), 1);

    all_possible_locations[0]
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SeatLocation {
    row: u32,
    column: u32,
}

impl SeatLocation {
    pub fn id(&self) -> u32 {
        self.row * 8 + self.column
    }
}

pub struct BoardingPass {
    partitions: Vec<Partition>,
}

impl BoardingPass {
    pub fn location(&self) -> SeatLocation {
        let mut bounds = Bounds {
            rows: 0..NUM_ROWS,
            columns: 0..NUM_COLUMNS,
        };

        for partition in &self.partitions {
            match partition {
                Partition::Front => bounds.rows = bounds.rows.start..bounds.middle_row(),
                Partition::Back => bounds.rows = bounds.middle_row() + 1..bounds.rows.end,
                Partition::Left => bounds.columns = bounds.columns.start..bounds.middle_column(),
                Partition::Right => bounds.columns = bounds.middle_column() + 1..bounds.columns.end,
            };
        }

        assert_eq!(bounds.rows.start, bounds.rows.end);
        assert_eq!(bounds.columns.start, bounds.columns.end);

        SeatLocation {
            row: bounds.rows.start,
            column: bounds.columns.start,
        }
    }
}

impl From<&str> for BoardingPass {
    fn from(s: &str) -> Self {
        Self {
            partitions: s.chars().map(|c| c.into()).collect(),
        }
    }
}

#[derive(Debug)]
struct Bounds {
    rows: Range<u32>,
    columns: Range<u32>,
}

impl Bounds {
    fn middle_row(&self) -> u32 {
        (self.rows.start + self.rows.end) / 2
    }

    fn middle_column(&self) -> u32 {
        (self.columns.start + self.columns.end) / 2
    }
}

#[derive(Debug, Copy, Clone)]
enum Partition {
    Front,
    Back,
    Left,
    Right,
}

impl From<char> for Partition {
    fn from(c: char) -> Self {
        match c {
            'F' => Self::Front,
            'B' => Self::Back,
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("invalid input"),
        }
    }
}

#[cfg(test)]
mod location_tests {
    use super::*;

    #[test]
    fn it_works() {
        let boarding_pass = BoardingPass {
            partitions: vec![
                Partition::Front,
                Partition::Back,
                Partition::Front,
                Partition::Back,
                Partition::Back,
                Partition::Front,
                Partition::Front,
                Partition::Right,
                Partition::Left,
                Partition::Right,
            ],
        };

        assert_eq!(
            boarding_pass.location(),
            SeatLocation { row: 44, column: 5 },
        )
    }
}
