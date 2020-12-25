struct Transform {
    last: u64,
    subject_number: u64,
    divisor: u64,
}

impl Transform {
    pub fn new(subject_number: u64, divisor: u64) -> Transform {
        Transform {
            last: 1,
            subject_number,
            divisor,
        }
    }
}

impl Iterator for Transform {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let next = (self.last * self.subject_number) % self.divisor;
        self.last = next;
        Some(next)
    }
}

fn main() {
    let transform = Transform::new(7, 20201227);

    let card_public = 8184785;
    let door_public = 5293040;

    let mut other_public = 0;
    let mut found_private = 0;

    for (loop_num, key) in transform.enumerate() {
        if key == card_public {
            found_private = loop_num + 1;
            other_public = door_public;
            break;
        } else if key == door_public {
            found_private = loop_num + 1;
            other_public = card_public;
            break;
        }
    }
    let mut shared = Transform::new(other_public, 20201227);
    let mut shared_secret = 0;
    for _ in 0..found_private {
        shared_secret = shared.next().unwrap();
        //println!("{}", shared_secret);
    }
    println!("{}", shared_secret);
}
