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
        self.birth_year
            .map_or(false, |byr| (1920..=2002).contains(&byr.parse().unwrap()))
            && self
                .issue_year
                .map_or(false, |iyr| (2010..=2020).contains(&iyr.parse().unwrap()))
            && self
                .expiration_year
                .map_or(false, |eyr| (2020..=2030).contains(&eyr.parse().unwrap()))
            && self.height.map_or(false, |hgt| {
                hgt.strip_suffix("cm").map_or_else(
                    || {
                        hgt.strip_suffix("in")
                            .map_or(false, |in_| (59..=76).contains(&in_.parse().unwrap()))
                    },
                    |cm| (150..=193).contains(&cm.parse().unwrap()),
                )
            })
            && self.hair_color.map_or(false, |hcl| {
                hcl.strip_prefix('#').map_or(false, |hex| {
                    hex.len() == 6 && hex.chars().all(|c| c.is_ascii_hexdigit())
                })
            })
            && self.eye_color.map_or(false, |ecl| -> bool {
                ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&ecl)
            })
            && self.passport_id.map_or(false, |pid| {
                pid.len() == 9 && pid.chars().all(|c| c.is_ascii_digit())
            })
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
