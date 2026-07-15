use std::{collections::HashMap, str::MatchIndices};

use rand::RngExt;

fn main() {
    let mut ints = generate_nth_array_of_integers(100);
    ints.sort();

    let mode = get_array_mode(&ints);
    let median = get_array_median(&ints);
    println!("array: {ints:?}");
    println!("mode: {mode}");
    println!("median: {median}");
}

fn generate_nth_array_of_integers(n: usize) -> Vec<u32> {
    let mut ints: Vec<u32> = Vec::with_capacity(n);

    let mut random_rng = rand::rng();

    for _ in 1..=n {
        let i: u32 = random_rng.random_range(1..=100);
        ints.push(i);
    }

    ints
}

fn get_array_median(ints: &Vec<u32>) -> u32 {
    let size = ints.len();
    let middle: usize = size / 2;

    ints[middle]
}

fn get_array_mode(ints: &Vec<u32>) -> u32 {
    let mut map = HashMap::new();
    let mut max_value = 0;
    let mut max_key = 0;

    for i in ints {
        let count = map.entry(i).or_insert(0);
        *count += 1;
        // map.insert(i, *count);
    }

    for (k, v) in map {
        if v > max_value {
            max_value = v;
            max_key = *k;
        }
    }

    max_key
}
