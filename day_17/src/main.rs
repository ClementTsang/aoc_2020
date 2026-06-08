use helpers::*;
use std::{collections::HashSet, path::Path};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum State {
    Active,
    Inactive,
}

impl From<char> for State {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Inactive,
            '#' => Self::Active,
            _ => unreachable!("invalid char"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    inner: [usize; 3],
}

impl Point {
    fn add_2d(x: usize, y: usize) -> Self {
        let mut inner = [0; _];
        inner[0] = x;
        inner[1] = y;

        Self { inner }
    }
}

fn parse_input(input: &Path) -> Vec<Vec<State>> {
    let s = std::fs::read_to_string(input).unwrap();

    s.lines()
        .map(|line| line.chars().map(|c| State::from(c)).collect())
        .collect()
}

fn part_1(input: &Path) -> usize {
    let initial_state = parse_input(input);
    let height = initial_state.len();
    let width = initial_state[0].len();
    let mut initial_state: HashSet<Point> = HashSet::new();

    for y in 0..height {
        for x in 0..width {
            initial_state.insert(Point::add_2d(x, y));
        }
    }

    for _i in 0..6 {
        let mut current_state = initial_state.clone();
        // let mut next_state = HashSet::new();

        for curr in &current_state {}
    }

    0
}

fn part_2(input: &Path) -> usize {
    0
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
        assert_eq!(part_1(Path::new("example.txt")), 112);
        assert_eq!(part_2(Path::new("example.txt")), 0);
    }
}
