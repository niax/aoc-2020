use bitvec::prelude::*;
use commons::io::load_file_records;
use multimap::MultiMap;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone)]
struct Tile {
    id: u64,
    rows: Vec<BitVec>,
}

impl Tile {
    fn edges_and_flipped(&self) -> Vec<(BitVec, BitVec)> {
        let unflipped = self.edges();
        let mut edge_pairs = Vec::with_capacity(4);
        for edge in unflipped {
            let mut rev = edge.clone();
            rev.reverse();
            edge_pairs.push((edge, rev));
        }

        edge_pairs
    }

    pub fn edges(&self) -> Vec<BitVec> {
        let mut right = BitVec::<Lsb0, usize>::with_capacity(self.rows.len());
        let mut left = BitVec::<Lsb0, usize>::with_capacity(self.rows.len());
        for r in &self.rows {
            right.push(r[r.len() - 1]);
            left.push(r[0]);
        }

        vec![
            self.rows[0].clone(),
            right,
            self.rows[self.rows.len() - 1].clone(),
            left,
        ]
    }

    pub fn normalized_edges(&self) -> Vec<BitVec> {
        self.edges_and_flipped()
            .iter()
            .map(|(edge, rev)| {
                if edge[0..edge.len()].load::<u16>() > rev[0..edge.len()].load::<u16>() {
                    rev.clone()
                } else {
                    edge.clone()
                }
            })
            .collect()
    }

    fn remap<F>(&self, coord_mapper: F) -> Tile
    where
        F: Fn(usize, usize) -> (usize, usize),
    {
        let mut new_rows = Vec::with_capacity(self.rows.len());
        for _ in 0..self.rows.len() {
            let mut new_row = BitVec::with_capacity(self.rows.len());
            new_row.resize(self.rows.len(), false);
            new_rows.push(new_row);
        }

        for (y, row) in self.rows.iter().enumerate() {
            for (x, bit) in row.iter().enumerate() {
                let (new_x, new_y) = coord_mapper(x, y);
                new_rows[new_y].as_mut_bitslice().set(new_x, *bit);
            }
        }

        Tile {
            id: self.id,
            rows: new_rows,
        }
    }

    pub fn rotate_left(&self) -> Tile {
        self.remap(|x, y| (y, self.rows.len() - 1 - x))
    }

    pub fn flip_vertical(&self) -> Tile {
        self.remap(|x, y| (x, self.rows.len() - 1 - y))
    }

    pub fn flip_horizontal(&self) -> Tile {
        self.remap(|x, y| (self.rows[0].len() - 1 - x, y))
    }

    pub fn fits_with(&self, other: &Tile, dir: usize) -> bool {
        let self_edges = self.edges();
        let other_edges = other.edges();
        let other_dir = match dir {
            0 => 2,
            1 => 3,
            2 => 0,
            3 => 1,
            _ => panic!("Bad dir!"),
        };
        self_edges[dir] == other_edges[other_dir]
    }

    pub fn combinations(&self) -> Vec<Tile> {
        let flipped = vec![self.clone(), self.flip_vertical(), self.flip_horizontal()];

        let mut l = Vec::new();

        for flip in flipped {
            let mut cur = flip.clone();
            for _ in 0..4 {
                l.push(cur.clone());
                cur = cur.rotate_left();
            }
        }

        l
    }

    pub fn print_pattern(&self) {
        for r in &self.rows {
            println!(
                "{}",
                r.iter()
                    .map(|x| if *x { '#' } else { '.' })
                    .collect::<String>()
            );
        }
    }
}

