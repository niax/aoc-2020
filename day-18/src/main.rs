use commons::io::load_file_lines;
use day_18::grammar;

fn main() {
    let ltr_parser = grammar::LtrExpressionParser::new();
    let infix_parser = grammar::InfixExpressionParser::new();
    let input: Vec<String> = load_file_lines::<String>("input.txt")
        .map(|res| res.unwrap())
        .collect();

    let part1: i64 = input
        .iter()
        .map(|i| ltr_parser.parse(i).unwrap().evaluate())
        .sum();
    println!("{}", part1);
    let part2: i64 = input
        .iter()
        .map(|i| infix_parser.parse(i).unwrap().evaluate())
        .sum();
    println!("{}", part2);
}
