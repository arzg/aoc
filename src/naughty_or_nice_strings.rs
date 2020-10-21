#[derive(Debug, PartialEq)]
pub enum Judgement {
    Nice,
    Naughty,
}

impl Judgement {
    pub fn of(s: &str, ruleset: Ruleset) -> Self {
        let is_nice = if ruleset == Ruleset::New {
            two_pair_test(s) && pair_with_other_middle_test(s)
        } else {
            vowel_test(s) && repeated_char_test(s) && does_not_contain_test(s)
        };

        if is_nice {
            Self::Nice
        } else {
            Self::Naughty
        }
    }
}

fn two_pair_test(s: &str) -> bool {
    s.chars()
        .zip(s.chars().skip(1))
        .any(|(char1, char2)| s.matches(&format!("{}{}", char1, char2)).count() >= 2)
}

fn pair_with_other_middle_test(s: &str) -> bool {
    s.chars()
        .zip(s.chars().skip(1))
        .zip(s.chars().skip(2))
        .any(|((first, _second), third)| first == third)
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

#[derive(Copy, Clone, PartialEq)]
pub enum Ruleset {
    Old,
    New,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ugk() {
        assert_eq!(
            Judgement::of("ugknbfddgicrmopn", Ruleset::Old),
            Judgement::Nice,
        );
    }

    #[test]
    fn aaa() {
        assert_eq!(Judgement::of("aaa", Ruleset::Old), Judgement::Nice);
    }

    #[test]
    fn jch() {
        assert_eq!(
            Judgement::of("jchzalrnumimnmhp", Ruleset::Old),
            Judgement::Naughty,
        );
    }

    #[test]
    fn hae() {
        assert_eq!(
            Judgement::of("haegwjzuvuyypxyu", Ruleset::Old),
            Judgement::Naughty,
        );
    }

    #[test]
    fn dvs() {
        assert_eq!(
            Judgement::of("dvszwmarrgswjxmb", Ruleset::Old),
            Judgement::Naughty,
        );
    }

    #[test]
    fn qjh() {
        assert_eq!(
            Judgement::of("qjhvhtzxzqqjkmpb", Ruleset::New),
            Judgement::Nice,
        );
    }

    #[test]
    fn xxy() {
        assert_eq!(Judgement::of("xxyxx", Ruleset::New), Judgement::Nice);
    }

    #[test]
    fn uur() {
        assert_eq!(
            Judgement::of("uurcxstgmygtbstg", Ruleset::New),
            Judgement::Naughty,
        );
    }

    #[test]
    fn ieo() {
        assert_eq!(
            Judgement::of("ieodomkazucvgmuy", Ruleset::New),
            Judgement::Naughty,
        );
    }
}
