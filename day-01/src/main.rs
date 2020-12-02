use std::fmt::Debug;
use std::fs::File;
use std::str::FromStr;
use std::io::prelude::*;

fn load_file_lines<T>(path: &str) -> Vec<T> 
        where T: FromStr, <T as FromStr>::Err : Debug {
    let mut file = File::open(path).expect("Could not open input file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Could not read input file");
    content.lines().map(|l| {
        match T::from_str(l) {
            Ok(i) => i,
            Err(e) => panic!("Couldn't parse '{}' => {:?}", l, e),
        }
    }).collect()
}


fn main() {
    let mut ints: Vec<u32> = load_file_lines("input.txt");
    ints.sort();
    let mut lower_idx = 0;
    let mut upper_idx = ints.len() - 1;
    loop {
        let lower = ints[lower_idx];
        let upper = ints[upper_idx];

        let sum = lower + upper;
        if sum == 2020 {
            println!("{} * {} = {}", lower, upper, lower*upper);
            break;
        } else if sum < 2020 {
            // Move the lower bound up to make bigger numbers
            lower_idx += 1;
        } else {
            // Move the upper bound down to make smaller numbers
            upper_idx -= 1;
        }
    }
}
