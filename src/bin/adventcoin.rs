use aoc::adventcoin::Miner;

const SECRET_KEY: &str = "bgvyzdsv";

fn main() {
    let mut miner = Miner::new(SECRET_KEY);
    println!("{}", miner.mine(5));
}
