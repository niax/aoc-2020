use std::fmt::Debug;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

pub trait FromLines {
    type Line: FromStr;

    fn parse_lines(input: impl Iterator<Item = String>) -> Self
    where
        Self: Sized,
        <<Self as FromLines>::Line as FromStr>::Err: Debug,
    {
        let mut line_iterator = input.map(|l| match Self::Line::from_str(l.as_str()) {
            Ok(i) => i,
            Err(e) => panic!("Couldn't parse '{}' => {:?}", l, e),
        });
        Self::from_lines(&mut line_iterator)
    }

    fn from_lines<T>(lines: &mut T) -> Self
    where
        Self: Sized,
        T: Iterator<Item = Self::Line>;
}

impl<T: FromStr> FromLines for Vec<T> {
    type Line = T;

    fn from_lines<I>(lines: &mut I) -> Self
    where
        I: Iterator<Item = T>,
    {
        lines.collect()
    }
}

pub fn load_file_lines<T>(path: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    load_file(path)
}

pub fn load_file<T>(path: &str) -> T
where
    T: FromLines,
    <<T as FromLines>::Line as FromStr>::Err: Debug,
{
    let file = File::open(path).expect("Could not open input file");
    let reader = BufReader::new(file);
    parse_lines(reader.lines().map(|res| res.unwrap()))
}

pub fn parse_lines<T>(input: impl Iterator<Item = String>) -> T
where
    T: FromLines,
    <<T as FromLines>::Line as FromStr>::Err: Debug,
{
    T::parse_lines(input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn items() {
        let input = "1\n2\n3\n4";
        let output = super::parse_lines::<Vec<u32>>(&input.to_string());
        assert_eq!(vec![1, 2, 3, 4], output);
    }
}
