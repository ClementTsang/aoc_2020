use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // part_one();

    part_two();
}

fn part_one() {
    let input = BufReader::new(File::open("input").unwrap());

    let mut highest_seat_id = 0;

    for line in input.lines() {
        if let Ok(line) = line {
            let mut lhs = 0;
            let mut rhs = 127;

            let v = line.chars().collect::<Vec<_>>();
            let chars = v.iter().rev().skip(4).rev();
            for c in chars {
                if *c == 'F' {
                    rhs = (lhs + rhs) / 2 - 1;
                } else if *c == 'B' {
                    lhs = (lhs + rhs) / 2 + 1;
                }
            }
            let row = if v[v.len() - 4] == 'F' {
                std::cmp::min(lhs, rhs)
            } else {
                std::cmp::max(lhs, rhs)
            };

            let mut lhs = 0;
            let mut rhs = 7;
            for i in vec![3, 2] {
                let c = v[v.len() - i];
                if c == 'L' {
                    rhs = (lhs + rhs) / 2 - 1;
                } else if c == 'R' {
                    lhs = (lhs + rhs) / 2 + 1;
                }
            }
            let col = if v[v.len() - 1] == 'L' {
                std::cmp::min(lhs, rhs)
            } else {
                std::cmp::max(lhs, rhs)
            };

            let sid = row * 8 + col;

            if sid > highest_seat_id {
                highest_seat_id = sid;
            }
        }
    }

    println!("Highest seat ID: {}", highest_seat_id);
}

fn part_two() {
    let input = BufReader::new(File::open("input").unwrap());

    let mut seats: Vec<usize> = vec![];

    for line in input.lines() {
        if let Ok(line) = line {
            let mut lhs = 0;
            let mut rhs = 127;

            let v = line.chars().collect::<Vec<_>>();
            let chars = v.iter().rev().skip(4).rev();
            for c in chars {
                if *c == 'F' {
                    rhs = (lhs + rhs) / 2;
                } else if *c == 'B' {
                    lhs = (lhs + rhs) / 2 + 1;
                }
            }
            let row = if v[v.len() - 4] == 'F' {
                std::cmp::min(lhs, rhs)
            } else {
                std::cmp::max(lhs, rhs)
            };

            let mut lhs = 0;
            let mut rhs = 7;
            for i in vec![3, 2] {
                let c = v[v.len() - i];
                if c == 'L' {
                    rhs = (lhs + rhs) / 2;
                } else if c == 'R' {
                    lhs = (lhs + rhs) / 2 + 1;
                }
            }
            let col = if v[v.len() - 1] == 'L' {
                std::cmp::min(lhs, rhs)
            } else {
                std::cmp::max(lhs, rhs)
            };

            let sid = row * 8 + col;

            seats.push(sid);
        }
    }
    seats.sort();

    let summation: usize = (*seats.first().unwrap()..(*seats.last().unwrap() + 1)).sum();
    let actual: usize = seats.iter().sum();

    println!("My seat: {}", summation - actual);
}
