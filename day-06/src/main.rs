use commons::io::{load_file, FromLines};
use std::collections::HashSet;

#[derive(Debug)]
struct CustomsGroupStore {
    groups: Vec<HashSet<char>>,
}

impl FromLines for CustomsGroupStore {
    type Line = String;

    fn from_lines<I>(lines: &mut I) -> Self
    where
        I: Iterator<Item = Self::Line>,
    {
        let mut groups = Vec::new();
        let mut current = HashSet::new();

        for line in lines {
            if line.len() == 0 {
                groups.push(current);
                current = HashSet::new();
                continue;
            }

            for c in line.chars() {
                current.insert(c);
            }
        }
        groups.push(current);

        CustomsGroupStore {
            groups
        }
    }
}

fn main() {
    let store = load_file::<CustomsGroupStore>("input.txt");

    let yes_count = store.groups.iter()
        .map(|yeses| yeses.len())
        .fold(0, |a, c| a + c);
    println!("{}", yes_count);

}
