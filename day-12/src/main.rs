use commons::geom::Point;
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
    position: Point<i32>,
    waypoint: Point<i32>,
    direction: i32,
}

impl Ship {
    pub fn new() -> Ship {
        Ship {
            position: Point::origin(),
            waypoint: Point::new(10, 1),
            direction: 90,
        }
    }

    pub fn x(&self) -> &i32 {
        self.position.x()
    }

    pub fn y(&self) -> &i32 {
        self.position.y()
    }

    fn update_dir(&mut self, dir: i32) {
        let new_dir = self.direction + dir;
        self.direction = if new_dir < 0 {
            new_dir + 360
        } else {
            new_dir % 360
        }
    }

    pub fn step(&mut self, ins: &Instruction) {
        let dir = match *ins {
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
        self.position += dir;
    }

    pub fn step_waypoint(&mut self, ins: &Instruction) {
        self.waypoint = match *ins {
            Instruction::North(arg) => self.waypoint + (0, arg as i32),
            Instruction::South(arg) => self.waypoint - (0, arg as i32),
            Instruction::East(arg) => self.waypoint + (arg as i32, 0),
            Instruction::West(arg) => self.waypoint - (arg as i32, 0),
            Instruction::Left(arg) => match arg {
                0 => self.waypoint,
                90 => Point::new(-*self.waypoint.y(), *self.waypoint.x()),
                180 => Point::new(-*self.waypoint.x(), -*self.waypoint.y()),
                270 => Point::new(*self.waypoint.y(), -*self.waypoint.x()),
                _ => panic!("Unknown direction: {}", arg),
            },
            Instruction::Right(arg) => match arg {
                0 => self.waypoint,
                90 => Point::new(*self.waypoint.y(), -*self.waypoint.x()),
                180 => Point::new(-*self.waypoint.x(), -*self.waypoint.y()),
                270 => Point::new(-*self.waypoint.y(), *self.waypoint.x()),
                _ => panic!("Unknown direction: {}", arg),
            },
            _ => self.waypoint,
        };
        self.position += match *ins {
            Instruction::Forward(arg) => (
                self.waypoint.x() * arg as i32,
                self.waypoint.y() * arg as i32,
            ),
            _ => (0, 0),
        };
    }
}

fn main() {
    let input =
        load_file_lines::<Instruction>("input.txt").map(|res| res.expect("Failed to load input"));
    let mut part1 = Ship::new();
    let mut part2 = Ship::new();
    for i in input {
        part1.step(&i);
        part2.step_waypoint(&i);
    }
    println!("{}", part1.x().abs() + part1.y().abs());
    println!("{}", part2.x().abs() + part2.y().abs());
}
