use std::collections::HashMap;

#[derive(Debug)]
pub enum MatchType {
    References(Vec<u32>),
    Or(Box<MatchType>, Box<MatchType>),
    Chars(String),
}

impl MatchType {
    pub fn regex(&self, ruleset: &HashMap<u32, Rule>) -> String {
        match self {
            MatchType::References(l) => {
                let mut p = String::new();
                for id in l {
                    let rule = ruleset.get(&id).unwrap();
                    p.push_str(rule.regex(ruleset).as_str())
                }
                p
            }
            MatchType::Or(a, b) => {
                let mut p = String::new();
                p.push('(');
                p.push_str(a.regex(ruleset).as_str());
                p.push('|');
                p.push_str(b.regex(ruleset).as_str());
                p.push(')');
                p
            }
            MatchType::Chars(a) => a.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Rule {
    pub id: u32,
    pub matcher: MatchType,
}

impl Rule {
    pub fn new(id: u32, matcher: MatchType) -> Rule {
        Rule { id, matcher }
    }

    pub fn regex(&self, ruleset: &HashMap<u32, Rule>) -> String {
        self.matcher.regex(ruleset)
    }
}
