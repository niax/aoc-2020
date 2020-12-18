use commons::io::load_file_lines;
use std::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug)]
struct Password {
    min: usize,
    max: usize,
    letter: char,
    string: String,
}

#[derive(Error, Debug)]
enum PasswordParseError {
    #[error("Could not find a field - {0}")]
    MissingField(&'static str),
    #[error("Could not parse an integer - {0:?}")]
    BadData(#[from] ParseIntError),
}

impl FromStr for Password {
    type Err = PasswordParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let s = input.to_string();
        let mut iter = s.split_whitespace();
        let range = iter
            .next()
            .ok_or(PasswordParseError::MissingField("Range"))?;
        let mut range_iter = range.split('-');
        let min = range_iter
            .next()
            .ok_or(PasswordParseError::MissingField("Range min"))?
            .parse()?;
        let max = range_iter
            .next()
            .ok_or(PasswordParseError::MissingField("Range max"))?
            .parse()?;
        let letter = iter
            .next()
            .ok_or(PasswordParseError::MissingField("Letter"))?
            .chars()
            .next()
            .ok_or(PasswordParseError::MissingField("Letter"))?;
        let string = iter.next().unwrap().to_string();

        Ok(Password {
            min,
            max,
            letter,
            string,
        })
    }
}

fn main() {
    let mut part1 = 0;
    let mut part2 = 0;
    for res in load_file_lines::<Password>("input.txt") {
        let l = res.unwrap();

        let letter_count = l.string.chars().filter(|c| *c == l.letter).count();
        if letter_count >= l.min && letter_count <= l.max {
            part1 += 1;
        }
        let first = l.string.chars().nth(l.min - 1);
        let second = l.string.chars().nth(l.max - 1);
        let part2_valid = match (first, second) {
            (Some(c), None) | (None, Some(c)) => c == l.letter,
            (Some(c1), Some(c2)) => (c1 == l.letter || c2 == l.letter) && c1 != c2,
            _ => false,
        };
        if part2_valid {
            part2 += 1;
        }
    }
    println!("{} valid", part1);
    println!("{} valid", part2);
}
