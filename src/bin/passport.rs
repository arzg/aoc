use aoc::passport::Passport;

const INPUT: &str = include_str!("passport_input");

fn main() {
    let passports: Vec<_> = INPUT.split("\n\n").map(Passport::from).collect();

    let num_complete_passports = passports
        .iter()
        .filter(|passport| passport.is_complete())
        .count();

    let num_valid_passports = passports
        .iter()
        .filter(|passport| passport.is_valid())
        .count();

    println!("Number of complete passports: {}", num_complete_passports);
    println!("Number of valid passports: {}", num_valid_passports);
}
