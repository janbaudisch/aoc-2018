mod polymer;

use common::input;
use polymer::Polymer;
use std::collections::HashSet;

fn main() {
    // grab input
    let mut polymer = Polymer::from(input::read_line());

    let mut list: HashSet<char> = HashSet::new();

    // get list of elements (discard lowercase <-> uppercase difference)
    for element in polymer.clone().into_iter() {
        list.insert(element.to_lowercase().next().unwrap());
    }

    let mut alternatives: Vec<Polymer> = Vec::new();

    // create alternative polymers
    for element in list {
        let mut alternative = polymer.clone();
        // filter out element
        alternative.filter(element);
        alternatives.push(alternative);
    }

    // part one: react and count elements
    polymer.react();
    println!("[PART ONE] number of resulting elements: {}", polymer.len());

    let mut shortest = polymer.len();

    // part two: react each alternative and count elements of the shortest one
    for mut alternative in alternatives {
        alternative.react();

        if alternative.len() < shortest {
            shortest = alternative.len();
        }
    }

    println!(
        "[PART TWO] shortest possible number of resulting elements: {}",
        shortest
    );
}
