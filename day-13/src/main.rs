use commons::io::load_file_lines;

fn main() {
    let mut input = load_file_lines::<String>("input.txt").map(|x| x.unwrap());
    let earliest_ts: u32 = input.next().unwrap().parse().unwrap();
    let bus_ids: Vec<Option<u32>>= input.next().unwrap().split(",").map(|x| x.parse().ok()).collect();

    let known_bus_ids: Vec<u32> = bus_ids.iter().filter(|x| x.is_some()).map(|x| x.unwrap()).collect();

    'outer: for t in earliest_ts..u32::MAX {
        for id in &known_bus_ids {
            if t % id == 0 {
                println!("{}", (t - earliest_ts) * id);
                break 'outer;
            }
        }
    }
}
