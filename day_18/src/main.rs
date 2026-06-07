use helpers::*;
use std::{collections::VecDeque, path::PathBuf};

#[derive(Clone)]
enum PathOrString {
    Path(PathBuf),
    String(String),
}

impl From<PathBuf> for PathOrString {
    fn from(value: PathBuf) -> Self {
        PathOrString::Path(value)
    }
}

impl From<&str> for PathOrString {
    fn from(value: &str) -> Self {
        PathOrString::String(value.to_string())
    }
}

impl PathOrString {
    fn to_string(&self) -> String {
        match self {
            PathOrString::Path(path) => std::fs::read_to_string(&path).unwrap().trim().to_string(),
            PathOrString::String(s) => s.trim().to_owned(),
        }
    }
}

fn tokenize(input: &str) -> VecDeque<String> {
    let mut tokens: VecDeque<String> = VecDeque::new();

    for s in input.split_whitespace() {
        let mut last = 0;
        for (index, matched) in s.match_indices(&[')', '(']) {
            if last != index {
                tokens.push_back(s[last..index].to_string());
            }
            tokens.push_back(matched.to_string());
            last = index + matched.len();
        }
        if last < s.len() {
            tokens.push_back(s[last..].to_string());
        }
    }

    tokens
}

fn evaluate(tokens: &mut VecDeque<String>) -> u64 {
    #[derive(Debug)]
    enum Operator {
        Addition,
        Multiplication,
    }

    impl std::fmt::Display for Operator {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Operator::Addition => write!(f, "+"),
                Operator::Multiplication => write!(f, "*"),
            }
        }
    }

    let mut lhs = 0;
    let mut operator = None;

    while let Some(curr) = tokens.pop_front() {
        if curr == "(" {
            let rhs = evaluate(tokens);
            if let Some(operator) = operator.take() {
                lhs = match operator {
                    Operator::Addition => lhs + rhs,
                    Operator::Multiplication => lhs * rhs,
                };
            } else {
                lhs = rhs;
            }
        } else if curr == ")" {
            return lhs;
        } else if let Ok(curr) = curr.parse::<u64>() {
            if let Some(operator) = operator.take() {
                lhs = match operator {
                    Operator::Addition => lhs + curr,
                    Operator::Multiplication => lhs * curr,
                };
            } else {
                lhs = curr;
            }
        } else if curr == "*" {
            operator = Some(Operator::Multiplication);
        } else if curr == "+" {
            operator = Some(Operator::Addition);
        } else {
            unreachable!("invalid token")
        }
    }

    lhs
}

fn part_1(input: &PathOrString) -> u64 {
    let input = input.to_string();
    let mut sum = 0;

    for input in input.split("\n") {
        let mut tokens = tokenize(input);
        sum += evaluate(&mut tokens);
    }

    sum
}

/// Shunting yard implementation with reversed precedence for + and *.
fn evaluate_2(tokens: &mut VecDeque<String>) -> u64 {
    #[derive(Debug)]
    enum Output {
        Value(u64),
        Addition,
        Multiplication,
    }

    let mut output = VecDeque::new();
    let mut stack = VecDeque::new();

    while let Some(curr) = tokens.pop_front() {
        if curr == "(" {
            stack.push_front('(');
        } else if curr == ")" {
            while let Some(popped) = stack.pop_front() {
                if popped == '(' {
                    break;
                } else {
                    match popped {
                        '+' => output.push_back(Output::Addition),
                        '*' => output.push_back(Output::Multiplication),
                        _ => unreachable!("invalid operation"),
                    }
                }
            }
        } else if let Ok(curr) = curr.parse::<u64>() {
            output.push_back(Output::Value(curr));
        } else if curr == "*" {
            while let Some(&front) = stack.front() {
                if front == '(' {
                    break;
                } else if front == '+' || front == '*' {
                    let popped = stack.pop_front().unwrap();
                    match popped {
                        '+' => output.push_back(Output::Addition),
                        '*' => output.push_back(Output::Multiplication),
                        _ => unreachable!("invalid operation"),
                    }
                }
            }

            stack.push_front('*');
        } else if curr == "+" {
            while let Some(&front) = stack.front() {
                if front == '(' {
                    break;
                } else if front == '+' {
                    stack.pop_front().unwrap();
                    output.push_back(Output::Addition)
                } else if front == '*' {
                    break;
                }
            }

            stack.push_front('+');
        } else {
            unreachable!("invalid token")
        }
    }

    for s in stack {
        match s {
            '+' => output.push_back(Output::Addition),
            '*' => output.push_back(Output::Multiplication),
            _ => {}
        }
    }

    let mut eval_queue = vec![];
    for item in output {
        match item {
            Output::Value(val) => eval_queue.push(val),
            Output::Addition => {
                let rhs = eval_queue.pop().unwrap();
                let lhs = eval_queue.pop().unwrap();
                eval_queue.push(lhs + rhs);
            }
            Output::Multiplication => {
                let rhs = eval_queue.pop().unwrap();
                let lhs = eval_queue.pop().unwrap();
                eval_queue.push(lhs * rhs);
            }
        }
    }

    *eval_queue.first().unwrap()
}

fn part_2(input: &PathOrString) -> u64 {
    let input = input.to_string();
    let mut sum = 0;

    for input in input.split("\n") {
        let mut tokens = tokenize(&input);
        sum += evaluate_2(&mut tokens);
    }

    sum
}

fn main() {
    let input = PathOrString::Path(get_input_file());

    println!("Part one: {}", part_1(&input));
    println!("Part two: {}", part_2(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(part_1(&PathBuf::from("example.txt").into()), 71);
        assert_eq!(part_1(&PathBuf::from("example_2.txt").into()), 51);
        assert_eq!(part_1(&"2 * 3 + (4 * 5)".into()), 26);
        assert_eq!(part_1(&"5 + (8 * 3 + 9 + 3 * 4 * 3)".into()), 437);
        assert_eq!(
            part_1(&"5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".into()),
            12240
        );
        assert_eq!(
            part_1(&"((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".into()),
            13632
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(part_2(&PathBuf::from("example.txt").into()), 231);
        assert_eq!(part_2(&PathBuf::from("example_2.txt").into()), 51);
        assert_eq!(part_2(&"2 * 3 + (4 * 5)".into()), 46);
        assert_eq!(part_2(&"5 + (8 * 3 + 9 + 3 * 4 * 3)".into()), 1445);
        assert_eq!(
            part_2(&"5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".into()),
            669060
        );
        assert_eq!(
            part_2(&"((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".into()),
            23340
        );
    }
}
