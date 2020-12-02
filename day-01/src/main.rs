use commons::io::load_file_lines;

fn main() {
    let mut ints: Vec<u32> = load_file_lines("input.txt");
    ints.sort();
    let mut lower_idx = 0;
    let mut upper_idx = ints.len() - 1;
    loop {
        let lower = ints[lower_idx];
        let upper = ints[upper_idx];

        let sum = lower + upper;
        if sum == 2020 {
            println!("{} * {} = {}", lower, upper, lower*upper);
            break;
        } else if sum < 2020 {
            // Move the lower bound up to make bigger numbers
            lower_idx += 1;
        } else {
            // Move the upper bound down to make smaller numbers
            upper_idx -= 1;
        }
    }
}
