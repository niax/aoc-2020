use bitvec::prelude::*;
use commons::io::load_file_records;
use multimap::MultiMap;

#[derive(Debug)]
struct Tile {
    id: u64,
    rows: Vec<BitVec>,
}

impl Tile {
    pub fn edges(&self) -> Vec<BitVec> {
        let mut unflipped = Vec::with_capacity(4);

        unflipped.push(self.rows[0].clone());
        unflipped.push(self.rows[self.rows.len() - 1].clone());
        let mut left = BitVec::<Lsb0, usize>::with_capacity(self.rows.len());
        let mut right = BitVec::<Lsb0, usize>::with_capacity(self.rows.len());
        for r in &self.rows {
            left.push(r[0]);
            right.push(r[r.len() - 1]);
        }
        unflipped.push(left);
        unflipped.push(right);

        let mut edges = Vec::new();
        for edge in unflipped {
            let mut rev = edge.clone();
            rev.reverse();
            if edge[0..edge.len()].load::<u16>() > rev[0..edge.len()].load::<u16>() {
                edges.push(rev);
            } else {
                edges.push(edge);
            }
        }

        edges
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
        for edge in tile.edges() {
            edge_owners.insert(edge, tile);
        }
    }

    let mut corners = Vec::new();
    for tile in &tiles {
        let common: usize = tile
            .edges()
            .iter()
            .map(|edge| edge_owners.get_vec(edge).unwrap().len() - 1)
            .sum();
        if common == 2 {
            corners.push(tile);
        }
    }

    let part1: u64 = corners.iter().map(|tile| tile.id).product();
    println!("{}", part1);
}
