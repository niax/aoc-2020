use commons::io::load_file_records;
use std::cmp;
use std::collections::VecDeque;

fn main() {
    let mut records = load_file_records::<String>("input.txt", "").map(|res| res.unwrap());
    let mut get_player = move || {
        records
            .next()
            .unwrap()
            .iter()
            .skip(1)
            .map(|s| s.parse().unwrap())
            .collect()
    };
    let mut player1: VecDeque<u32> = get_player();
    let mut player2: VecDeque<u32> = get_player();

    while !player1.is_empty() && !player2.is_empty() {
        let c1 = player1.pop_front().unwrap();
        let c2 = player2.pop_front().unwrap();

        let winner = if c1 > c2 { &mut player1 } else { &mut player2 };

        winner.push_back(cmp::max(c1, c2));
        winner.push_back(cmp::min(c1, c2));
    }

    let winner = if player1.len() > player2.len() {
        &player1
    } else {
        &player2
    };
    let part1: usize = winner
        .iter()
        .enumerate()
        .map(|(i, x)| (winner.len() - i) * (*x as usize))
        .sum();
    println!("{}", part1);
}
