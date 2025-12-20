use std::fs::File;
use std::io::{BufRead, BufReader};

const BIRTH_YEAR: &str = "byr";
const ISSUE_YEAR: &str = "iyr";
const EXPIRATION_YEAR: &str = "eyr";
const HEIGHT: &str = "hgt";
const HAIR_COLOR: &str = "hcl";
const EYE_COLOR: &str = "ecl";
const PASSPORT_ID: &str = "pid";

fn main() {
    // part one
    // part_one();

    // part two
    part_two();
}

fn part_one() {
    let input = BufReader::new(File::open("input.txt").unwrap());

    let mut seen_birth_year = false;
    let mut seen_issue_year = false;
    let mut seen_exp_year = false;
    let mut seen_height = false;
    let mut seen_hair_color = false;
    let mut seen_eye_color = false;
    let mut seen_pid = false;

    let mut valid = 0;
    let mut total = 0;

    for line in input.lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                // Then we've hit a blank line.  Check!

                if seen_birth_year
                    && seen_issue_year
                    && seen_exp_year
                    && seen_height
                    && seen_hair_color
                    && seen_eye_color
                    && seen_pid
                {
                    valid += 1;
                }
                total += 1;

                seen_birth_year = false;
                seen_issue_year = false;
                seen_exp_year = false;
                seen_height = false;
                seen_hair_color = false;
                seen_eye_color = false;
                seen_pid = false;
            } else {
                // Delimit by spaces
                for token in line.split_ascii_whitespace() {
                    let s = *token.split(":").collect::<Vec<_>>().first().unwrap();
                    match s {
                        BIRTH_YEAR => seen_birth_year = true,
                        ISSUE_YEAR => seen_issue_year = true,
                        EXPIRATION_YEAR => seen_exp_year = true,
                        HEIGHT => seen_height = true,
                        HAIR_COLOR => seen_hair_color = true,
                        EYE_COLOR => seen_eye_color = true,
                        PASSPORT_ID => seen_pid = true,
                        _ => {}
                    }
                }
            }
        }
    }

    // Check one last time
    if seen_birth_year
        && seen_issue_year
        && seen_exp_year
        && seen_height
        && seen_hair_color
        && seen_eye_color
        && seen_pid
    {
        valid += 1;
    }
    total += 1;

    println!("Valid: {} out of {}", valid, total);
}

fn part_two() {
    let input = BufReader::new(File::open("input.txt").unwrap());

    let mut seen_birth_year = false;
    let mut seen_issue_year = false;
    let mut seen_exp_year = false;
    let mut seen_height = false;
    let mut seen_hair_color = false;
    let mut seen_eye_color = false;
    let mut seen_pid = false;

    let mut valid = 0;
    let mut total = 0;

    for line in input.lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                // Then we've hit a blank line.  Check!

                if seen_birth_year
                    && seen_issue_year
                    && seen_exp_year
                    && seen_height
                    && seen_hair_color
                    && seen_eye_color
                    && seen_pid
                {
                    valid += 1;
                }
                total += 1;

                seen_birth_year = false;
                seen_issue_year = false;
                seen_exp_year = false;
                seen_height = false;
                seen_hair_color = false;
                seen_eye_color = false;
                seen_pid = false;
            } else {
                // Delimit by spaces
                for token in line.split_ascii_whitespace() {
                    let tok_split = token.split(":").collect::<Vec<_>>();
                    let s = *tok_split.first().unwrap();
                    let t = *tok_split.get(1).unwrap();

                    match s {
                        BIRTH_YEAR => {
                            if t.len() == 4 {
                                if let Ok(byr) = t.parse::<usize>() {
                                    if byr >= 1920 && byr <= 2002 {
                                        seen_birth_year = true;
                                    }
                                }
                            }
                        }
                        ISSUE_YEAR => {
                            if t.len() == 4 {
                                if let Ok(iyr) = t.parse::<usize>() {
                                    if iyr >= 2010 && iyr <= 2020 {
                                        seen_issue_year = true;
                                    }
                                }
                            }
                        }
                        EXPIRATION_YEAR => {
                            if t.len() == 4 {
                                if let Ok(eyr) = t.parse::<usize>() {
                                    if eyr >= 2020 && eyr <= 2030 {
                                        seen_exp_year = true;
                                    }
                                }
                            }
                        }
                        HEIGHT => {
                            if t.ends_with("in") {
                                let hgt = &t[..(t.len() - 2)];
                                if let Ok(hgt) = hgt.parse::<usize>() {
                                    if hgt >= 59 && hgt <= 76 {
                                        seen_height = true
                                    }
                                }
                            } else if t.ends_with("cm") {
                                let hgt = &t[..(t.len() - 2)];
                                if let Ok(hgt) = hgt.parse::<usize>() {
                                    if hgt >= 150 && hgt <= 193 {
                                        seen_height = true
                                    }
                                }
                            }
                        }
                        HAIR_COLOR => {
                            if t.starts_with("#") && t.len() == 7 {
                                for c in t.chars() {
                                    if c.is_numeric() || ['a', 'b', 'c', 'd', 'e', 'f'].contains(&c)
                                    {
                                        seen_hair_color = true;
                                    }
                                }
                            }
                        }
                        EYE_COLOR => {
                            if ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&t) {
                                seen_eye_color = true;
                            }
                        }
                        PASSPORT_ID => {
                            if t.len() == 9 {
                                seen_pid = true;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    // Check one last time
    if seen_birth_year
        && seen_issue_year
        && seen_exp_year
        && seen_height
        && seen_hair_color
        && seen_eye_color
        && seen_pid
    {
        valid += 1;
    }
    total += 1;

    println!("Valid: {} out of {}", valid, total);
}
