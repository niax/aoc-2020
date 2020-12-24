use commons::io::load_file_lines;
use commons::geom::Point;
use commons::grid::{Grid, SparseGrid};

#[derive(Debug)]
enum HexDir {
    NorthWest,
    NorthEast,
    West,
    East,
    SouthWest,
    SouthEast,
}

impl HexDir {
    pub fn parse_line(s: &str) -> Vec<HexDir> {
        let mut chars = s.chars();
        let mut res = Vec::new();
        loop {
            let dir = match chars.next() {
                Some('n') => match chars.next() {
                    Some('w') => HexDir::NorthWest,
                    Some('e') => HexDir::NorthEast,
                    _ => panic!("Wanted character, but not found"),
                },
                Some('s') => match chars.next() {
                    Some('w') => HexDir::SouthWest,
                    Some('e') => HexDir::SouthEast,
                    _ => panic!("Wanted character, but not found"),
                },
                Some('w') => HexDir::West,
                Some('e') => HexDir::East,
                None => break,
                _ => panic!("Wanted character, but not found"),
            };
            res.push(dir);
        }

        res
    }

    pub fn offset(&self) -> (isize, isize) {
        match self {
            HexDir::NorthWest => (-1, 1),
            HexDir::NorthEast => (1, 1),
            HexDir::West => (-2, 0),
            HexDir::East => (2, 0),
            HexDir::SouthWest => (-1, -1),
            HexDir::SouthEast => (1, -1),
        }
    }
}

fn main() {
    let input_strs = load_file_lines::<String>("input.txt").map(|res| res.unwrap());
    let input = input_strs.map(|s| HexDir::parse_line(&s));
    let mut grid: SparseGrid<bool> = SparseGrid::new();

    for line in input {
        let mut coord = Point::origin();
        for dir in line {
            coord += dir.offset();
        }
        let x = *coord.x();
        let y = *coord.y();
        let current = match grid.at(&(x, y)) {
            Some(a) => *a,
            None => false,
        };
        grid.set(x, y, !current);
    }

    let part1 = grid.points().iter().filter(|(_, v)| **v).count();
    println!("{}", part1);
}
