use std::collections::HashMap;

pub struct Passport {
    fields: HashMap<String, String>,
}

impl Passport {
    pub fn is_valid(&self) -> bool {
        self.fields.contains_key("byr")
            && self.fields.contains_key("iyr")
            && self.fields.contains_key("eyr")
            && self.fields.contains_key("hgt")
            && self.fields.contains_key("hcl")
            && self.fields.contains_key("ecl")
            && self.fields.contains_key("pid")
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
