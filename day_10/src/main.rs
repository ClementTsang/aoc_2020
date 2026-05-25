use std::{collections::HashMap, path::Path};

use helpers::*;

fn part_1(input: &Path) {
    let mut joltages = std::fs::read_to_string(input)
        .unwrap()
        .lines()
        .map(|v| v.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    joltages.sort();

    let mut current_joltage = 0;
    let mut current_ones = 0;
    let mut current_threes = 0;

    for joltage in joltages {
        let diff = joltage - current_joltage;
        if diff == 1 {
            current_ones += 1;
        } else if diff == 3 {
            current_threes += 1;
        }
        current_joltage = joltage;
    }

    // Add the one for the final device.
    current_threes += 1;

    let result = current_ones * current_threes;
    println!("Part 1: {result}");
}

fn part_2(input: &Path) {
    let mut joltages = std::fs::read_to_string(input)
        .unwrap()
        .lines()
        .map(|v| v.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    joltages.sort();

    // Lazy way of doing caching.
    let mut seen: HashMap<(u64, Vec<u64>), u64> = HashMap::new();

    fn get_arrangements(
        seen: &mut HashMap<(u64, Vec<u64>), u64>,
        joltages: &[u64],
        current_joltage: u64,
    ) -> u64 {
        let mut results = 0;

        // println!("seen: {seen:?}, joltages: {joltages:?}, current_joltage: {current_joltage}");

        let owned_joltages = joltages.to_vec();
        if let Some(cached_results) = seen.get(&(current_joltage, owned_joltages.clone())) {
            return *cached_results;
        }

        for i in 0..joltages.len() {
            let curr = joltages[i];

            if curr - current_joltage > 3 {
                // Can't finish picking this, stop here and return your successful results.
                seen.insert((current_joltage, owned_joltages), results);
                return results;
            }

            results += get_arrangements(seen, &joltages[(i + 1)..], curr);
        }

        return results + 1;
    }

    // lol so I noticed the way I did it here doubles the result, so this just fixes it.
    let answer = get_arrangements(&mut seen, &joltages, 0) / 2;
    println!("Part 2: {answer}");
}

fn main() {
    let input = get_input_file();

    part_1(&input);
    part_2(&input);
}
