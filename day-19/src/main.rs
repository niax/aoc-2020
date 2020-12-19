use commons::io::load_file_lines;
use day_19::{grammar, types::MatchType};
use std::collections::HashMap;

fn main() {
    let parser = grammar::RuleParser::new();
    let input_lines = load_file_lines::<String>("input.txt").map(|res| res.unwrap());
    let mut ruleset: HashMap<u32, MatchType> = HashMap::new();
    let mut parse = true;
    let mut test_lines = Vec::new();
    for line in input_lines {
        if parse {
            if line.is_empty() {
                parse = false;
            } else {
                let rule = parser.parse(&line).unwrap();
                ruleset.insert(rule.id, rule.matcher);
            }
        } else {
            test_lines.push(line);
        }
    }

    let rule = ruleset.get(&0).unwrap();
    let part1 = test_lines
        .iter()
        .map(|line| rule.matches_exact(&ruleset, line))
        .filter(|x| *x)
        .count();
    println!("{}", part1);

    let mut ruleset_2 = ruleset.clone();
    let new_8 = parser.parse("8: 42 | 42 8").unwrap();
    let new_11 = parser.parse("11: 42 31 | 42 11 31").unwrap();
    ruleset_2.insert(8, new_8.matcher);
    ruleset_2.insert(11, new_11.matcher);

    let part2 = test_lines
        .iter()
        .map(|line| rule.matches_exact(&ruleset_2, line))
        .filter(|x| *x)
        .count();
    println!("{}", part2);
}
