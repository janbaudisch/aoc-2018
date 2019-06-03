mod id;

use common::input;
use id::ID;

fn main() {
    let ids: Vec<ID> = input::read_lines()
        .iter()
        .map(|x| ID::from(x.to_string()))
        .collect();

    let mut two_same = 0;
    let mut three_same = 0;

    // check ids for letters recurring two and/or three times
    for id in ids.clone() {
        let (two, three) = id.scan();

        if two {
            two_same += 1;
        }

        if three {
            three_same += 1;
        }
    }

    println!("[PART ONE] checksum: {}", two_same * three_same);

    // find IDs differing by only one letter
    'outer: for mut id in ids.clone() {
        for other_id in ids.clone() {
            let diff = id.diff(&other_id);

            if diff.len() == 1 {
                id.remove(diff[0]);
                println!("[PART TWO] common letters between correct IDs: {}", id);
                break 'outer;
            }
        }
    }
}
