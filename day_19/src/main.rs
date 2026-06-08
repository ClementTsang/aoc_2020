use helpers::*;
use std::{collections::HashMap, path::Path};

#[derive(Debug)]
enum Rule {
    Pattern(char),
    /// A bunch of ANDs OR'd together (e.g. `2 3 | 3 2` -> `[[2, 3], [3, 2]]`).
    SubRules(Vec<Vec<u64>>),
}

fn flatten(rules: &HashMap<u64, Rule>, contents: &Vec<Vec<u64>>) -> String {
    let mut and_rules = vec![];

    for or_rule in contents {
        let mut or_str = String::new();
        for and_rule in or_rule {
            match rules.get(and_rule).unwrap() {
                Rule::Pattern(c) => or_str.push(*c),
                Rule::SubRules(items) => {
                    or_str.push('(');
                    or_str.push_str(flatten(rules, items).as_str());
                    or_str.push(')');
                }
            }
        }

        and_rules.push(or_str);
    }

    let ret = format!("({})", and_rules.join("|"));
    ret
}

fn parse_input(input: &Path) -> (HashMap<u64, Rule>, Vec<String>) {
    let mut rules: HashMap<u64, Rule> = HashMap::default();
    let mut to_check = vec![];
    let mut checking_messages = false;

    for line in std::fs::read_to_string(input).unwrap().lines() {
        if line.is_empty() {
            checking_messages = true;
            continue;
        }

        if !checking_messages {
            let (index, rule) = line.split_once(": ").unwrap();
            let index: u64 = index.trim().parse().unwrap();

            if rule.starts_with('"') {
                let rule = rule.trim_matches('"');
                rules.insert(index, Rule::Pattern(rule.chars().next().unwrap()));
            } else {
                let sub_rules = rule
                    .split('|')
                    .map(|inner_rule| {
                        inner_rule
                            .trim()
                            .split(' ')
                            .map(|r| r.parse::<u64>().unwrap())
                            .collect::<Vec<_>>()
                    })
                    .collect();

                rules.insert(index, Rule::SubRules(sub_rules));
            }
        } else {
            to_check.push(line.to_string());
        }
    }

    (rules, to_check)
}

fn part_1(input: &Path) -> usize {
    let (rules, to_check) = parse_input(input);

    // Assemble rule 0 by flattening it. Cheaty way to do it with regexes.
    let Rule::SubRules(rule_0_contents) = rules.get(&0).unwrap() else {
        panic!("rule 0 must contain subrules")
    };

    let rule_0_regex = format!(r"^{}$", flatten(&rules, rule_0_contents));
    let r = regex::Regex::new(&rule_0_regex).unwrap();

    to_check.iter().filter(|s| r.is_match(s)).count()
}

fn part_2(input: &Path) -> usize {
    let (rules, to_check) = parse_input(input);

    let rule_31 = {
        let Rule::SubRules(rule_contents) = rules.get(&31).unwrap() else {
            panic!("rule 31 must exist and contain subcontents")
        };

        flatten(&rules, rule_contents)
    };

    let rule_42 = {
        let Rule::SubRules(rule_contents) = rules.get(&42).unwrap() else {
            panic!("rule 42 must exist and contain subcontents")
        };

        flatten(&rules, rule_contents)
    };

    // Since rule 0 -> 8 11 in both the second example and input, and
    // we know that 8 -> {rule_42}+ and 11 -> {rule_42}{n} {rule_31}{n} for some n
    // we can just manually define the regex for it. And we can just set some high n I guess.

    (1..20)
        .map(|n| {
            let rule_0_regex = format!(r"^{rule_42}+{rule_42}{{{n}}}{rule_31}{{{n}}}$");
            let r = regex::Regex::new(&rule_0_regex).unwrap();
            to_check.iter().filter(|s| r.is_match(s)).count()
        })
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
        assert_eq!(part_1(Path::new("example.txt")), 2);
        assert_eq!(part_1(Path::new("example_2.txt")), 3);
        assert_eq!(part_2(Path::new("example_2.txt")), 12);
    }
}
