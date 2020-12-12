use commons::io::load_file_lines;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug)]
enum Instruction {
    North(u32),
    South(u32),
    East(u32),
    West(u32),
    Left(u32),
    Right(u32),
    Forward(u32),
}

#[derive(Debug, Error)]
enum ParseError {
    #[error("Unknown instruction character")]
    UnknownChar,
    #[error("Couldn't parse argument")]
    BadArg,
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let arg = s[1..].parse().map_err(|_| ParseError::BadArg)?;
        match &s[0..1] {
            "N" => Ok(Instruction::North(arg)),
            "S" => Ok(Instruction::South(arg)),
            "E" => Ok(Instruction::East(arg)),
            "W" => Ok(Instruction::West(arg)),
            "L" => Ok(Instruction::Left(arg)),
            "R" => Ok(Instruction::Right(arg)),
            "F" => Ok(Instruction::Forward(arg)),
            _ => Err(ParseError::UnknownChar),
        }
    }
}

#[derive(Debug)]
struct Ship {
    x: i32,
    y: i32,
    direction: i32,
}

impl Ship {
    pub fn new() -> Ship {
        Ship {
            x: 0,
            y: 0,
            direction: 90,
        }
    }

    fn update_dir(&mut self, dir: i32) {
        let new_dir = self.direction + dir;
        self.direction = if new_dir < 0 {
            new_dir + 360
        } else {
            new_dir % 360
        }
    }

    pub fn step(&mut self, ins: Instruction) {
        let dir = match ins {
            Instruction::North(arg) => (0, arg as i32),
            Instruction::South(arg) => (0, -(arg as i32)),
            Instruction::East(arg) => (arg as i32, 0),
            Instruction::West(arg) => (-(arg as i32), 0),
            Instruction::Forward(arg) => {
                let dir = match self.direction {
                    0 => (0, 1),
                    90 => (1, 0),
                    180 => (0, -1),
                    270 => (-1, 0),
                    _ => panic!("Unknown direction: {}", self.direction),
                };
                (dir.0 * arg as i32, dir.1 * arg as i32)
            }
            Instruction::Left(arg) => {
                self.update_dir(-(arg as i32));
                (0, 0)
            }
            Instruction::Right(arg) => {
                self.update_dir(arg as i32);
                (0, 0)
            }
        };
        self.x += dir.0;
        self.y += dir.1;
    }
}

fn main() {
    let input =
        load_file_lines::<Instruction>("input.txt").map(|res| res.expect("Failed to load input"));
    let mut ship = Ship::new();
    for i in input {
        println!("{:?}", i);
        ship.step(i);
        println!("{:?}", ship);
    }
    println!("{}", ship.x.abs() + ship.y.abs());
}
