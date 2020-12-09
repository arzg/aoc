pub struct Input {
    data: Vec<u64>,
}

impl Input {
    pub fn new(s: &str) -> Self {
        Self {
            data: s.lines().map(|line| line.parse().unwrap()).collect(),
        }
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
