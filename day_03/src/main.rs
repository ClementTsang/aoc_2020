use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Part 1
    // count_trees(3, 1);

    // Part 2
    let m = count_trees(1, 1)
        * count_trees(3, 1)
        * count_trees(5, 1)
        * count_trees(7, 1)
        * count_trees(1, 2);
    println!("m: {}", m);
}

fn count_trees(right: usize, down: usize) -> usize {
    let input = BufReader::new(File::open("input.txt").unwrap());

    let mut x = 0;
    let mut y = 0;
    let mut trees = 0;

    let mut first_run = true;
    let mut line_width = 0;

    for line in input.lines() {
        if let Ok(line) = line {
            if first_run {
                first_run = false;
                line_width = line.len();
                continue;
            }

            y += 1; // We always iterate by a line.
            y = y % down;
            if y != 0 {
                continue;
            }

            x += right;
            x = x % line_width;

            if let Some(c) = line.chars().collect::<Vec<_>>().get(x) {
                if *c == '#' {
                    trees += 1;
                }
            }
        }
    }

    println!("Trees: {}", trees);
    trees
}
