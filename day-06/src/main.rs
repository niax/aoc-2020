use commons::io::{load_file, FromLines};
use std::collections::HashSet;

#[derive(Debug)]
struct CustomsGroupStore {
    groups: Vec<CustomsGroup>,
}

#[derive(Debug)]
struct CustomsGroup {
    yes_answers: Vec<HashSet<char>>
}

impl CustomsGroup {
    pub fn uniq_answers(&self) -> HashSet<char> {
        self.yes_answers.iter().flat_map(|s| s).map(|c| *c).collect()
    }

    pub fn answer_intersection(&self) -> HashSet<char> {
        let mut intersection = self.yes_answers[0].clone();
        for answers in &self.yes_answers {
            intersection = intersection.intersection(&answers).map(|c| *c).collect();
        }

        intersection
    }
}

impl FromLines for CustomsGroupStore {
    type Line = String;

    fn from_lines<I>(lines: &mut I) -> Self
    where
        I: Iterator<Item = Self::Line>,
    {
        let mut groups = Vec::new();
        let mut current_group = Vec::new();

        for line in lines {
            if line.len() == 0 {
                groups.push(CustomsGroup {
                    yes_answers: current_group,
                });
                current_group = Vec::new();
                continue;
            }

            let mut person = HashSet::new();
            for c in line.chars() {
                person.insert(c);
            }
            current_group.push(person);
        }
        groups.push(CustomsGroup {
            yes_answers: current_group,
        });

        CustomsGroupStore {
            groups
        }
    }
}

fn main() {
    let store = load_file::<CustomsGroupStore>("input.txt");

    let yes_count = store.groups.iter()
        .map(|group| group.uniq_answers().len())
        .fold(0, |a, c| a + c);
    println!("{}", yes_count);

    let part2 = store.groups.iter()
        .map(|group| group.answer_intersection().len())
        .fold(0, |a, c| a + c);
    println!("{}", part2);
}
