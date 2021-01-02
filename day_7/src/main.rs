use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part_1();
    part_2();
}

struct BagContent {
    pub name: String,
    pub children: Vec<(usize, String)>,
}

fn transform_line(s: String) -> BagContent {
    let parts = s.split("contain").collect::<Vec<_>>();
    let name_split = parts[0].split(" ").collect::<Vec<_>>();
    let name: String = name_split[0..2]
        .join(" ")
        .trim_end()
        .trim_start()
        .to_string();
    let contents = parts[1].split(",").collect::<Vec<_>>();
    let mut children = vec![];

    if contents[0].trim_start().trim_end() != "no other bags." {
        for c in contents {
            let split = c.trim_start().trim_end().split(" ").collect::<Vec<_>>();
            let (count, bag_name) = (split[0], split[1..split.len() - 1].join(" "));

            children.push((
                count.parse::<usize>().unwrap(),
                bag_name
                    .trim_start()
                    .trim_end()
                    .trim_end_matches(".")
                    .to_string(),
            ));
        }
    }

    BagContent {
        name: name,
        children,
    }
}

fn part_1() {
    let input = BufReader::new(File::open("input").unwrap());

    // Awful tree substitute.
    let mut tree: HashMap<String, Vec<(usize, String)>> = HashMap::new();

    for line in input.lines() {
        if let Ok(line) = line {
            let bag_content = transform_line(line);

            (*(tree.entry(bag_content.name).or_insert(vec![]))).extend(bag_content.children);
        }
    }

    // Now traverse ""tree""
    let mut count = 0;
    let mut token = vec!["shiny gold".to_string()];
    let mut checked = HashSet::new();
    checked.insert(token[0].clone());

    while !token.is_empty() {
        let val = token.pop().unwrap();

        for (k, v) in &tree {
            for child in v {
                if child.1 == val && !checked.contains(k) {
                    checked.insert(k.clone());
                    count += 1;
                    token.push(k.to_string());
                }
            }
        }
    }

    println!("Part 1: {}", count);
}

fn part_2() {
    let input = BufReader::new(File::open("input").unwrap());

    // Awful tree substitute.
    let mut tree: HashMap<String, Vec<(usize, String)>> = HashMap::new();

    for line in input.lines() {
        if let Ok(line) = line {
            let bag_content = transform_line(line);

            (*(tree.entry(bag_content.name).or_insert(vec![]))).extend(bag_content.children);
        }
    }

    // Now traverse ""tree""
    let mut count = 0;
    let mut token = vec![(1, "shiny gold".to_string())];
    let mut checked: HashSet<String> = HashSet::new();
    checked.insert(token[0].1.clone());

    while !token.is_empty() {
        let (mult, bag) = token.pop().unwrap();

        if let Some(children) = tree.get(&bag) {
            for child in children {
                let (num, child_bag) = child;

                count += num * mult;
                token.push((*num * mult, child_bag.clone()));
            }
        }
    }

    println!("Part 2: {}", count);
}
