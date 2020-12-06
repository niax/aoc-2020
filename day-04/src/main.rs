use commons::io::{load_file_records, ParseLinesError};
use derive_builder::Builder;
use lazy_static::lazy_static;
use std::convert::Infallible;
use std::str::FromStr;

#[derive(Debug, Clone)]
enum Distance {
    Centimeters(u32),
    Inches(u32),
}

#[derive(Debug)]
enum DistanceFromStrErr {
    InvalidUnit(String),
}

impl FromStr for Distance {
    type Err = DistanceFromStrErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut size_str = String::new();
        let mut unit = String::new();
        for c in s.chars() {
            if c.is_ascii_digit() {
                size_str.push(c);
            } else {
                unit.push(c);
            }
        }

        let size: u32 = size_str.parse().unwrap();
        match unit.as_str() {
            "cm" => Ok(Distance::Centimeters(size)),
            "in" => Ok(Distance::Inches(size)),
            _ => Err(DistanceFromStrErr::InvalidUnit(unit)),
        }
    }
}

#[derive(Debug, Builder)]
struct Passport {
    birth_year: u32,
    issue_year: u32,
    expiration_year: u32,
    height: String,
    hair_colour: String,
    eye_colour: String,
    passport_id: String,
    #[builder(setter(into, strip_option), default)]
    country_id: Option<String>,
}

impl Passport {
    fn valid_birth_year(&self) -> bool {
        self.birth_year >= 1920 && self.birth_year <= 2002
    }

    fn valid_issue_year(&self) -> bool {
        self.issue_year >= 2010 && self.issue_year <= 2020
    }

    fn valid_expiration_year(&self) -> bool {
        self.expiration_year >= 2020 && self.expiration_year <= 2030
    }

    fn valid_height(&self) -> bool {
        match self.height.parse() {
            Ok(Distance::Centimeters(cm)) => cm >= 150 && cm <= 193,
            Ok(Distance::Inches(i)) => i >= 59 && i <= 76,
            _ => false,
        }
    }

    fn valid_hair_colour(&self) -> bool {
        let mut chars = self.hair_colour.chars();
        match chars.next().unwrap() {
            '#' => chars.all(|c| c.is_ascii_hexdigit()),
            _ => false,
        }
    }

    fn valid_eye_colour(&self) -> bool {
        lazy_static! {
            static ref VALID_COLOURS: Vec<&'static str> =
                vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        }

        VALID_COLOURS
            .iter()
            .any(|valid| valid == &self.eye_colour.as_str())
    }

    fn valid_passport_id(&self) -> bool {
        self.passport_id.len() == 9 && self.passport_id.chars().all(|c| c.is_ascii_digit())
    }

    pub fn is_valid(&self) -> bool {
        self.valid_birth_year()
            && self.valid_issue_year()
            && self.valid_expiration_year()
            && self.valid_height()
            && self.valid_hair_colour()
            && self.valid_eye_colour()
            && self.valid_passport_id()
    }

    fn from_iter<'a>(it: &mut impl Iterator<Item = &'a String>) -> Result<Passport, String> {
        let mut builder = PassportBuilder::default();
        for line in it {
            // Pull out the fields, put them in the builder
            line.split_whitespace().for_each(|f| {
                let mut field_iter = f.split(":");
                let key = field_iter.next().unwrap();
                let value = field_iter.next().unwrap();
                match key {
                    "byr" => {
                        builder.birth_year(value.parse().unwrap());
                    }
                    "iyr" => {
                        builder.issue_year(value.parse().unwrap());
                    }
                    "eyr" => {
                        builder.expiration_year(value.parse().unwrap());
                    }
                    "hgt" => {
                        builder.height(value.parse().unwrap());
                    }
                    "hcl" => {
                        builder.hair_colour(value.to_string());
                    }
                    "ecl" => {
                        builder.eye_colour(value.to_string());
                    }
                    "pid" => {
                        builder.passport_id(value.to_string());
                    }
                    "cid" => {
                        builder.country_id(value.to_string());
                    }
                    _ => panic!(format!("I don't know this field: '{}'", key)),
                }
            });
        }
        builder.build()
    }
}

#[derive(Debug)]
struct PassportStore {
    passports: Vec<Passport>,
}

impl PassportStore {
    fn from_iter(
        it: &mut impl Iterator<Item = Result<Vec<String>, ParseLinesError<Infallible>>>,
    ) -> Result<PassportStore, ParseLinesError<Infallible>> {
        let mut passports = Vec::new();
        for record in it {
            let lines = record?;
            if let Ok(passport) = Passport::from_iter(&mut lines.iter()) {
                passports.push(passport);
            }
        }

        return Ok(PassportStore { passports });
    }
}

fn main() {
    let mut record_iter = load_file_records("input.txt", "");
    let store = PassportStore::from_iter(&mut record_iter).expect("Failed to load passport store");

    println!("{}", store.passports.len());

    let valid = store.passports.iter().filter(|p| p.is_valid()).count();
    println!("{}", valid);
}
