use bitvec::prelude::*;
use commons::io::load_file_lines;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
struct BitRanges {
    bitvec: BitVec<Msb0, u8>,
}

impl BitRanges {
    pub fn new(max: usize) -> Self {
        let mut bitvec = BitVec::with_capacity(max);
        bitvec.resize(max, false);
        BitRanges { bitvec }
    }

    pub fn add_inclusive(&mut self, from: u16, to: u16) {
        for i in from..to + 1 {
            *self.bitvec.get_mut(i as usize).unwrap() = true;
        }
    }

    pub fn contains(&self, v: u16) -> bool {
        self.bitvec[v as usize]
    }
}

fn numbers_in_range<'a>(mut numbers: impl Iterator<Item = &'a u16>, range: &BitRanges) -> bool {
    numbers.find(|n| !range.contains(**n)).is_none()
}

struct Input {
    aggregate_ranges: BitRanges,
    rules: HashMap<String, BitRanges>,
    our_ticket: Vec<u16>,
    nearby_tickets: Vec<Vec<u16>>,
}

impl Input {
    pub fn from_iter(mut input: impl Iterator<Item = String>) -> Input {
        let mut aggregate_ranges = BitRanges::new(1024);
        let mut rules: HashMap<String, BitRanges> = HashMap::new();
        for i in &mut input {
            if i.is_empty() {
                // End of the rules block
                break;
            }
            let mut parts = i.split(":");
            let category = parts.next().unwrap();
            let range_strings = parts.next().unwrap().split_whitespace();
            let mut rule_ranges = BitRanges::new(1024);
            for p in range_strings {
                if p.contains("-") {
                    let range_parts = p.split("-").collect::<Vec<&str>>();
                    let from: u16 = range_parts[0].parse().unwrap();
                    let to: u16 = range_parts[1].parse().unwrap();
                    rule_ranges.add_inclusive(from, to);
                    aggregate_ranges.add_inclusive(from, to);
                }
            }
            rules.insert(category.to_string(), rule_ranges);
        }

        let mut our_ticket: Vec<u16> = Vec::new();
        for i in &mut input {
            if i.is_empty() {
                // End of the ticket block
                break;
            } else if i.contains(",") {
                our_ticket = i.split(",").map(|s| s.parse().unwrap()).collect();
            }
        }

        let mut nearby_tickets: Vec<Vec<u16>> = Vec::new();
        for i in &mut input {
            if i.contains(",") {
                nearby_tickets.push(i.split(",").map(|s| s.parse().unwrap()).collect());
            }
        }

        Input {
            aggregate_ranges,
            rules,
            our_ticket,
            nearby_tickets,
        }
    }

    pub fn part1(&self) -> u16 {
        return self
            .nearby_tickets
            .iter()
            .map(|ticket| {
                ticket
                    .iter()
                    .map(|i| (i, self.aggregate_ranges.contains(*i)))
                    .filter(|(_, v)| !*v)
                    .map(|(i, _)| *i)
                    .sum::<u16>()
            })
            .sum();
    }

    pub fn part2(&self) -> u128 {
        // Transpose known tickets into being vec of the values in the same position
        let mut fields: Vec<Vec<u16>> = Vec::new();
        for v in &self.our_ticket {
            let mut l = Vec::with_capacity(self.nearby_tickets.len());
            l.push(*v);
            fields.push(l);
        }

        for ticket in &self.nearby_tickets {
            let valid = ticket
                .iter()
                .map(|&i| (i, self.aggregate_ranges.contains(i)))
                .filter(|(_, v)| !v)
                .count()
                == 0;
            if valid {
                for (field, value) in ticket.iter().enumerate() {
                    fields[field].push(*value);
                }
            }
        }

        let mut possible_field_names = Vec::new();
        for field_values in fields {
            let possibles: HashSet<&String> = self
                .rules
                .iter()
                .filter(|(_, rule_range)| numbers_in_range(field_values.iter(), rule_range))
                .map(|(name, _)| name)
                .collect();
            possible_field_names.push(possibles);
        }
        println!("{:?}", possible_field_names);

        let mut field_names = HashMap::new();
        for (i, possible_names) in possible_field_names
            .iter()
            .enumerate()
            .sorted_by_key(|(_, p)| p.len())
        {
            let real_possibles: Vec<&&String> = possible_names
                .iter()
                .filter(|&x| !field_names.contains_key(x))
                .collect();
            if real_possibles.len() == 1 {
                field_names.insert(*real_possibles[0], i);
            } else {
                println!("Ambigious: {:?}", real_possibles);
            }
        }

        println!("{:?}", field_names);

        field_names
            .iter()
            .filter(|(name, _)| name.contains("departure"))
            .map(|(_, column)| self.our_ticket[*column])
            .fold(1 as u128, |a, x| a * (x as u128))
    }
}

fn main() {
    let input = Input::from_iter(load_file_lines("input.txt").map(|res| res.unwrap()));

    println!("{}", input.part1());
    println!("{}", input.part2());
}
