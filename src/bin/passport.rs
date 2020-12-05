use aoc::passport::Passport;

const INPUT: &str = include_str!("passport_input");

fn main() {
    let num_valid_passports = INPUT
        .split("\n\n")
        .map(Passport::from)
        .filter(|passport| passport.is_valid())
        .count();

    println!("{}", num_valid_passports);
}
