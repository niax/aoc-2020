use commons::io::load_file_lines;
use itertools::rev;
use std::collections::HashMap;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug)]
enum Action {
    SetMask(u64, u64, u64),
    SetValue(usize, u64),
}

#[derive(Error, Debug)]
enum ParseError {
    #[error("Bad mask!")]
    Action,
    #[error("Bad mask!")]
    Mask,
    #[error("Bad cell!")]
    Cell,
    #[error("Bad value!")]
    Value,
}

impl FromStr for Action {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        let action = parts[0];
        if action == "mask" {
            let mut set_bits = 0;
            let mut clear_bits = 0;
            let mut float_bits = 0;
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
                    'X' => {
                        float_bits |= bitmask;
                    }
                    _ => return Err(ParseError::Mask),
                }
            }
            Ok(Action::SetMask(set_bits, clear_bits, float_bits))
        } else if &action[0..4] == "mem[" {
            let cell_num_str: String = action
                .chars()
                .skip(4)
                .take_while(|c| c.is_numeric())
                .collect();
            let cell_num = cell_num_str.parse().map_err(|_| ParseError::Cell)?;
            let value = parts[2].parse().map_err(|_| ParseError::Value)?;
            Ok(Action::SetValue(cell_num, value))
        } else {
            Err(ParseError::Action)
        }
    }
}

fn run_part1(input: &[Action]) -> HashMap<usize, u64> {
    let mut set_bits = 0;
    let mut clear_bits = 0;
    let mut memory: HashMap<usize, u64> = HashMap::new();
    for i in input {
        match i {
            Action::SetMask(set, clear, _) => {
                set_bits = *set;
                clear_bits = *clear
            }
            Action::SetValue(cell, val) => {
                let val = *val | set_bits;
                let val = val & !clear_bits;
                memory.insert(*cell, val);
            }
        }
    }

    memory
}

fn run_part2(input: &[Action]) -> HashMap<usize, u64> {
    let mut set_bits = 0;
    let mut floating_bits = 0;
    let mut memory: HashMap<usize, u64> = HashMap::new();
    for i in input {
        match i {
            Action::SetMask(set, _, floating) => {
                set_bits = *set as usize;
                floating_bits = *floating as usize;
            }
            Action::SetValue(cell, val) => {
                let cell = cell | set_bits;
                let mut cells = Vec::new();
                for i in 0..36 {
                    let bit = floating_bits & (1 << i);
                    if bit != 0 {
                        let mut new_cells = cells.clone();
                        for &previous_cell in &cells {
                            new_cells.push(previous_cell | bit);
                            new_cells.push(previous_cell & !bit);
                        }
                        // Add new values for just this bit
                        new_cells.push(cell | bit);
                        new_cells.push(cell & !bit);
                        cells = new_cells;
                    }
                }
                for cell in cells {
                    memory.insert(cell, *val);
                }
            }
        }
    }

    memory
}

fn main() {
    let input: Vec<Action> = load_file_lines("input.txt")
        .map(|res| res.expect("Failed to read input"))
        .collect();

    let part1 = run_part1(&input);
    println!("{}", part1.values().sum::<u64>());

    let part2 = run_part2(&input);
    println!("{}", part2.values().sum::<u64>());
}
