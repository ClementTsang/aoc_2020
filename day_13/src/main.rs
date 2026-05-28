use helpers::*;
use std::path::Path;

#[derive(Debug)]
struct Input {
    earliest_timestamp: u64,
    bus_ids: Vec<Option<u64>>,
}

fn read_input(input: &Path) -> Input {
    let s = std::fs::read_to_string(input).unwrap();
    let mut lines = s.lines();

    let earliest_timestamp = lines.next().unwrap().parse().unwrap();
    let bus_ids = lines
        .next()
        .unwrap()
        .split(",")
        .map(|s| s.parse().ok())
        .collect();

    Input {
        earliest_timestamp,
        bus_ids,
    }
}

fn part_1(input: &Path) -> u64 {
    let input = read_input(input);
    let mut best_bus = 0;
    let mut best_wait_time = u64::MAX;

    // Basic logic is take modulo and then subtract it from the value. Smallest wins.
    for bus_id in input.bus_ids {
        let Some(bus_id) = bus_id else {
            continue;
        };

        let wait_time = bus_id - (input.earliest_timestamp % bus_id);
        if wait_time < best_wait_time {
            best_bus = bus_id;
            best_wait_time = wait_time;
        }
    }

    best_bus * best_wait_time
}

/// Can solve this with Chinese remainder theorem, surprise.
fn part_2(input: &Path) -> u64 {
    let input = read_input(input);

    let pairs = input
        .bus_ids
        .iter()
        .enumerate()
        .filter_map(|(itx, id)| id.map(|id| (itx as u64, id)))
        .collect::<Vec<_>>();

    let total_product: u64 = pairs.iter().map(|(_, id)| *id).product();
    let products = pairs.iter().map(|(remainder, n)| {
        let b_i = n - remainder % n;
        let n_i = total_product / *n;
        let lhs = n_i % n;

        // Lazy way to calculate the inverse.
        let mut inverse = 1;
        while (lhs * inverse) % n != 1 {
            inverse += 1;
        }

        b_i * n_i * inverse
    });

    // Return the sum mod total_product.
    products.sum::<u64>() % total_product
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
        assert_eq!(part_1(Path::new("example.txt")), 295);
        assert_eq!(part_2(Path::new("example.txt")), 1068781);
    }
}
