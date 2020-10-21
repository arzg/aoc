#[derive(Debug, PartialEq)]
pub enum Judgement {
    Nice,
    Naughty,
}

impl Judgement {
    pub fn of(s: &str) -> Self {
        if vowel_test(s) && repeated_char_test(s) && does_not_contain_test(s) {
            Self::Nice
        } else {
            Self::Naughty
        }
    }
}

fn vowel_test(s: &str) -> bool {
    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    s.matches(&VOWELS[..]).count() >= 3
}

fn repeated_char_test(s: &str) -> bool {
    s.chars()
        .zip(s.chars().skip(1))
        .any(|(char1, char2)| char1 == char2)
}

fn does_not_contain_test(s: &str) -> bool {
    const EVIL_STRINGS: [&str; 4] = ["ab", "cd", "pq", "xy"];

    !EVIL_STRINGS
        .iter()
        .any(|evil_string| s.contains(evil_string))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ugk() {
        assert_eq!(Judgement::of("ugknbfddgicrmopn"), Judgement::Nice);
    }

    #[test]
    fn aaa() {
        assert_eq!(Judgement::of("aaa"), Judgement::Nice);
    }

    #[test]
    fn jch() {
        assert_eq!(Judgement::of("jchzalrnumimnmhp"), Judgement::Naughty);
    }

    #[test]
    fn hae() {
        assert_eq!(Judgement::of("haegwjzuvuyypxyu"), Judgement::Naughty);
    }

    #[test]
    fn dvs() {
        assert_eq!(Judgement::of("dvszwmarrgswjxmb"), Judgement::Naughty);
    }
}
