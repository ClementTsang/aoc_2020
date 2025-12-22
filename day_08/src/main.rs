use std::{
    collections::HashSet,
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
        "day_08/input.txt".into()
    }
}

fn main() {
    let file = get_input_file();
    let lines = to_lines(&file);

    let ops = lines
        .into_iter()
        .map(|line| {
            let split = line.split(" ").collect::<Vec<_>>();

            let op = split[0];
            let value = split[1];

            let positive = value.starts_with("+");
            let val: usize = value[1..].parse().unwrap();

            (op.to_string(), positive, val)
        })
        .collect::<Vec<_>>();

    {
        let mut seen = HashSet::new();
        let mut pc: usize = 0;
        let mut acc: usize = 0;

        while !seen.contains(&pc) {
            seen.insert(pc);

            let (op, positive, value) = &ops[pc];

            match op.as_str() {
                "acc" => {
                    if *positive {
                        acc += value;
                    } else {
                        acc -= value;
                    }
                    pc += 1;
                }
                "jmp" => {
                    if *positive {
                        pc += value;
                    } else {
                        pc -= value;
                    }
                }
                "nop" => {
                    pc += 1;
                }
                _ => unreachable!(),
            }
        }

        println!("Part 1: {acc}");
    }

    {
        for i in 0..ops.len() {
            if ops[i].0 == "jmp" || ops[i].0 == "nop" {
                let was_jmp = ops[i].0 == "jmp";

                let mut seen = HashSet::new();
                let mut pc: usize = 0;
                let mut acc: usize = 0;

                while !seen.contains(&pc) {
                    seen.insert(pc);

                    let (op, positive, value) = &ops[pc];
                    let op = if pc == i {
                        if was_jmp { "nop" } else { "jmp" }
                    } else {
                        op.as_str()
                    };

                    match op {
                        "acc" => {
                            if *positive {
                                acc += value;
                            } else {
                                acc -= value;
                            }
                            pc += 1;
                        }
                        "jmp" => {
                            if *positive {
                                pc += value;
                            } else {
                                pc -= value;
                            }
                        }
                        "nop" => {
                            pc += 1;
                        }
                        _ => unreachable!(),
                    }

                    if pc == ops.len() {
                        println!("Part 2: {acc}");
                        return;
                    }
                }
            }
        }
    }
}
