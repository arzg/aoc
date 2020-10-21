use md5::{Digest, Md5};

pub struct Miner {
    key: &'static str,
    current_guess: u32,
}

impl Miner {
    pub fn new(key: &'static str) -> Self {
        Self {
            key,
            current_guess: 0,
        }
    }

    pub fn mine(&mut self, num_zeroes: usize) -> u32 {
        let required_prefix = "0".repeat(num_zeroes);

        let is_guess_valid = |hash| hex::encode(hash)[0..num_zeroes] == required_prefix;

        loop {
            let hash_input = format!("{}{}", self.key, self.current_guess);
            let hash = Md5::digest(hash_input.as_bytes());

            if is_guess_valid(hash) {
                return self.current_guess;
            }

            self.current_guess += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abcdef() {
        let mut miner = Miner::new("abcdef");
        assert_eq!(miner.mine(5), 609043);
    }

    #[test]
    fn pqrstuv() {
        let mut miner = Miner::new("pqrstuv");
        assert_eq!(miner.mine(5), 1048970);
    }
}
