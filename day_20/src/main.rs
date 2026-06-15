use helpers::*;
use std::path::Path;

fn part_1(input: &Path) -> u64 {
    0
}

fn part_2(input: &Path) -> u64 {
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
        assert_eq!(part_1(Path::new("example.txt")), 0);
        assert_eq!(part_2(Path::new("example.txt")), 0);
    }
}
