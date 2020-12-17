use commons::io::load_file_lines;
use std::collections::{HashSet, VecDeque};
use std::convert::TryInto;
use std::hash::Hash;

trait Neighbours {
    fn neighbours(&self) -> Vec<Self>
    where
        Self: Sized;
}

type ThreePoint = (i64, i64, i64);

impl Neighbours for ThreePoint {
    fn neighbours(&self) -> Vec<Self> {
        let mut a = Vec::new();
        for x in -1..2 {
            for y in -1..2 {
                for z in -1..2 {
                    if x == 0 && y == 0 && z == 0 {
                        continue;
                    }
                    a.push((self.0 + x, self.1 + y, self.2 + z));
                }
            }
        }
        a
    }
}

type FourPoint = (i64, i64, i64, i64);

impl Neighbours for FourPoint {
    fn neighbours(&self) -> Vec<Self> {
        let mut a = Vec::new();
        for x in -1..2 {
            for y in -1..2 {
                for z in -1..2 {
                    for w in -1..2 {
                        if x == 0 && y == 0 && z == 0 && w == 0 {
                            continue;
                        }
                        a.push((self.0 + x, self.1 + y, self.2 + z, self.3 + w));
                    }
                }
            }
        }
        a
    }
}

struct Grid<T> {
    in_use: HashSet<T>,
}

impl<T> Grid<T>
where
    T: Neighbours + Eq + Hash + Copy,
{
    pub fn new() -> Grid<T> {
        Grid {
            in_use: HashSet::new(),
        }
    }

    pub fn set(&mut self, point: T) {
        self.in_use.insert(point);
    }

    pub fn active_cells(&self) -> usize {
        self.in_use.len()
    }

    pub fn step(&self) -> Self {
        let mut new_grid = Grid::<T>::new();
        let mut seen = HashSet::new();
        let mut queue: VecDeque<T> = self.in_use.iter().map(|x| *x).collect();
        while let Some(point) = queue.pop_front() {
            if seen.contains(&point) {
                continue;
            }
            seen.insert(point);
            let mut active_neighbours = 0;
            for neighbour in point.neighbours() {
                if self.in_use.contains(&neighbour) {
                    active_neighbours += 1;
                }
                if self.in_use.contains(&point) {
                    queue.push_back(neighbour);
                }
            }
            let active = match (self.in_use.contains(&point), active_neighbours) {
                (true, 2) => true,
                (_, 3) => true,
                _ => false,
            };
            if active {
                new_grid.set(point);
            }
        }
        new_grid
    }
}

fn main() {
    let mut part1_grid = Grid::new();
    let mut part2_grid = Grid::new();
    for (x, line) in load_file_lines::<String>("input.txt")
        .map(|res| res.unwrap())
        .enumerate()
    {
        for (y, c) in line.chars().enumerate() {
            if c == '#' {
                part1_grid.set((x.try_into().unwrap(), y.try_into().unwrap(), 0));
                part2_grid.set((x.try_into().unwrap(), y.try_into().unwrap(), 0, 0));
            }
        }
    }

    for _ in 0..6 {
        part1_grid = part1_grid.step();
    }
    println!("{}", part1_grid.active_cells());

    for _ in 0..6 {
        part2_grid = part2_grid.step();
    }
    println!("{}", part2_grid.active_cells());
}
