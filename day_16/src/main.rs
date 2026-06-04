use std::{collections::HashMap, ops::RangeInclusive, path::Path};

use helpers::*;

type Fields = HashMap<String, (RangeInclusive<u64>, RangeInclusive<u64>)>;

#[derive(Debug)]
struct Notes {
    fields: Fields,
    your_ticket: Vec<u64>,
    nearby_tickets: Vec<Vec<u64>>,
}

fn parse_notes(input: &Path) -> Notes {
    let lines = std::fs::read_to_string(input).unwrap();

    #[derive(Default, Debug)]
    enum ParsingState {
        #[default]
        TicketFields,
        YourTicket,
        NearbyTickets,
    }

    let mut state = ParsingState::default();

    let mut fields = HashMap::new();
    let mut your_ticket = vec![];
    let mut nearby_tickets = vec![];

    for line in lines.lines() {
        if line.is_empty() {
            continue;
        }

        match line {
            "your ticket:" => {
                state = ParsingState::YourTicket;
                continue;
            }
            "nearby tickets:" => {
                state = ParsingState::NearbyTickets;
                continue;
            }
            _ => {}
        }

        match state {
            ParsingState::TicketFields => {
                let (name, values) = line.split_once(":").unwrap();
                let (range_one, range_two) = values.split_once(" or ").unwrap();
                let (r1s, r1e) = range_one.split_once("-").unwrap();
                let (r2s, r2e) = range_two.split_once("-").unwrap();

                let range_one = (r1s.trim().parse().unwrap())..=(r1e.trim().parse().unwrap());
                let range_two = (r2s.trim().parse().unwrap())..=(r2e.trim().parse().unwrap());

                fields.insert(name.to_string(), (range_one, range_two));
            }
            ParsingState::YourTicket => {
                your_ticket = line
                    .split(",")
                    .map(|v| v.trim().parse::<u64>().unwrap())
                    .collect();
            }
            ParsingState::NearbyTickets => {
                nearby_tickets.push(
                    line.split(",")
                        .map(|v| v.trim().parse::<u64>().unwrap())
                        .collect(),
                );
            }
        }
    }

    Notes {
        fields,
        your_ticket,
        nearby_tickets,
    }
}

fn part_1(input: &Path) -> u64 {
    let Notes {
        fields,
        your_ticket,
        nearby_tickets,
    } = parse_notes(input);

    let mut invalid_sum = 0;
    for ticket in std::iter::once(your_ticket).chain(nearby_tickets) {
        for value in ticket {
            if !fields.values().any(|(range_one, range_two)| {
                range_one.contains(&value) || range_two.contains(&value)
            }) {
                invalid_sum += value;
            }
        }
    }

    invalid_sum
}

fn part_2(input: &Path, keyword: &str) -> u64 {
    let Notes {
        fields,
        your_ticket,
        nearby_tickets,
    } = parse_notes(input);

    let filtered_tickets = nearby_tickets
        .into_iter()
        .filter(|ticket| {
            ticket.into_iter().all(|value| {
                fields.values().any(|(range_one, range_two)| {
                    range_one.contains(&value) || range_two.contains(&value)
                })
            })
        })
        .collect::<Vec<_>>();

    // Awful brute force search. Woo.
    fn search(
        fields: &Fields,
        field_names: Vec<String>,
        current_ordering: Vec<String>,
        filtered_tickets: &[Vec<u64>],
    ) -> Option<Vec<String>> {
        println!("current_ordering: {current_ordering:?}");
        if field_names.is_empty() {
            return Some(current_ordering);
        }

        for itx in 0..field_names.len() {
            let field_name = field_names[itx].clone();
            let ranges = fields.get(&field_name).unwrap();

            if !filtered_tickets.iter().all(|ticket| {
                ranges.0.contains(&ticket[current_ordering.len()])
                    || ranges.1.contains(&ticket[current_ordering.len()])
            }) {
                continue;
            }

            let mut field_names = field_names.clone();
            field_names.remove(itx);

            let mut current_ordering = current_ordering.clone();
            current_ordering.push(field_name);

            if let Some(result) = search(fields, field_names, current_ordering, filtered_tickets) {
                return Some(result);
            }
        }

        None
    }

    let field_names = fields.keys().cloned().collect::<Vec<_>>();
    let current_ordering = search(&fields, field_names, vec![], &filtered_tickets).unwrap();

    current_ordering
        .iter()
        .enumerate()
        .filter_map(|(itx, field)| {
            if field.starts_with(keyword) {
                Some(your_ticket[itx])
            } else {
                None
            }
        })
        .product()
}

fn main() {
    let input = get_input_file();

    println!("Part one: {}", part_1(&input));
    println!("Part two: {}", part_2(&input, "departure"));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(part_1(Path::new("example.txt")), 71);
        assert_eq!(part_2(Path::new("example_2.txt"), "class"), 12);
        assert_eq!(part_2(Path::new("example_2.txt"), "row"), 11);
        assert_eq!(part_2(Path::new("example_2.txt"), "seat"), 13);
    }
}
