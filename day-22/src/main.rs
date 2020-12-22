use commons::io::load_file_records;
use std::cmp;
use std::collections::{HashSet, VecDeque};

fn simple_game<'a>(
    deck1: impl IntoIterator<Item = &'a usize>,
    deck2: impl IntoIterator<Item = &'a usize>,
) -> VecDeque<usize> {
    let mut player1: VecDeque<usize> = deck1.into_iter().copied().collect();
    let mut player2: VecDeque<usize> = deck2.into_iter().copied().collect();

    while !(player1.is_empty() || player2.is_empty()) {
        let c1 = player1.pop_front().unwrap();
        let c2 = player2.pop_front().unwrap();

        let winner = if c1 > c2 { &mut player1 } else { &mut player2 };

        winner.push_back(cmp::max(c1, c2));
        winner.push_back(cmp::min(c1, c2));
    }

    if player1.len() > player2.len() {
        player1
    } else {
        player2
    }
}

fn score(hand: VecDeque<usize>) -> usize {
    hand.iter()
        .enumerate()
        .map(|(i, x)| (hand.len() - i) * (*x as usize))
        .sum()
}

fn recursive_game<'a>(
    deck1: impl IntoIterator<Item = &'a usize>,
    deck2: impl IntoIterator<Item = &'a usize>,
) -> (usize, VecDeque<usize>) {
    let mut player1: VecDeque<usize> = deck1.into_iter().copied().collect();
    let mut player2: VecDeque<usize> = deck2.into_iter().copied().collect();
    let mut seen_games = HashSet::new();

    while !(player1.is_empty() || player2.is_empty()) {
        let hand_key = (player1.clone(), player2.clone());
        if seen_games.contains(&hand_key) {
            return (1, player1);
        }
        seen_games.insert(hand_key);

        let c1 = player1.pop_front().unwrap();
        let c2 = player2.pop_front().unwrap();

        let winning_player = if player1.len() >= c1 && player2.len() >= c2 {
            recursive_game(
                player1.clone().iter().take(c1),
                player2.clone().iter().take(c2),
            )
            .0
        } else if c1 > c2 {
            1
        } else {
            2
        };

        let (winner, c_first, c_second) = match winning_player {
            1 => (&mut player1, c1, c2),
            2 => (&mut player2, c2, c1),
            _ => panic!("Unknown player"),
        };

        winner.push_back(c_first);
        winner.push_back(c_second);
    }

    let (winner, winning_hand) = if player1.len() > player2.len() {
        (1, player1)
    } else {
        (2, player2)
    };

    (winner, winning_hand)
}

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
    let player1: Vec<usize> = get_player();
    let player2: Vec<usize> = get_player();

    let part1 = simple_game(&player1, &player2);
    println!("{}", score(part1));

    let (_, part2) = recursive_game(&player1, &player2);
    println!("{}", score(part2));
}
