use commons::io::load_file_lines;
// I am using it for sorted, but it doesn't know that I am
#[allow(unused_imports)]
use itertools::Itertools;
use std::collections::VecDeque;

fn main() {
    let input_str = load_file_lines::<String>("input.txt")
        .map(|res| res.unwrap())
        .next()
        .unwrap();
    let mut input: VecDeque<i32> = input_str
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect();
    let input_min = *input.iter().min().unwrap();
    let input_max = *input.iter().max().unwrap();

    for _ in 0..100 {
        let current_cup = input.pop_front().unwrap();
        //println!("cups: ({}) {:?}", current_cup, input);
        let next_three: Vec<i32> = (0..3).map(|_| input.pop_front().unwrap()).collect();
        //println!("pick up: {:?}", next_three);
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
        //println!("target: ({})", target_cup);
        let (target_idx, _) = input
            .iter()
            .enumerate()
            .find(|(_, cup)| target_cup == **cup)
            .unwrap();
        //println!("target: {:?} ({})", target_cup, target_idx);
        for cup in next_three.iter().rev() {
            input.insert(target_idx + 1, *cup);
        }
        //println!();
        input.push_back(current_cup);
    }

    loop {
        let i = input.pop_front().unwrap();
        if i == 1 {
            println!(
                "{}",
                input
                    .iter()
                    .fold(String::new(), |a, c| a + c.to_string().as_str())
            );
            break;
        }
        input.push_back(i);
    }
}
