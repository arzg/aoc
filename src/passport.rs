use std::collections::HashMap;

pub struct Passport<'a> {
    birth_year: Option<&'a str>,
    issue_year: Option<&'a str>,
    expiration_year: Option<&'a str>,
    height: Option<&'a str>,
    hair_color: Option<&'a str>,
    eye_color: Option<&'a str>,
    passport_id: Option<&'a str>,
    #[allow(dead_code)]
    country_id: Option<&'a str>,
}

impl Passport<'_> {
    pub fn is_complete(&self) -> bool {
        self.birth_year.is_some()
            && self.issue_year.is_some()
            && self.expiration_year.is_some()
            && self.height.is_some()
            && self.hair_color.is_some()
            && self.eye_color.is_some()
            && self.passport_id.is_some()
    }

    pub fn is_valid(&self) -> bool {
        if !self.is_complete() {
            return false;
        }

        let birth_year = self.birth_year.unwrap();
        let issue_year = self.issue_year.unwrap();
        let expiration_year = self.expiration_year.unwrap();
        let height = self.height.unwrap();
        let hair_color = self.hair_color.unwrap();
        let eye_color = self.eye_color.unwrap();
        let passport_id = self.passport_id.unwrap();

        (1920..=2002).contains(&birth_year.parse().unwrap())
            && (2010..=2020).contains(&issue_year.parse().unwrap())
            && (2020..=2030).contains(&expiration_year.parse().unwrap())
            && height.strip_suffix("cm").map_or_else(
                || {
                    height
                        .strip_suffix("in")
                        .map_or(false, |in_| (59..=76).contains(&in_.parse().unwrap()))
                },
                |cm| (150..=193).contains(&cm.parse().unwrap()),
            )
            && hair_color.strip_prefix('#').map_or(false, |hex| {
                hex.len() == 6 && hex.chars().all(|c| c.is_ascii_hexdigit())
            })
            && ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&eye_color)
            && passport_id.len() == 9
            && passport_id.chars().all(|c| c.is_ascii_digit())
    }
}

impl<'a> From<&'a str> for Passport<'a> {
    fn from(s: &'a str) -> Self {
        let fields: HashMap<_, _> = s
            .split_whitespace()
            .map(|field| {
                let colon_idx = field.find(':').unwrap();
                let (field_name, field_value) = (&field[..colon_idx], &field[colon_idx + 1..]);

                (field_name, field_value)
            })
            .collect();

        Passport {
            birth_year: fields.get("byr").copied(),
            issue_year: fields.get("iyr").copied(),
            expiration_year: fields.get("eyr").copied(),
            height: fields.get("hgt").copied(),
            hair_color: fields.get("hcl").copied(),
            eye_color: fields.get("ecl").copied(),
            passport_id: fields.get("pid").copied(),
            country_id: fields.get("cid").copied(),
        }
    }
}
