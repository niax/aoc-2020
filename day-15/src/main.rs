use commons::io::load_file_lines;
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

#[derive(Debug)]
struct MostRecent<T> {
    q: VecDeque<T>,
    n: usize,
}

impl<T> MostRecent<T> {
    pub fn new(n: usize) -> MostRecent<T> {
        MostRecent {
            q: VecDeque::with_capacity(n + 1),
            n,
        }
    }

    pub fn push(&mut self, val: T) {
        self.q.push_front(val);
        // Get us back to where we want to be
        while self.q.len() > self.n {
            self.q.pop_back();
        }
    }

    pub fn get(&self, i: usize) -> Option<&T> {
        self.q.get(i)
    }
}

#[derive(Debug)]
struct RecencyMap<K, V> {
    map: HashMap<K, MostRecent<V>>,
    n: usize,
}

impl <K, V> RecencyMap<K, V>
where
    K: Eq + Hash + Copy
{
    pub fn new(n: usize) -> RecencyMap<K, V> {
        RecencyMap {
            map: HashMap::new(),
            n,
        }
    }

    fn get_key(&mut self, key: &K) -> &mut MostRecent<V> {
        self.map.entry(*key).or_insert(MostRecent::new(self.n))
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.get_key(&key).push(value)
    }

    pub fn get(&self, key: &K, nth: usize) -> Option<&V> {
        self.map.get(key).map(|recent| recent.get(nth)).flatten()
    }
}

fn main() {
    let lines: Vec<String> = load_file_lines::<String>("input.txt")
        .map(|res| res.unwrap())
        .collect();
    let input: Vec<u32> = lines[0]
        .split(",")
        .map(|res| res.parse().unwrap())
        .collect();

    let mut last_occurance: RecencyMap<u32, usize> = RecencyMap::new(2);
    let mut last_number = 0;
    for (i, val) in input.iter().enumerate() {
        last_occurance.insert(*val, i);
        last_number = *val;
    }

    for i in input.len()..30000000 {
        let next_number = match (last_occurance.get(&last_number, 0), last_occurance.get(&last_number, 1)) {
            (Some(a), Some(b)) => (a - b) as u32,
            _ => 0,
        };
        last_occurance.insert(next_number, i);
        last_number = next_number;
        if i == 2019 {
            println!("{}", last_number);
        }
    }

    println!("{}", last_number);
}
