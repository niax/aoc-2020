use commons::io::load_file_lines;
use lazy_static::lazy_static;
use std::collections::{HashSet, VecDeque};
use std::convert::TryInto;
use std::fmt;

type ThreePoint = (i64, i64, i64);

lazy_static! {
    static ref ADJACENT: Vec<ThreePoint> = {
        let mut a = Vec::new();
        for x in -1..2 {
            for y in -1..2 {
                for z in -1..2 {
                    if x == 0 && y == 0 && z == 0 {
                        continue;
                    }
                    a.push((x, y, z));
                }
            }
        }
        a
    };
}

struct Grid {
    in_use: HashSet<ThreePoint>,
}

impl Grid {
    pub fn new() -> Grid {
        Grid {
            in_use: HashSet::new(),
        }
    }

    fn set(&mut self, point: ThreePoint) {
        self.in_use.insert(point);
    }

    pub fn active_cells(&self) -> usize {
        self.in_use.len()
    }

    pub fn set_point(&mut self, x: i64, y: i64, z: i64) {
        self.set((x, y, z));
    }

    pub fn step(&self) -> Grid {
        let mut new_grid = Grid::new();
        let mut seen = HashSet::new();
        let mut queue: VecDeque<ThreePoint> = self.in_use.iter().map(|x| *x).collect();
        while let Some(point) = queue.pop_front() {
            if seen.contains(&point) {
                continue;
            }
            seen.insert(point);
            let (x, y, z) = point;
            let mut active_neighbours = 0;
            for offset in ADJACENT.iter() {
                let (x_off, y_off, z_off) = offset;
                let neighbour = (x + x_off, y + y_off, z + z_off);
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

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let min_x = self.in_use.iter().map(|x| x.0).min().unwrap();
        let max_x = self.in_use.iter().map(|x| x.0).max().unwrap();
        let min_y = self.in_use.iter().map(|x| x.1).min().unwrap();
        let max_y = self.in_use.iter().map(|x| x.1).max().unwrap();
        let min_z = self.in_use.iter().map(|x| x.2).min().unwrap();
        let max_z = self.in_use.iter().map(|x| x.2).max().unwrap();

        for z in min_z..max_z + 1 {
            f.write_str(format!("z={}\n", z).as_str())?;
            for x in min_x..max_x + 1 {
                for y in min_y..max_y + 1 {
                    let c = match self.in_use.contains(&(x, y, z)) {
                        true => '#',
                        false => '.',
                    };
                    f.write_str(format!("{}", c).as_str())?;
                }
                f.write_str("\n")?;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

fn main() {
    let mut grid = Grid::new();
    for (x, line) in load_file_lines::<String>("input.txt")
        .map(|res| res.unwrap())
        .enumerate()
    {
        for (y, c) in line.chars().enumerate() {
            if c == '#' {
                grid.set_point(x.try_into().unwrap(), y.try_into().unwrap(), 0);
            }
        }
    }

    for _ in 0..6 {
        grid = grid.step();
    }
    println!("{}", grid.active_cells());
}
