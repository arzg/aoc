use std::collections::HashMap;

pub struct Passport {
    birth_year: Option<String>,
    issue_year: Option<String>,
    expiration_year: Option<String>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    #[allow(dead_code)]
    country_id: Option<String>,
}

impl Passport {
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
            .as_ref()
            .map_or(false, |byr| (1920..=2002).contains(&byr.parse().unwrap()))
            && self
                .issue_year
                .as_ref()
                .map_or(false, |iyr| (2010..=2020).contains(&iyr.parse().unwrap()))
            && self
                .expiration_year
                .as_ref()
                .map_or(false, |eyr| (2020..=2030).contains(&eyr.parse().unwrap()))
            && self.height.as_ref().map_or(false, |hgt| {
                hgt.strip_suffix("cm").map_or_else(
                    || {
                        hgt.strip_suffix("in")
                            .map_or(false, |in_| (59..=76).contains(&in_.parse().unwrap()))
                    },
                    |cm| (150..=193).contains(&cm.parse().unwrap()),
                )
            })
            && self.hair_color.as_ref().map_or(false, |hcl| {
                hcl.strip_prefix('#').map_or(false, |hex| {
                    hex.len() == 6 && hex.chars().all(|c| c.is_ascii_hexdigit())
                })
            })
            && self.eye_color.as_ref().map_or(false, |ecl| {
                ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&ecl.as_str())
            })
            && self.passport_id.as_ref().map_or(false, |pid| {
                pid.len() == 9 && pid.chars().all(|c| c.is_ascii_digit())
            })
    }
}

impl From<&str> for Passport {
    fn from(s: &str) -> Self {
        let fields: HashMap<String, String> = s
            .split_whitespace()
            .map(|field| {
                let colon_idx = field.find(':').unwrap();
                let (field_name, field_value) = (&field[..colon_idx], &field[colon_idx + 1..]);

                (field_name.to_string(), field_value.to_string())
            })
            .collect();

        Passport {
            birth_year: fields.get("byr").cloned(),
            issue_year: fields.get("iyr").cloned(),
            expiration_year: fields.get("eyr").cloned(),
            height: fields.get("hgt").cloned(),
            hair_color: fields.get("hcl").cloned(),
            eye_color: fields.get("ecl").cloned(),
            passport_id: fields.get("pid").cloned(),
            country_id: fields.get("cid").cloned(),
        }
    }
}
