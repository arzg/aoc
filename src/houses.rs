pub struct Moves {
    moves: Vec<Move>,
}

impl Moves {
    pub fn new(s: &str) -> Option<Self> {
        let mut moves = Vec::new();

        for c in s.chars() {
            moves.push(match c {
                '^' => Move::Up,
                'v' => Move::Down,
                '<' => Move::Left,
                '>' => Move::Right,
                _ => return None,
            });
        }

        Some(Self { moves })
    }

    pub fn num_houses_with_presents(&self) -> usize {
        let mut visited_houses = self.visited_houses();
        visited_houses.sort_unstable();
        visited_houses.dedup();

        visited_houses.len()
    }

    fn visited_houses(&self) -> Vec<House> {
        let mut current_pos = Coordinate { x: 0, y: 0 };
        let mut visited_houses = vec![House { pos: current_pos }];

        for move_ in &self.moves {
            current_pos.apply_move(*move_);
            visited_houses.push(House { pos: current_pos });
        }

        visited_houses
    }
}

#[derive(Copy, Clone)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct House {
    pos: Coordinate,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn apply_move(&mut self, move_: Move) {
        match move_ {
            Move::Up => self.y -= 1,
            Move::Down => self.y += 1,
            Move::Left => self.x -= 1,
            Move::Right => self.x += 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(input: &str, expected_num_houses_with_presents: usize) {
        let moves = Moves::new(input).unwrap();

        assert_eq!(
            moves.num_houses_with_presents(),
            expected_num_houses_with_presents,
        );
    }

    #[test]
    fn right() {
        check(">", 2);
    }

    #[test]
    fn up_right_down_left() {
        check("^>v<", 4);
    }

    #[test]
    fn up_down_up_down_up_down() {
        check("^v^v^v", 2);
    }
}
