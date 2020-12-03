use std::collections::HashMap;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input2.txt").unwrap();

    // part_1(&contents);

    part_2(&contents);
}

fn part_1(contents: &String) {
    #[derive(Debug)]
    struct PasswordLine {
        pub low: u32,
        pub hi: u32,
        pub rune: char,
        pub password_map: HashMap<char, u32>,
    }

    let lines = contents.split_terminator("\n").map(|s| {
        let mut split_line = s.split_ascii_whitespace();

        let low_hi = split_line.next().unwrap().split('-').collect::<Vec<_>>();

        PasswordLine {
            low: low_hi[0].parse::<u32>().unwrap(),
            hi: low_hi[1].parse::<u32>().unwrap(),
            rune: split_line
                .next()
                .unwrap()
                .to_string()
                .chars()
                .next()
                .unwrap(),
            password_map: {
                let mut map = HashMap::new();

                for c in split_line.next().unwrap().to_string().chars() {
                    let entry = map.entry(c).or_insert(0);
                    *entry += 1;
                }

                map
            },
        }
    });

    let mut valid_passwords = 0;
    for line in lines {
        // println!("Line: {:?}", line);

        let count = *line.password_map.get(&line.rune).unwrap_or(&0);

        if count >= line.low && count <= line.hi {
            valid_passwords += 1;
        }
    }

    println!("Valid passwords: {}", valid_passwords);
}

fn part_2(contents: &String) {
    let mut valid_passwords = 0;

    contents.split_terminator("\n").for_each(|s| {
        let mut split_line = s.split_ascii_whitespace();

        let low_hi = split_line.next().unwrap().split('-').collect::<Vec<_>>();
        let low = low_hi[0].parse::<usize>().unwrap() - 1;
        let hi = low_hi[1].parse::<usize>().unwrap() - 1;

        let rune = split_line
            .next()
            .unwrap()
            .to_string()
            .chars()
            .next()
            .unwrap();

        let password = split_line.next().unwrap().chars().collect::<Vec<char>>();
        let char_low = password.get(low);
        let char_high = password.get(hi);

        if let (Some(l), Some(h)) = (char_low, char_high) {
            if (*l == rune && *h != rune) || (*l != rune && *h == rune) {
                valid_passwords += 1;
            }
        } else if let Some(l) = char_low {
            if *l == rune {
                valid_passwords += 1;
            }
        } else if let Some(h) = char_high {
            if *h == rune {
                valid_passwords += 1;
            }
        }
    });

    println!("Valid passwords: {}", valid_passwords);
}
