use commons::io::load_file_lines;
// I am using it for sorted, but it doesn't know that I am
#[allow(unused_imports)]
use itertools::Itertools;

fn game(input: &mut Vec<usize>, first: usize, iterations: usize) {
    let input_min = input
        .iter()
        .enumerate()
        .filter(|(_, x)| **x != 0)
        .map(|(i, _)| i)
        .min()
        .unwrap();
    let input_max = input
        .iter()
        .enumerate()
        .filter(|(_, x)| **x != 0)
        .map(|(i, _)| i)
        .max()
        .unwrap();

    let mut current_cup = first;
    for _ in 0..iterations {
        //print_from(input, current_cup);

        let mut next_three = Vec::with_capacity(3);
        next_three.push(input[current_cup]);
        next_three.push(input[next_three[0]]);
        next_three.push(input[next_three[1]]);
        let next_cup = input[next_three[2]];

        let mut target_cup = current_cup - 1;
        loop {
            if target_cup < input_min {
                target_cup = input_max;
            }
            if next_three.contains(&target_cup) {
                target_cup -= 1;
            } else {
                break;
            }
        }

        let insertion_end = input[target_cup];
        input[target_cup] = next_three[0];
        input[next_three[2]] = insertion_end;
        input[current_cup] = next_cup;

        current_cup = next_cup;
    }
}

fn part1(mut input: Vec<usize>, first: usize) -> String {
    game(&mut input, first, 100);

    let mut s = String::new();
    let mut i = input[1];
    while i != 1 {
        s += i.to_string().as_str();
        i = input[i];
    }
    s
}

fn part2(mut input: Vec<usize>, first: usize) -> u64 {
    game(&mut input, first, 10000000);

    let first = input[1];
    let second = input[first];

    (first as u64) * (second as u64)
}

fn main() {
    let input_str = load_file_lines::<String>("input.txt")
        .map(|res| res.unwrap())
        .next()
        .unwrap();
    let input: Vec<usize> = input_str
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect();
    let input_max = *input.iter().max().unwrap();

    let mut reflist = Vec::new();
    reflist.resize_with(input_max + 1, Default::default);
    let mut last = input[0];
    for &i in input.iter().skip(1) {
        reflist[last] = i;
        last = i;
    }
    reflist[last] = input[0];

    let part1_result = part1(reflist.clone(), input[0]);
    println!("{}", part1_result);

    reflist.resize_with(1000001, Default::default);
    let mut last = input[input.len() - 1];
    let mut v = input_max + 1;
    while v <= 1000000 {
        reflist[last] = v;
        last = v;
        v += 1;
    }
    reflist[1000000] = input[0];
    let part2_result = part2(reflist, input[0]);
    println!("{}", part2_result);
}
