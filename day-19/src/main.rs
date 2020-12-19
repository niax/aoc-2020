use commons::io::load_file_lines;
use day_19::{grammar, types::Rule};
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let parser = grammar::RuleParser::new();
    let input_lines = load_file_lines::<String>("input.txt").map(|res| res.unwrap());
    let mut ruleset: HashMap<u32, Rule> = HashMap::new();
    let mut parse = true;
    let mut part1 = 0;
    let mut re = Regex::new("").unwrap(); 
    for line in input_lines {
        if parse {
            if line.is_empty() {
                parse = false;
                let rule = ruleset.get(&0).unwrap();
                let mut re_str = String::new();
                re_str.push('^');
                re_str.push_str(rule.regex(&ruleset).as_str());
                re_str.push('$');

                re = Regex::new(re_str.as_str()).unwrap(); 
            } else {
                let rule = parser.parse(&line).unwrap();
                ruleset.insert(rule.id, rule);
            }
        } else {
            if re.is_match(&line) {
                part1 += 1;
            }
        }
    }

    println!("{}", part1);
}
