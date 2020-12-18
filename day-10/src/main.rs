use commons::io::load_file_lines;
use itertools::Itertools;
use petgraph::graphmap::GraphMap;
use petgraph::{Directed, Direction};
use std::collections::HashMap;

fn main() {
    let input: Vec<u32> = load_file_lines("input.txt")
        .map(|x| x.expect("Could not read input"))
        .sorted()
        .collect();
    let laptop_charge = input[input.len() - 1] + 3;

    let mut one_diff = 0;
    let mut three_diff = 0;
    let mut last = 0;
    for i in &input {
        let diff = i - last;
        if diff == 1 {
            one_diff += 1;
        } else if diff == 3 {
            three_diff += 1;
        } else {
            panic!("Different diff! {}", diff);
        }
        last = *i;
    }
    three_diff += 1; // Jump up to laptop

    println!("{}", one_diff * three_diff);

    let mut extended_input = input.clone();
    extended_input.insert(0, 0);
    // Whack 4 in such that each of the original input appears first in the window
    for _ in 1..4 {
        extended_input.push(laptop_charge);
    }
    // We could probably live without a graph here, and do something with back references in a map.
    // However, I started solving this problem with a graph, so I finished with a graph.
    let mut graph = GraphMap::<u32, u32, Directed>::with_capacity(input.len(), input.len() * 3);
    for mut window in extended_input.windows(4).map(|a| a.iter()) {
        let first = window.next().unwrap();
        graph.add_node(*first);
        for i in window {
            if *i > first + 3 {
                break;
            }
            graph.add_node(*i);
            graph.add_edge(*first, *i, 1);
        }
    }

    // Track the number of ways we can get to N, such that we can add together the number of paths
    // to a different N based on the new N's neighbours
    let mut ways_to_n = HashMap::with_capacity(input.len());
    for &i in &extended_input {
        let mut ways_to_i: u64 = graph
            .neighbors_directed(i, Direction::Incoming)
            .map(|n| ways_to_n[&n])
            .sum();
        if ways_to_i == 0 {
            // Prime the ways to get to a thing with the original port
            ways_to_i = 1;
        }
        ways_to_n.insert(i, ways_to_i);
    }

    println!("{}", ways_to_n[&laptop_charge]);
}
