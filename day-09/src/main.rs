use commons::io::load_file_lines;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::VecDeque;

#[derive(Debug)]
struct RingBuffer<T> {
    buf: Vec<T>,
    size: usize,
    insert_at: usize,
}

impl<T> RingBuffer<T> {
    pub fn new(size: usize) -> Self {
        RingBuffer {
            buf: Vec::new(),
            insert_at: 0,
            size,
        }
    }

    pub fn push(&mut self, i: T) {
        if let Some(elem) = self.buf.get_mut(self.insert_at) {
            *elem = i;
        } else {
            self.buf.push(i);
        }
        self.insert_at = (self.insert_at + 1) % self.size;
    }
}

impl<'a, T> IntoIterator for &'a RingBuffer<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.buf.iter()
    }
}

fn main() {
    let input: Vec<u64> = load_file_lines("input.txt")
        .map(|x| x.expect("Can't read input"))
        .collect();

    let mut ring = RingBuffer::new(25);
    let mut input_iter = input.iter();
    // Read in the preamble
    for _ in 0..25 {
        ring.push(*input_iter.next().unwrap());
    }

    let mut part1 = 0;

    for &i in input_iter {
        let mut current_sorted: VecDeque<&u64> = ring.into_iter().sorted().collect();
        let mut smaller = current_sorted.pop_front();
        let mut larger = current_sorted.pop_back();
        while smaller.is_some() && larger.is_some() {
            let sum = smaller.unwrap() + larger.unwrap();
            match sum.cmp(&i) {
                Ordering::Equal => {
                    break;
                }
                Ordering::Greater => {
                    // Too big, try a smaller larger number
                    larger = current_sorted.pop_back();
                }
                Ordering::Less => {
                    // Too small, try a larger smaller number
                    smaller = current_sorted.pop_front();
                }
            }
        }

        if smaller.is_none() || larger.is_none() {
            part1 = i;
            break;
        } else {
            ring.push(i);
        }
    }
    println!("{}", part1);

    let mut current_sum = 0;
    let mut current_numbers = VecDeque::new();
    for &i in input.iter() {
        if current_sum == part1 {
            break;
        }

        current_sum += i;
        current_numbers.push_back(i);
        while current_sum > part1 {
            // Start pulling off the earlier numbers until we're back below
            current_sum -= current_numbers.pop_front().unwrap();
        }
    }
    let min = current_numbers.iter().min().unwrap();
    let max = current_numbers.iter().max().unwrap();
    println!("{}", max + min);
}
