use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let input = BufReader::new(File::open("input.txt").unwrap());

    let mut s = HashSet::new();
    let mut c = 0;
    for line in input.lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                // We check our results before.
                c += s.len();
                s.clear();
            } else {
                for c in line.chars() {
                    s.insert(c);
                }
            }
        }
    }

    println!("Part 1: {}", c);
}

fn part_2() {
    let input = BufReader::new(File::open("input.txt").unwrap());

    let mut m = HashMap::new();
    let mut group_size = 0;
    let mut c = 0;
    for line in input.lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                // We check our results before.
                for v in m.values() {
                    if *v == group_size {
                        c += 1;
                    }
                }
                group_size = 0;
                m.clear();
            } else {
                for c in line.chars() {
                    *(m.entry(c).or_insert(0)) += 1;
                }
                group_size += 1;
            }
        }
    }

    println!("Part 2: {}", c);
}
