use helpers::*;
use std::{collections::HashMap, fs, path::Path};

fn solve(input: &Path, end: usize) -> usize {
    let s = fs::read_to_string(input).unwrap();
    let numbers = s
        .trim()
        .split(",")
        .map(|v| v.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut prev = 0;
    let mut seen = HashMap::new();

    for i in 1..=end {
        match numbers.get(i - 1) {
            Some(num) => {
                seen.insert(prev, i - 1);
                prev = *num;
            }
            None => match seen.get(&prev) {
                Some(&last_seen) => {
                    seen.insert(prev, i - 1);
                    prev = i - 1 - last_seen;
                }
                None => {
                    seen.insert(prev, i - 1);
                    prev = 0;
                }
            },
        }
    }

    prev
}

fn part_1(input: &Path) -> usize {
    solve(input, 2020)
}

fn part_2(input: &Path) -> usize {
    solve(input, 30000000)
}

fn main() {
    let input = get_input_file();

    println!("Part one: {}", part_1(&input));
    println!("Part two: {}", part_2(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(part_1(Path::new("example.txt")), 436);
        assert_eq!(part_2(Path::new("example.txt")), 175594);
    }
}
