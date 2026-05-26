use helpers::*;
use std::path::Path;

fn num_occupied_simple(previous_state: &[Vec<char>], i: usize, j: usize) -> usize {
    let mut num_occupied = 0;

    for u in [-1, 0, 1] {
        for v in [-1, 0, 1] {
            if u == 0 && v == 0 {
                continue;
            }

            let ii = i as i64 + u;
            let jj = j as i64 + v;

            if (ii < 0 || ii >= previous_state.len() as i64)
                || (jj < 0 || jj >= previous_state[i].len() as i64)
            {
                continue;
            }

            let ii = ii as usize;
            let jj = jj as usize;

            if previous_state[ii][jj] == '#' {
                num_occupied += 1;
            }
        }
    }

    num_occupied
}

fn part_1(input: &Path) -> usize {
    let mut previous_state = std::fs::read_to_string(input)
        .unwrap()
        .lines()
        .map(|v| v.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    loop {
        let mut new_state = previous_state.clone();

        for i in 0..previous_state.len() {
            for j in 0..previous_state[i].len() {
                match previous_state[i][j] {
                    'L' => {
                        if num_occupied_simple(&previous_state, i, j) == 0 {
                            new_state[i][j] = '#';
                        }
                    }
                    '#' => {
                        if num_occupied_simple(&previous_state, i, j) >= 4 {
                            new_state[i][j] = 'L';
                        }
                    }
                    '.' | _ => {}
                }
            }
        }

        if new_state == previous_state {
            previous_state = new_state;
            break;
        } else {
            previous_state = new_state;
        }
    }

    previous_state
        .iter()
        .map(|inner| inner.iter().filter(|c| **c == '#').count())
        .sum()
}

fn num_occupied_complex(previous_state: &[Vec<char>], i: usize, j: usize) -> usize {
    let mut num_occupied = 0;

    for y_dir in [-1, 0, 1] {
        for x_dir in [-1, 0, 1] {
            if y_dir == 0 && x_dir == 0 {
                continue;
            }

            let mut candidate_y = i as i64;
            let mut candidate_x = j as i64;

            loop {
                candidate_y += y_dir;
                candidate_x += x_dir;

                if (candidate_y < 0 || candidate_y >= previous_state.len() as i64)
                    || (candidate_x < 0 || candidate_x >= previous_state[i].len() as i64)
                {
                    break;
                }

                match previous_state[candidate_y as usize][candidate_x as usize] {
                    '#' => {
                        num_occupied += 1;
                        break;
                    }
                    '.' => {}
                    _ => break,
                }
            }
        }
    }

    num_occupied
}

fn part_2(input: &Path) -> usize {
    let mut previous_state = std::fs::read_to_string(input)
        .unwrap()
        .lines()
        .map(|v| v.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    loop {
        let mut new_state = previous_state.clone();

        for i in 0..previous_state.len() {
            for j in 0..previous_state[i].len() {
                match previous_state[i][j] {
                    'L' => {
                        if num_occupied_complex(&previous_state, i, j) == 0 {
                            new_state[i][j] = '#';
                        }
                    }
                    '#' => {
                        if num_occupied_complex(&previous_state, i, j) >= 5 {
                            new_state[i][j] = 'L';
                        }
                    }
                    '.' | _ => {}
                }
            }
        }

        if new_state == previous_state {
            previous_state = new_state;
            break;
        } else {
            previous_state = new_state;
        }
    }

    previous_state
        .iter()
        .map(|inner| inner.iter().filter(|c| **c == '#').count())
        .sum()
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
        assert_eq!(part_1(Path::new("example.txt")), 37);
        assert_eq!(part_2(Path::new("example.txt")), 26);
    }
}
