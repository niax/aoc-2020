use commons::io::load_file_lines;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug)]
enum Cell {
    Tree,
    Clear,
}

#[derive(Debug)]
struct Row {
    cells: Vec<Cell>,
}

#[derive(Error, Debug)]
pub enum RowParseError {
    #[error("Unknown character '{0}'")]
    BadChar(char),
}

impl FromStr for Row {
    type Err = RowParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut cells: Vec<Cell> = Vec::new();

        for c in input.to_string().chars() {
            let cell = match c {
                '.' => Ok(Cell::Clear),
                '#' => Ok(Cell::Tree),
                _ => Err(RowParseError::BadChar(c)),
            }?;
            cells.push(cell)
        }
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

    pub fn from_iter<E>(it: impl Iterator<Item = Result<Row, E>>) -> Result<Grid, E> {
        let mut rows = Vec::new();
        for res in it {
            let row = res?;
            rows.push(row);
        }

        Ok(Grid::new(rows))
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

fn main() {
    let row_iter = load_file_lines::<Row>("input.txt");
    let grid = Grid::from_iter(row_iter).unwrap();

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
