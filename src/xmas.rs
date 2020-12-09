pub struct Input {
    data: Vec<u64>,
}

impl Input {
    pub fn new(s: &str) -> Self {
        Self {
            data: s.lines().map(|line| line.parse().unwrap()).collect(),
        }
    }

    pub fn determine_weakness(&self) -> u64 {
        let first_invalid_number = self.first_invalid_number();

        for start_idx in 0..self.data.len() {
            for end_idx in start_idx + 2..self.data.len() {
                let range = &self.data[start_idx..end_idx];

                if range.iter().sum::<u64>() == first_invalid_number {
                    let smallest_in_range = range.iter().min().unwrap();
                    let largest_in_range = range.iter().max().unwrap();

                    return smallest_in_range + largest_in_range;
                }
            }
        }

        unreachable!()
    }

    pub fn first_invalid_number(&self) -> u64 {
        'data: for (idx, current) in self.data.iter().copied().enumerate().skip(25) {
            let last_25_numbers = &self.data[idx - 25..idx];

            for x in last_25_numbers {
                for y in last_25_numbers {
                    if x + y == current {
                        continue 'data;
                    }
                }
            }

            return current;
        }

        unreachable!()
    }
}
