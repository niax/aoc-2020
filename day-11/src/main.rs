use commons::io::load_file_lines;
use lazy_static::lazy_static;
use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq)]
enum Position {
    Floor,
    Empty,
    Occupied,
}

impl Position {
    pub fn from_char(c: char) -> Result<Position, char> {
        Ok(match c {
            '.' => Position::Floor,
            'L' => Position::Empty,
            '#' => Position::Occupied,
            _ => return Err(c),
        })
    }

    pub fn char(&self) -> char {
        match self {
            Position::Floor => '.',
            Position::Empty => 'L',
            Position::Occupied => '#',
        }
    }

    pub fn is_seat(&self) -> bool {
        match self {
            Position::Floor => false,
            Position::Empty | Position::Occupied => true,
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
struct SeatPlan {
    seats: Vec<Vec<Position>>,
    width: usize,
}

lazy_static! {
    static ref AROUND: Vec<(i32, i32)> = vec![
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];
}

impl SeatPlan {
    pub fn new(seats: Vec<Vec<Position>>) -> SeatPlan {
        let width = seats[0].len();
        SeatPlan { seats, width }
    }

    fn at(&self, x: i32, y: i32) -> Option<&Position> {
        if x < 0 || y < 0 {
            None
        } else {
            self.seats
                .get(y as usize)
                .map(|row| row.get(x as usize))
                .flatten()
        }
    }

    fn around(&self, x: usize, y: usize) -> Vec<&Position> {
        AROUND
            .iter()
            .map(|offset| {
                let (x_off, y_off) = offset;
                let new_x = x as i32 + x_off;
                let new_y = y as i32 + y_off;
                self.at(new_x, new_y)
            })
            .filter(|opt| opt.is_some())
            .map(|opt| opt.unwrap())
            .collect()
    }

    fn first_seen(&self, x: usize, y: usize) -> Vec<&Position> {
        AROUND
            .iter()
            .map(|gradient| {
                let (x_grad, y_grad) = gradient;
                let mut new_x = x as i32 + x_grad;
                let mut new_y = y as i32 + y_grad;
                loop {
                    let seat_opt = self.at(new_x, new_y);
                    let seat = match seat_opt {
                        Some(seat) => seat,
                        None => return None, // We ran off the world
                    };

                    if seat.is_seat() {
                        return Some(seat);
                    }
                    new_x += x_grad;
                    new_y += y_grad;
                }
            })
            .filter(|opt| opt.is_some())
            .map(|opt| opt.unwrap())
            .collect()
    }

    pub fn step(&self) -> SeatPlan {
        let mut new_rows = Vec::new();
        for (y, row) in self.seats.iter().enumerate() {
            let mut new_row = Vec::new();
            for (x, seat) in row.iter().enumerate() {
                let around = self.around(x, y);
                let occupied_count = around.iter().filter(|s| ***s == Position::Occupied).count();
                let new_seat = match seat {
                    Position::Empty => {
                        if occupied_count > 0 {
                            Position::Empty
                        } else {
                            Position::Occupied
                        }
                    }
                    Position::Occupied => {
                        if occupied_count >= 4 {
                            Position::Empty
                        } else {
                            Position::Occupied
                        }
                    }
                    Position::Floor => Position::Floor,
                };
                new_row.push(new_seat);
            }
            new_rows.push(new_row);
        }

        SeatPlan::new(new_rows)
    }

    pub fn step_part2(&self) -> SeatPlan {
        let new_rows = self
            .seats
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, seat)| {
                        let around = self.first_seen(x, y);
                        let occupied_count =
                            around.iter().filter(|s| ***s == Position::Occupied).count();
                        match seat {
                            Position::Empty => {
                                if occupied_count > 0 {
                                    Position::Empty
                                } else {
                                    Position::Occupied
                                }
                            }
                            Position::Occupied => {
                                if occupied_count >= 5 {
                                    Position::Empty
                                } else {
                                    Position::Occupied
                                }
                            }
                            Position::Floor => Position::Floor,
                        }
                    })
                    .collect()
            })
            .collect();

        SeatPlan::new(new_rows)
    }
}

impl fmt::Display for SeatPlan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.seats {
            for seat in row {
                write!(f, "{}", seat.char())?;
            }
            write!(f, "\n")?;
        }

        fmt::Result::Ok(())
    }
}

fn converge<F>(in_plan: &SeatPlan, step: F) -> usize
where
    F: Fn(&SeatPlan) -> SeatPlan,
{
    let mut plan = in_plan.clone();
    loop {
        println!("{}", plan);
        let new_plan = step(&plan);
        if new_plan == plan {
            break;
        }
        plan = new_plan;
    }
    plan.seats
        .iter()
        .map(|row| {
            row.iter()
                .filter(|seat| **seat == Position::Occupied)
                .count()
        })
        .sum()
}

fn main() {
    let plan = SeatPlan::new(
        load_file_lines::<String>("input.txt")
            .map(|res| {
                res.unwrap()
                    .chars()
                    .map(|c| Position::from_char(c).unwrap())
                    .collect()
            })
            .collect(),
    );

    let part1 = converge(&plan, |p| p.step());
    println!("{}", part1);

    let part2 = converge(&plan, |p| p.step_part2());
    println!("{}", part2);
}
