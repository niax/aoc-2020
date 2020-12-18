use commons::io::load_file_lines;
use petgraph::graph::Graph;
use petgraph::prelude::*;
use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct BagDescriptor {
    adjective: String,
    colour: String,
}

impl PartialOrd for BagDescriptor {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            self.adjective
                .cmp(&other.adjective)
                .then(self.colour.cmp(&other.colour)),
        )
    }
}

impl Ord for BagDescriptor {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl BagDescriptor {
    fn from_iter<'a>(it: &mut impl Iterator<Item = &'a str>) -> BagDescriptor {
        let adjective = it.next().unwrap().to_string();
        let colour = it.next().unwrap().to_string();
        it.next().unwrap(); // bags

        BagDescriptor { adjective, colour }
    }
}

struct BagGraph {
    graph: Graph<BagDescriptor, usize, Directed>,
    id_map: HashMap<BagDescriptor, NodeIndex>,
}

impl BagGraph {
    pub fn new() -> BagGraph {
        BagGraph {
            graph: Graph::<BagDescriptor, usize, Directed>::new(),
            id_map: HashMap::new(),
        }
    }

    fn add_node<'a>(&mut self, node: &'a BagDescriptor) -> NodeIndex {
        match self.id_map.entry(node.clone()) {
            Entry::Occupied(o) => *o.get(),
            Entry::Vacant(v) => *v.insert(self.graph.add_node(node.clone())),
        }
    }

    pub fn add_edge<'a>(
        &mut self,
        parent: &'a BagDescriptor,
        child: &'a BagDescriptor,
        count: usize,
    ) {
        let parent_id = self.add_node(parent);
        let child_id = self.add_node(child);
        match self.graph.find_edge(child_id, parent_id) {
            Some(edge_idx) => *self.graph.edge_weight_mut(edge_idx).unwrap() += count,
            None => {
                self.graph.add_edge(child_id, parent_id, count);
            }
        }
    }

    pub fn bags_containing(&self, target: &BagDescriptor) -> HashSet<BagDescriptor> {
        let mut seen = HashSet::new();
        let mut nodeq = VecDeque::new();
        nodeq.push_back(target);
        while let Some(desc) = nodeq.pop_front() {
            let node_idx = self.id_map.get(desc).unwrap();
            for neighbour in self.graph.neighbors(*node_idx) {
                let neigh_desc = &self.graph[neighbour];
                if !seen.contains(neigh_desc) {
                    nodeq.push_back(neigh_desc);
                    seen.insert(neigh_desc.clone());
                }
            }
        }

        seen
    }

    pub fn count_bags_inside(&self, target: &BagDescriptor) -> usize {
        let mut bag_count = 1;

        let node_idx = self.id_map.get(target).unwrap();
        for neighbour in self
            .graph
            .neighbors_directed(*node_idx, Direction::Incoming)
        {
            let neigh_desc = &self.graph[neighbour];
            let edge_idx = self.graph.find_edge(neighbour, *node_idx).unwrap();
            let edge_bag_count = self.graph[edge_idx];
            let sub_bag_count = self.count_bags_inside(neigh_desc);

            bag_count += edge_bag_count * sub_bag_count;
        }

        bag_count
    }
}

fn main() {
    let gold_bag = BagDescriptor {
        adjective: "shiny".to_string(),
        colour: "gold".to_string(),
    };

    let mut graph = BagGraph::new();
    for res in load_file_lines("input.txt") {
        let line: String = res.unwrap();
        let mut it = line.split(' ').peekable();
        let src_bag = BagDescriptor::from_iter(&mut it);
        it.next(); // contains
        if it.peek().unwrap() != &"no" {
            while it.peek().is_some() {
                let count: usize = it.next().unwrap().parse().unwrap();
                let descriptor = BagDescriptor::from_iter(&mut it);
                graph.add_edge(&src_bag, &descriptor, count);
            }
        }
    }

    println!("{}", graph.bags_containing(&gold_bag).len());
    println!("{}", graph.count_bags_inside(&gold_bag) - 1); // -1 to account for the gold bag
}
