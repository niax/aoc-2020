use commons::io::load_file_lines;
use day_18::{grammar, types};

fn main() {
    let parser = grammar::ExpressionParser::new();
    let input: Vec<types::Expr> = load_file_lines::<String>("input.txt")
        .map(|res| res.unwrap())
        .map(|s| parser.parse(s.as_str()).unwrap())
        .collect();

    let part1: i64 = input.iter().map(|i| i.evaluate()).sum();
    println!("{}", part1);
}
