use commons::geom::Point;
use commons::grid::{Grid, SparseGrid};
use commons::io::load_file_lines;
use std::collections::{HashSet, VecDeque};

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

    pub fn offsets() -> Vec<(isize, isize)> {
        vec![(-1, 1), (1, 1), (-2, 0), (2, 0), (-1, -1), (1, -1)]
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

    for _ in 0..100 {
        let mut seen = HashSet::new();
        let mut q: VecDeque<(isize, isize)> = grid
            .points()
            .iter()
            .map(|(coord, _)| coord)
            .copied()
            .collect();
        let mut next_grid = SparseGrid::new();
        while !q.is_empty() {
            let coord = q.pop_front().unwrap();
            let cell_is_black = grid.at(&coord).copied().unwrap_or(false);
            let mut black_arround = 0;
            for offset in HexDir::offsets() {
                let surround_coord = (coord.0 + offset.0, coord.1 + offset.1);
                // Queue cells surrounding black cells as they may have changed
                if cell_is_black && !seen.contains(&surround_coord) {
                    seen.insert(surround_coord);
                    q.push_back(surround_coord);
                }
                black_arround += match grid.at(&surround_coord) {
                    Some(true) => 1,
                    _ => 0,
                };
            }

            let new_value = match grid.at(&coord) {
                Some(true) => {
                    // Currently black, so flip depending on rules
                    !(black_arround == 0 || black_arround > 2)
                }
                _ => {
                    // Currently white
                    black_arround == 2
                }
            };

            if new_value {
                next_grid.set(coord.0, coord.1, true);
            }
        }
        grid = next_grid;
    }

    let part2 = grid.points().iter().filter(|(_, v)| **v).count();
    println!("{}", part2);
}
