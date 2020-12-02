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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_with_two() {
        assert_eq!(
            two_add_to_2020(&[1721, 979, 366, 299, 675, 1456]),
            Some(514579),
        );
    }

    #[test]
    fn it_works_with_three() {
        assert_eq!(
            three_add_to_2020(&[1721, 979, 366, 299, 675, 1456]),
            Some(241861950),
        );
    }
}
