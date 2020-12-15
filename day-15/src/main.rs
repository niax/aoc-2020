use commons::io::load_file_lines;
use std::collections::{HashMap, VecDeque};

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

fn main() {
    let lines: Vec<String> = load_file_lines::<String>("input.txt")
        .map(|res| res.unwrap())
        .collect();
    let input: Vec<u32> = lines[0]
        .split(",")
        .map(|res| res.parse().unwrap())
        .collect();

    let mut last_occurance: HashMap<u32, MostRecent<usize>> = HashMap::new();
    let mut last_number = 0;
    for (i, val) in input.iter().enumerate() {
        let idxs = last_occurance.entry(*val).or_insert(MostRecent::new(2));
        idxs.push(i);
        last_number = *val;
    }

    for i in input.len()..30000000 {
        let next_number = match last_occurance.get(&last_number) {
            Some(n) => match (n.get(0), n.get(1)) {
                (Some(a), Some(b)) => (a - b) as u32,
                _ => 0,
            },
            None => 0,
        };
        let idxs = last_occurance
            .entry(next_number)
            .or_insert(MostRecent::new(2));
        idxs.push(i);
        //println!("Iteration {}: {} was the previous number {} is the next - {:?}", i + 1, last_number, next_number, last_occurance);
        last_number = next_number;
        if i == 2019 {
            println!("{}", last_number);
        }
    }

    println!("{}", last_number);
}
