use std::str::FromStr;
use commons::io::load_file_lines;

#[derive(Debug)]
struct Line {
    min: usize,
    max: usize,
    letter: char,
    string: String,
}

impl FromStr for Line {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let s = input.to_string();
        let mut iter = s.split_whitespace();
        let range = iter.next().expect("range");
        let mut range_iter = range.split("-");
        let min = range_iter.next().expect("min - field").parse().expect("min - parse");
        let max = range_iter.next().expect("max - field").parse().expect("max - parse");
        let letter = iter.next().unwrap().chars().next().unwrap(); // Ewww
        let string = iter.next().unwrap().to_string();

        Ok(Line {
            min, max, letter, string
        })
    }
}

fn main() {
    let lines: Vec<Line> = load_file_lines("input.txt");
    let mut part1 = 0;
    let mut part2 = 0;
    for l in lines {
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
