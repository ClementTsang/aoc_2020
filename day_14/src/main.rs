use helpers::*;
use std::{collections::HashMap, path::Path};

#[derive(Debug, PartialEq, Eq)]
enum Bit {
    Zero,
    One,
    X,
}

#[derive(Debug)]
struct Program {
    mask: Vec<(u32, Bit)>,
    instructions: Vec<(u64, u64)>,
}

impl Program {
    fn new(mask_str: &str) -> Self {
        let mut mask: Vec<(u32, Bit)> = vec![];

        for (itx, c) in mask_str.chars().rev().enumerate() {
            if c == '0' {
                mask.push((itx as u32, Bit::Zero));
            } else if c == '1' {
                mask.push((itx as u32, Bit::One));
            } else {
                mask.push((itx as u32, Bit::X));
            }
        }

        Program {
            mask,
            instructions: vec![],
        }
    }
}

fn read_input(input: &Path) -> Vec<Program> {
    let s = std::fs::read_to_string(input).unwrap();
    let mut programs = vec![];
    let mut current_program = None;

    for line in s.lines() {
        let Some((inst, value)) = line.split_once(" = ") else {
            break;
        };

        if inst == "mask" {
            let to_push = current_program.take();
            if let Some(to_push) = to_push {
                programs.push(to_push);
            }

            current_program = Some(Program::new(value));
        } else {
            // Memory address write of the form mem[8]
            let inst = inst.trim_start_matches("mem[");
            let inst = inst.trim_end_matches("]");

            let addr: u64 = inst.parse().unwrap();
            let value: u64 = value.parse().unwrap();

            current_program
                .as_mut()
                .unwrap()
                .instructions
                .push((addr, value));
        }
    }

    if let Some(to_push) = current_program {
        programs.push(to_push);
    }

    programs
}

fn part_1(input: &Path) -> u64 {
    let programs = read_input(input);

    // mapping of memory address -> 36-bit value
    let mut memory: HashMap<u64, u64> = HashMap::new();

    for program in &programs {
        let Program { mask, instructions } = program;

        for (address, value) in instructions {
            let v = memory.entry(*address).or_insert(0);

            let mut to_write = *value;
            for (bit, val) in mask {
                match *val {
                    Bit::Zero => {
                        to_write &= !(1 << *bit);
                    }
                    Bit::One => {
                        to_write |= 1 << *bit;
                    }
                    Bit::X => {} // Ignored
                }
            }
            *v = to_write;
        }
    }

    memory
        .values()
        .map(|v| {
            let mut v = *v;
            for i in 36..64 {
                v &= !(1 << i);
            }

            v
        })
        .sum()
}

fn part_2(input: &Path) -> u64 {
    let programs = read_input(input);

    // mapping of memory address -> 36-bit values
    let mut memory: HashMap<u64, u64> = HashMap::new();

    for program in &programs {
        let Program { mask, instructions } = program;

        for (address, value) in instructions {
            let mut new_addresses = vec![*address];
            for (bit, val) in mask {
                match *val {
                    Bit::Zero => {}
                    Bit::One => {
                        for addr in &mut new_addresses {
                            *addr |= 1 << *bit;
                        }
                    }
                    Bit::X => {
                        let mut new_new_addresses = Vec::with_capacity(new_addresses.len() * 2);
                        for addr in &new_addresses {
                            let mut a = *addr;
                            a &= !(1 << *bit);

                            let mut b = *addr;
                            b |= 1 << *bit;

                            new_new_addresses.push(a);
                            new_new_addresses.push(b);
                        }

                        new_addresses = new_new_addresses;
                    }
                }
            }

            for addr in new_addresses {
                let vs = memory.entry(addr).or_insert(0);
                *vs = *value;
            }
        }
    }

    memory
        .values()
        .map(|v| {
            let mut v = *v;
            for i in 36..64 {
                v &= !(1 << i);
            }

            v
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
        assert_eq!(part_1(Path::new("example.txt")), 165);
    }

    #[test]
    fn example_2() {
        assert_eq!(part_2(Path::new("example_2.txt")), 208);
    }
}
