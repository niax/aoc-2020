use commons::io::load_file_lines;

fn sum_to_target(ints: &[u32], target: u32) -> Option<(u32, u32)> {
    let mut lower_idx = 0;
    let mut upper_idx = ints.len() - 1;
    while lower_idx != upper_idx {
        let lower = ints[lower_idx];
        let upper = ints[upper_idx];

        let sum = lower + upper;
        if sum == target {
            return Some((lower, upper));
        } else if sum < target {
            // Move the lower bound up to make bigger numbers
            lower_idx += 1;
        } else {
            // Move the upper bound down to make smaller numbers
            upper_idx -= 1;
        }
    }
    None
}

fn main() {
    let target = 2020;
    let mut ints: Vec<u32> = load_file_lines("input.txt");
    ints.sort();
    // Part 1
    let part1 = sum_to_target(&ints[..], 2020);
    match part1 {
        Some((a, b)) => println!("{} * {} = {}", a, b, a * b),
        _ => println!("Noooes"),
    }

    // Part 2
    for (i, c) in ints.iter().enumerate() {
        let remain = &ints[i..];
        let part_target = target - c;
        if let Some((a, b)) = sum_to_target(remain, part_target) {
            println!("{} * {} * {} = {}", c, a, b, c * a * b);
        }
    }
}
