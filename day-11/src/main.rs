use commons::io::load_file_lines;
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
}

#[derive(PartialEq, Eq)]
struct SeatPlan {
    seats: Vec<Vec<Position>>,
}

impl SeatPlan {
    fn at(&self, x: usize, y: usize) -> Option<&Position> {
        self.seats.get(y).map(|row| row.get(x)).flatten()
    }

    fn around(&self, x: usize, y: usize) -> Vec<&Position> {
        vec![
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ]
        .iter()
        .map(|offset| {
            let (x_off, y_off) = offset;
            let new_x = x as i32 + x_off;
            let new_y = y as i32 + y_off;
            if new_x < 0 || new_y < 0 {
                None
            } else {
                self.at(new_x as usize, new_y as usize)
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

        SeatPlan { seats: new_rows }
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

fn main() {
    let lines = load_file_lines::<String>("input.txt")
        .map(|res| {
            res.unwrap()
                .chars()
                .map(|c| Position::from_char(c).unwrap())
                .collect()
        })
        .collect();
    let mut plan = SeatPlan { seats: lines };

    loop {
        let new_plan = plan.step();
        if new_plan == plan {
            break;
        }
        plan = new_plan;
    }
    let occupied: usize = plan
        .seats
        .iter()
        .map(|row| {
            row.iter()
                .filter(|seat| **seat == Position::Occupied)
                .count()
        })
        .sum();
    println!("{}", occupied);
}
