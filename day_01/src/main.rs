use common::input;

fn main() {
    let changes: Vec<i32> = input::read_lines()
        .iter()
        .map(|x| i32::from_str_radix(&x, 10).expect("error converting input"))
        .collect();

    println!(
        "[PART ONE] resulting frequency: {}",
        changes.clone().iter().sum::<i32>()
    );

    let mut frequencies: Vec<i32> = vec![changes[0]];
    let mut i = 1;

    // loop through changes until frequency is seen twice
    loop {
        let frequency = frequencies[frequencies.len() - 1] + changes[i];

        // check if already seen
        if frequencies.contains(&frequency) {
            // if so, push and exit loop
            frequencies.push(frequency);
            break;
        } else {
            // if not, push and step forward
            frequencies.push(frequency);

            i += 1;

            // reset counter if reached end of changes
            if i == changes.len() {
                i = 0;
            }
        }
    }

    println!(
        "[PART TWO] first frequency reached twice: {}",
        frequencies[frequencies.len() - 1]
    );
}