fn main() {
    let records = load_file_records::<String>("input.txt", "").map(|res| res.unwrap());
    let mut tiles = Vec::new();
    for r in records {
        let id_str: String = r[0].chars().filter(|c| c.is_numeric()).collect();
        let rows = r[1..]
            .iter()
            .map(|l| {
                let mut v = BitVec::with_capacity(l.len());
                for c in l.chars() {
                    v.push(c == '#');
                }
                v
            })
            .collect();
        let tile = Tile {
            id: id_str.parse().unwrap(),
            rows,
        };
        tiles.push(tile);
    }

    let mut edge_owners = MultiMap::new();
    for tile in &tiles {
        for edge in tile.normalized_edges() {
            edge_owners.insert(edge, tile);
        }
    }

    let mut corners = Vec::new();
    for tile in &tiles {
        let common: usize = tile
            .normalized_edges()
            .iter()
            .map(|edge| edge_owners.get_vec(edge).unwrap().len() - 1)
            .sum();
        if common == 2 {
            corners.push(tile);
        }
    }

    let part1: u64 = corners.iter().map(|tile| tile.id).product();
    println!("{}", part1);

    let mut next_cells: VecDeque<(i64, i64)> = VecDeque::new();
    let mut grid: HashMap<(i64, i64), Tile> = HashMap::new();
    let mut used_ids = HashSet::new();

    grid.insert((0, 0), corners[0].clone());
    next_cells.push_back((0, 0));
    used_ids.insert(corners[0].id);

    let dirs = vec![(0, 1, 2), (1, 0, 1), (0, -1, 0), (-1, 0, 3)];

    while !next_cells.is_empty() {
        let cell = next_cells.pop_front().unwrap();

        for dir in &dirs {
            let adj_cell = (cell.0 + dir.0, cell.1 + dir.1);
            if grid.contains_key(&adj_cell) {
                continue;
            }

            let adj_to_new: Vec<(&usize, Tile)> = dirs
                .iter()
                .map(|(x, y, d)| {
                    let more_adj_cell = (adj_cell.0 + x, adj_cell.1 + y);
                    (d, grid.get(&more_adj_cell))
                })
                .filter(|(_, o)| o.is_some())
                .map(|(d, o)| (d, o.unwrap().clone()))
                .collect();

            for other_tile in &tiles {
                if used_ids.contains(&other_tile.id) {
                    continue;
                }
                for comb in other_tile.combinations() {
                    let fits = adj_to_new
                        .iter()
                        .all(|(dir, adj_tile)| comb.fits_with(adj_tile, **dir));
                    if fits {
                        grid.insert(adj_cell, comb.clone());
                        next_cells.push_back(adj_cell);
                        used_ids.insert(other_tile.id);
                        break;
                    }
                }
            }
        }
    }

    let min_x = grid.keys().map(|x| x.0).min().unwrap();
    let max_x = grid.keys().map(|x| x.0).max().unwrap();
    let min_y = grid.keys().map(|x| x.1).min().unwrap();
    let max_y = grid.keys().map(|x| x.1).max().unwrap();
    let mut final_rows = Vec::new();
    for tile_y in min_y..=max_y {
        let mut block_rows = Vec::new();
        for y in 1..(corners[0].rows.len() - 1) {
            let mut block_row = BitVec::<Lsb0, usize>::new();

            for tile_x in min_x..=max_x {
                let tile = grid.get(&(tile_x, tile_y)).unwrap();
                block_row.extend(&tile.rows[y][1..tile.rows[y].len() - 1]);
            }

            block_rows.push(block_row);
        }
        final_rows.extend(block_rows);
    }

    let combined = Tile {
        id: 0,
        rows: final_rows
            .iter()
            .filter(|x| !x.is_empty())
            .cloned()
            .collect(),
    };

    let monster = vec![
        (0, 1),
        (1, 2),
        (4, 2),
        (5, 1),
        (6, 1),
        (7, 2),
        (10, 2),
        (11, 1),
        (12, 1),
        (13, 2),
        (16, 2),
        (17, 1),
        (18, 1),
        (18, 0),
        (19, 1),
    ];

    for opt in combined.combinations() {
        let mut monster_cells = Vec::new();
        for x in 0..(opt.rows[0].len() - 19) {
            for y in 0..(opt.rows.len() - 3) {
                let potential_monster: Vec<(usize, usize)> =
                    monster.iter().map(|(mx, my)| (x + mx, y + my)).collect();
                let monster_here = potential_monster.iter().all(|(mx, my)| opt.rows[*my][*mx]);

                if monster_here {
                    monster_cells.extend(potential_monster);
                }
            }
        }

        if !monster_cells.is_empty() {
            let monster_cell_count = monster_cells.len();
            let active_cell_count = opt
                .rows
                .iter()
                .flat_map(|row| row.iter())
                .filter(|x| **x)
                .count();
            println!("{}", active_cell_count - monster_cell_count);
            break;
        }
    }
}
