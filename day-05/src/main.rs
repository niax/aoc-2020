use commons::io::load_file_lines;
use std::cmp::Ordering;
use std::convert::Infallible;
use std::str::FromStr;

#[derive(Debug, Eq, Copy, Clone)]
struct SeatLocator {
    row: u16,
    column: u16,
}

impl SeatLocator {
    pub fn seat_id(&self) -> u32 {
        ((self.row as u32) * 8) + (self.column as u32)
    }
}

impl Ord for SeatLocator {
    fn cmp(&self, other: &Self) -> Ordering {
        let row_cmp = self.row.cmp(&other.row);
        if row_cmp == Ordering::Equal {
            self.column.cmp(&other.column)
        } else {
            row_cmp
        }
    }
}

impl PartialOrd for SeatLocator {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for SeatLocator {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl FromStr for SeatLocator {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut pos: u16 = 0;

        for c in input.chars() {
            pos = pos << 1;
            pos += match c {
                'B' | 'R' => 1,
                _ => 0,
            }
        }

        let row = (pos & 0x3f8) >> 3;
        let column = pos & 0x7;

        Ok(SeatLocator { row, column })
    }
}

fn main() {
    let mut seats: Vec<SeatLocator> = load_file_lines("input.txt").map(|r| r.unwrap()).collect();
    seats.sort();

    let max_seat = seats[seats.len() - 1];
    println!("{:?}", max_seat.seat_id());

    let mut last_id = seats[0].seat_id();
    for seat in seats[1..].iter() {
        let id = seat.seat_id();
        if id == last_id + 2 {
            println!("{}", id - 1);
            break;
        }
        last_id = id;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        input: &'static str,
        row: u16,
        column: u16,
        seat_id: u32,
    }
    #[test]
    fn parse() {
        let cases = vec![
            TestCase {
                input: "FBFBBFFRLR",
                row: 44,
                column: 5,
                seat_id: 357,
            },
            TestCase {
                input: "BFFFBBFRRR",
                row: 70,
                column: 7,
                seat_id: 567,
            },
            TestCase {
                input: "FFFBBBFRRR",
                row: 14,
                column: 7,
                seat_id: 119,
            },
            TestCase {
                input: "BBFFBBFRLL",
                row: 102,
                column: 4,
                seat_id: 820,
            },
        ];

        for case in cases {
            let seat: SeatLocator = case.input.parse().unwrap();
            assert_eq!(case.row, seat.row);
            assert_eq!(case.column, seat.column);
            assert_eq!(case.seat_id, seat.seat_id());
        }
    }
}
