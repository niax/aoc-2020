use commons::io::{load_file_records, ParseLinesError};
use std::collections::HashSet;
use std::convert::Infallible;

#[derive(Debug)]
struct CustomsGroupStore {
    groups: Vec<CustomsGroup>,
}

#[derive(Debug)]
struct CustomsGroup {
    yes_answers: Vec<HashSet<char>>,
}

impl CustomsGroup {
    pub fn uniq_answers(&self) -> HashSet<char> {
        self.yes_answers
            .iter()
            .flat_map(|s| s)
            .map(|c| *c)
            .collect()
    }

    pub fn answer_intersection(&self) -> HashSet<char> {
        let mut intersection = self.yes_answers[0].clone();
        for answers in &self.yes_answers {
            intersection = intersection.intersection(&answers).map(|c| *c).collect();
        }

        intersection
    }

    fn from_iter<'a>(it: &mut impl Iterator<Item = &'a String>) -> CustomsGroup {
        let yes_answers = it.map(|line| line.chars().collect()).collect();
        CustomsGroup { yes_answers }
    }
}

impl CustomsGroupStore {
    fn from_iter(
        it: &mut impl Iterator<Item = Result<Vec<String>, ParseLinesError<Infallible>>>,
    ) -> Result<CustomsGroupStore, ParseLinesError<Infallible>> {
        let mut groups = Vec::new();
        for record in it {
            let lines = record?;
            groups.push(CustomsGroup::from_iter(&mut lines.iter()));
        }

        return Ok(CustomsGroupStore { groups });
    }
}

fn main() {
    let mut record_iter = load_file_records("input.txt", "");
    let store =
        CustomsGroupStore::from_iter(&mut record_iter).expect("Failed to load customs groups");

    let yes_count = store
        .groups
        .iter()
        .map(|group| group.uniq_answers().len())
        .fold(0, |a, c| a + c);
    println!("{}", yes_count);

    let part2 = store
        .groups
        .iter()
        .map(|group| group.answer_intersection().len())
        .fold(0, |a, c| a + c);
    println!("{}", part2);
}
