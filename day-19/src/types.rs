use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum MatchType {
    References(Vec<u32>),
    Or(Box<MatchType>, Box<MatchType>),
    Chars(String),
}

impl MatchType {
    pub fn matches<'a>(&self, ruleset: &HashMap<u32, MatchType>, s: &'a str) -> Vec<&'a str> {
        if s.is_empty() {
            return Vec::new();
        }

        match self {
            MatchType::References(l) => {
                let mut next = Vec::new();
                next.push(s);

                for id in l {
                    let rule = ruleset.get(&id).unwrap();
                    let next_next = next
                        .iter()
                        .map(|remain| rule.matches(ruleset, remain))
                        .filter(|v| !v.is_empty())
                        .flatten()
                        .collect();
                    next = next_next;
                }

                next
            }
            MatchType::Or(a, b) => a
                .matches(ruleset, s)
                .iter()
                .chain(b.matches(ruleset, s).iter())
                .copied()
                .collect(),
            MatchType::Chars(a) => {
                if &s[0..a.len()] == a {
                    vec![&s[a.len()..]]
                } else {
                    Vec::new()
                }
            }
        }
    }

    pub fn matches_exact(&self, ruleset: &HashMap<u32, MatchType>, s: &str) -> bool {
        self.matches(ruleset, s).iter().any(|x| x.is_empty())
    }
}

#[derive(Debug, Clone)]
pub struct Rule {
    pub id: u32,
    pub matcher: MatchType,
}

impl Rule {
    pub fn new(id: u32, matcher: MatchType) -> Rule {
        Rule { id, matcher }
    }
}
