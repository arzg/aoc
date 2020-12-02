pub fn two_add_to_2020(expenses: &[u32]) -> Option<u32> {
    for x in expenses {
        for y in expenses {
            if x + y == 2020 {
                return Some(x * y);
            }
        }
    }

    None
}

pub fn three_add_to_2020(expenses: &[u32]) -> Option<u32> {
    for x in expenses {
        for y in expenses {
            for z in expenses {
                if x + y + z == 2020 {
                    return Some(x * y * z);
                }
            }
        }
    }

    None
}
