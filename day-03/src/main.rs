use commons::io::{load_file_lines, FromLines};
use std::str::FromStr;

#[derive(Debug)]
enum Cell {
    Tree,
    Clear,
}

#[derive(Debug)]
struct Row {
    cells: Vec<Cell>,
}

impl FromStr for Row {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let cells: Vec<Cell> = input
            .to_string()
            .chars()
            .map(|c| match c {
                '.' => Cell::Clear,
                '#' => Cell::Tree,
                _ => panic!("I don't know what this is"),
            })
            .collect();
        Ok(Row { cells })
    }
}

struct Grid {
    rows: Vec<Row>,
    width: usize,
}

impl Grid {
    pub fn new(rows: Vec<Row>) -> Grid {
        let width = rows[0].cells.len();
        Grid {
            rows: rows,
            width: width,
        }
    }

    pub fn height(&self) -> usize {
        self.rows.len()
    }

    pub fn at(&self, x: usize, y: usize) -> &Cell {
        let row = &self.rows[y];
        &row.cells[x % self.width]
    }

    pub fn trees_hit(&self, stride: (usize, usize)) -> u32 {
        let mut pos = (0, 0);
        let mut tree_count = 0;
        while pos.1 < self.height() {
            tree_count += match self.at(pos.0, pos.1) {
                Cell::Tree => 1,
                Cell::Clear => 0,
            };
            pos = (pos.0 + stride.0, pos.1 + stride.1)
        }
        tree_count
    }
}

impl FromLines for Grid {
    type Line = Row;

    fn from_lines<I>(lines: &mut I) -> Self
    where
        I: Iterator<Item = Self::Line>,
    {
        let rows = lines.collect();
        Grid::new(rows)
    }
}

fn main() {
    let grid = Grid::new(load_file_lines("input.txt"));

    let part1 = grid.trees_hit((3, 1));
    println!("{}", part1);

    let part2_parts = vec![(1, 1), (5, 1), (7, 1), (1, 2)];
    let part2 = part2_parts
        .iter()
        .map(|stride| grid.trees_hit(*stride))
        .fold(1, |a, b| a * b);
    // Part 2's parts include part 1, but we intentionally don't do it
    // again. Instead, we multiply it here.
    println!("{}", part1 * part2);
}
