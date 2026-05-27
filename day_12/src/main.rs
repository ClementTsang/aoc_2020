use helpers::*;
use std::path::Path;

#[derive(Debug)]
enum Action {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn left(curr: Direction) -> Direction {
        match curr {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
            Direction::West => Direction::South,
        }
    }

    fn right(curr: Direction) -> Direction {
        match curr {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Debug)]
struct Instruction {
    action: Action,
    value: i64,
}

fn parse_action(line: &str) -> Instruction {
    let (action, value) = line.split_at(1);

    let action = match action {
        "N" => Action::North,
        "S" => Action::South,
        "E" => Action::East,
        "W" => Action::West,
        "L" => Action::Left,
        "R" => Action::Right,
        "F" => Action::Forward,
        _ => unreachable!("invalid action"),
    };

    let value = value.parse::<i64>().unwrap();

    Instruction { action, value }
}

fn part_1(input: &Path) -> i64 {
    let instructions = std::fs::read_to_string(input)
        .unwrap()
        .lines()
        .map(parse_action)
        .collect::<Vec<_>>();

    let mut current_direction = Direction::East;
    let mut east_value = 0;
    let mut north_value = 0;

    for instruction in instructions {
        let Instruction { action, value } = instruction;

        match action {
            Action::North => north_value += value,
            Action::South => north_value -= value,
            Action::East => east_value += value,
            Action::West => east_value -= value,
            Action::Left => {
                for _ in 0..(value / 90) {
                    let new_direction = Direction::left(current_direction);
                    current_direction = new_direction;
                }
            }
            Action::Right => {
                for _ in 0..(value / 90) {
                    let new_direction = Direction::right(current_direction);
                    current_direction = new_direction;
                }
            }
            Action::Forward => match current_direction {
                Direction::North => north_value += value,
                Direction::South => north_value -= value,
                Direction::East => east_value += value,
                Direction::West => east_value -= value,
            },
        }
    }

    north_value.abs() + east_value.abs()
}

fn part_2(input: &Path) -> i64 {
    let instructions = std::fs::read_to_string(input)
        .unwrap()
        .lines()
        .map(parse_action)
        .collect::<Vec<_>>();

    let mut ship_east_value = 0;
    let mut ship_north_value = 0;

    // These are relative to the ship.
    let mut waypoint_east_rel_value = 10;
    let mut waypoint_north_rel_value = 1;

    for instruction in instructions {
        let Instruction { action, value } = instruction;

        match action {
            Action::North => waypoint_north_rel_value += value,
            Action::South => waypoint_north_rel_value -= value,
            Action::East => waypoint_east_rel_value += value,
            Action::West => waypoint_east_rel_value -= value,
            Action::Left => {
                for _ in 0..(value / 90) {
                    let new_waypoint_north = waypoint_east_rel_value;
                    let new_waypoint_east = waypoint_north_rel_value * -1;

                    waypoint_east_rel_value = new_waypoint_east;
                    waypoint_north_rel_value = new_waypoint_north;
                }
            }
            Action::Right => {
                for _ in 0..(value / 90) {
                    let new_waypoint_north = waypoint_east_rel_value * -1;
                    let new_waypoint_east = waypoint_north_rel_value;

                    waypoint_east_rel_value = new_waypoint_east;
                    waypoint_north_rel_value = new_waypoint_north;
                }
            }
            Action::Forward => {
                ship_east_value += waypoint_east_rel_value * value;
                ship_north_value += waypoint_north_rel_value * value;
            }
        }
    }

    ship_north_value.abs() + ship_east_value.abs()
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
        assert_eq!(part_1(Path::new("example.txt")), 25);
        assert_eq!(part_2(Path::new("example.txt")), 286);
    }
}
