use commons::io::load_file_lines;
use itertools::rev;
use std::collections::HashMap;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug)]
enum Action {
    SetMask(u64, u64),
    SetValue(usize, u64),
}

#[derive(Error, Debug)]
enum ParseError {
    #[error("Bad mask!")]
    BadAction,
    #[error("Bad mask!")]
    BadMask,
    #[error("Bad cell!")]
    BadCell,
    #[error("Bad value!")]
    BadValue,
}

impl FromStr for Action {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        let action = parts[0];
        if action == "mask" {
            let mut set_bits = 0;
            let mut clear_bits = 0;
            // Parse the mask
            for (i, bit) in rev(parts[2].chars()).enumerate() {
                let bitmask = 1 << i;
                match bit {
                    '1' => {
                        set_bits |= bitmask;
                    }
                    '0' => {
                        clear_bits |= bitmask;
                    }
                    'X' => {}
                    _ => return Err(ParseError::BadMask),
                }
            }
            Ok(Action::SetMask(set_bits, clear_bits))
        } else if &action[0..4] == "mem[" {
            let cell_num_str: String = action
                .chars()
                .skip(4)
                .take_while(|c| c.is_numeric())
                .collect();
            let cell_num = cell_num_str.parse().map_err(|_| ParseError::BadCell)?;
            let value = parts[2].parse().map_err(|_| ParseError::BadValue)?;
            Ok(Action::SetValue(cell_num, value))
        } else {
            Err(ParseError::BadAction)
        }
    }
}

fn main() {
    let input =
        load_file_lines::<Action>("input.txt").map(|res| res.expect("Failed to read input"));

    let mut set_bits = 0;
    let mut clear_bits = 0;
    let mut memory: HashMap<usize, u64> = HashMap::new();
    for i in input {
        match i {
            Action::SetMask(set, clear) => {
                set_bits = set;
                clear_bits = clear
            }
            Action::SetValue(cell, val) => {
                let val = val | set_bits;
                let val = val & !clear_bits;
                memory.insert(cell, val);
            }
        }
    }
    println!("{}", memory.values().sum::<u64>());
}
