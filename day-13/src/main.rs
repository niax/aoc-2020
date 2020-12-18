use commons::io::load_file_lines;

fn main() {
    let mut input = load_file_lines::<String>("input.txt").map(|x| x.unwrap());
    let earliest_ts: u32 = input.next().unwrap().parse().unwrap();
    let bus_ids: Vec<Option<u32>> = input
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse().ok())
        .collect();

    let known_bus_ids: Vec<u32> = bus_ids
        .iter()
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect();

    'outer: for t in earliest_ts..u32::MAX {
        for id in &known_bus_ids {
            if t % id == 0 {
                println!("{}", (t - earliest_ts) * id);
                break 'outer;
            }
        }
    }

    let mut t: u64 = 0;
    loop {
        let mut found = true;
        let mut incr = 1;
        for (i, opt_id) in bus_ids.iter().enumerate() {
            if let Some(id) = opt_id {
                let bus_ts = t + i as u64;
                if bus_ts % (*id as u64) != 0 {
                    found = false;
                    break;
                } else {
                    // The next common place will be at least the multiple of all the currently
                    // matching IDs, there aren't any places sooner where the multiples align.
                    incr *= *id as u64
                }
            }
        }

        if found {
            println!("{}", t);
            break;
        }
        t += incr;
    }
}
