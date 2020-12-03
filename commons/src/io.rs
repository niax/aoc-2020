use std::fmt::Debug;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

pub fn load_file_lines<T>(path: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let mut file = File::open(path).expect("Could not open input file");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Could not read input file");
    content
        .lines()
        .map(|l| match T::from_str(l) {
            Ok(i) => i,
            Err(e) => panic!("Couldn't parse '{}' => {:?}", l, e),
        })
        .collect()
}
