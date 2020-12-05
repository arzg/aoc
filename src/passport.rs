use std::collections::HashMap;

pub struct Passport {
    fields: HashMap<String, String>,
}

impl Passport {
    pub fn is_complete(&self) -> bool {
        self.fields.contains_key("byr")
            && self.fields.contains_key("iyr")
            && self.fields.contains_key("eyr")
            && self.fields.contains_key("hgt")
            && self.fields.contains_key("hcl")
            && self.fields.contains_key("ecl")
            && self.fields.contains_key("pid")
    }

    pub fn is_valid(&self) -> bool {
        self.fields
            .get("byr")
            .map_or(false, |byr| (1920..=2002).contains(&byr.parse().unwrap()))
            && self
                .fields
                .get("iyr")
                .map_or(false, |iyr| (2010..=2020).contains(&iyr.parse().unwrap()))
            && self
                .fields
                .get("eyr")
                .map_or(false, |eyr| (2020..=2030).contains(&eyr.parse().unwrap()))
            && self.fields.get("hgt").map_or(false, |hgt| {
                hgt.strip_suffix("cm").map_or_else(
                    || {
                        hgt.strip_suffix("in")
                            .map_or(false, |in_| (59..=76).contains(&in_.parse().unwrap()))
                    },
                    |cm| (150..=193).contains(&cm.parse().unwrap()),
                )
            })
            && self.fields.get("hcl").map_or(false, |hcl| {
                hcl.strip_prefix('#').map_or(false, |hex| {
                    hex.len() == 6 && hex.chars().all(|c| c.is_ascii_hexdigit())
                })
            })
            && self.fields.get("ecl").map_or(false, |ecl| {
                ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&ecl.as_str())
            })
            && self.fields.get("pid").map_or(false, |pid| {
                pid.len() == 9 && pid.chars().all(|c| c.is_ascii_digit())
            })
    }
}

impl From<&str> for Passport {
    fn from(s: &str) -> Self {
        Passport {
            fields: s
                .split_whitespace()
                .map(|field| {
                    let colon_idx = field.find(':').unwrap();
                    let (field_name, field_value) = (&field[..colon_idx], &field[colon_idx + 1..]);

                    (field_name.to_string(), field_value.to_string())
                })
                .collect(),
        }
    }
}
