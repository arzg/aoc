use aoc::adventcoin::Miner;

const SECRET_KEY: &str = "bgvyzdsv";

fn main() {
    let mut miner = Miner::new(SECRET_KEY);

    println!("Lowest with 5 leading zeroes: {}", miner.mine(5));
    println!("Lowest with 6 leading zeroes: {}", miner.mine(6));
}
