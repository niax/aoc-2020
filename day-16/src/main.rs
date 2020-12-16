use bitvec::prelude::*;
use commons::io::load_file_lines;
use std::collections::HashMap;

#[derive(Debug)]
struct Range {
    from: u16,
    to: u16,
}

enum ParseState {
    Rules,
    OurTicket,
    NearbyTickets,
}

fn main() {
    let input = load_file_lines::<String>("input.txt").map(|res| res.unwrap());
    let mut ranges: HashMap<String, Vec<Range>> = HashMap::new();
    let mut aggregate_bitvec = bitarr![0; 1024];
    let mut our_ticket: Vec<u16> = Vec::new();
    let mut nearby_tickets: Vec<Vec<u16>> = Vec::new();
    let mut parse_state = ParseState::Rules;
    for i in input {
        parse_state = match parse_state {
            ParseState::Rules => {
                if i.is_empty() {
                    ParseState::OurTicket
                } else {
                    let mut parts = i.split_whitespace();
                    let category = parts.next().unwrap();
                    let mut rule_ranges = Vec::new();
                    for p in parts {
                        if p.contains("-") {
                            let range_parts = p.split("-").collect::<Vec<&str>>();
                            let this_range = Range {
                                from: range_parts[0].parse().unwrap(),
                                to: range_parts[1].parse().unwrap(),
                            };
                            for i in this_range.from..this_range.to + 1 {
                                *aggregate_bitvec.get_mut(i as usize).unwrap() = true;
                            }
                            rule_ranges.push(this_range);
                        }
                    }
                    ranges.insert(category.to_string(), rule_ranges);
                    ParseState::Rules
                }
            }
            ParseState::OurTicket => {
                if i.is_empty() {
                    ParseState::NearbyTickets
                } else if i.contains(",") {
                    our_ticket = i.split(",").map(|s| s.parse().unwrap()).collect();
                    ParseState::OurTicket
                } else {
                    ParseState::OurTicket
                }
            }
            ParseState::NearbyTickets => {
                if i.contains(",") {
                    nearby_tickets.push(i.split(",").map(|s| s.parse().unwrap()).collect());
                }
                ParseState::NearbyTickets
            }
        }
    }

    let part1: u16 = nearby_tickets.iter().map(|ticket| {
        ticket
            .iter()
            .map(|i| (i, aggregate_bitvec[*i as usize]))
            .filter(|(_, v)| !*v)
            .map(|(i, _)| *i)
            .sum::<u16>()
    }).sum();
    println!("{}", part1);
}
