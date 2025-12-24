use std::{
    collections::VecDeque,
    env,
    fs::read_to_string,
    path::{Path, PathBuf},
};

fn to_lines(path: &Path) -> Vec<String> {
    println!("Reading from {}", path.display());

    read_to_string(path)
        .unwrap()
        .split("\n")
        .map(|v| v.to_string())
        .filter(|v| !v.is_empty())
        .collect()
}

fn get_input_file() -> PathBuf {
    let args: Vec<String> = env::args().collect();

    if let Some(file) = args.get(1) {
        Path::new(file).to_path_buf()
    } else {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        format!("{manifest_dir}/input.txt").into()
    }
}

fn valid_sum(horizon: usize, values: &[u64], current: usize) -> bool {
    let target = values[current];
    let to_consider = &values[current - horizon..current];

    for i in 0..(to_consider.len() - 1) {
        for j in (i + 1)..(to_consider.len()) {
            if to_consider[i] + to_consider[j] == target {
                return true;
            }
        }
    }

    false
}

fn main() {
    let file = get_input_file();
    let lines = to_lines(&file);

    let horizon: usize = if file.to_str().unwrap().contains("example.txt") {
        5
    } else {
        25
    };

    let values = lines
        .iter()
        .map(|v| v.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut invalid_number = 0;
    for (itx, &value) in values.iter().enumerate().skip(horizon) {
        if !valid_sum(horizon, &values, itx) {
            println!("Part one: {value}");

            invalid_number = value;
            break;
        }
    }

    let mut trail: VecDeque<u64> = VecDeque::default();

    for value in values {
        let current_sum = trail.iter().sum::<u64>();
        if current_sum + value == invalid_number && trail.len() > 1 {
            trail.push_back(value);

            let smallest = trail.iter().min().unwrap();
            let largest = trail.iter().max().unwrap();

            println!(
                "Part two: {} ({}, {})",
                smallest + largest,
                smallest,
                largest
            );
            break;
        } else if current_sum + value < invalid_number {
            trail.push_back(value);
        } else {
            trail.push_back(value);
            trail.pop_front();

            while trail.iter().sum::<u64>() > invalid_number {
                trail.pop_front();
            }
        }

        if trail.iter().sum::<u64>() == invalid_number && trail.len() > 1 {
            let smallest = trail.iter().min().unwrap();
            let largest = trail.iter().max().unwrap();

            println!(
                "Part two: {} ({}, {})",
                smallest + largest,
                smallest,
                largest
            );
            break;
        }
    }
}
