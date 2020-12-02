use itertools::Itertools;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input1.txt").unwrap();
    let final_val = 2020;

    let sorted_collection = contents
        .split("\n")
        .map(|s| s.parse::<u32>().unwrap_or(0))
        .sorted()
        .collect::<Vec<_>>();

    // two_sum(final_val, &sorted_collection);
    three_sum(final_val, &sorted_collection);
}

fn two_sum(final_val: u32, sorted_collection: &[u32]) -> Option<(u32, u32)> {
    let mut lhs = 0;
    let mut rhs = sorted_collection.len() - 1;

    while lhs < rhs {
        let sum = sorted_collection[lhs] + sorted_collection[rhs];
        if sum > final_val {
            rhs -= 1;
        } else if sum < final_val {
            lhs += 1;
        } else {
            println!(
                "Done.  Num 1: {}, num 2: {}",
                sorted_collection[lhs], sorted_collection[rhs]
            );
            println!(
                "Product is therefore {}",
                sorted_collection[lhs] * sorted_collection[rhs]
            );

            return Some((sorted_collection[lhs], sorted_collection[rhs]));
        }
    }

    None
}

fn three_sum(final_val: u32, sorted_collection: &Vec<u32>) {
    for itx in 0..sorted_collection.len() - 2 {
        let sliced_vec = &sorted_collection[itx..];
        if let Some((num_one, num_two)) = two_sum(final_val - sorted_collection[itx], sliced_vec) {
            println!(
                "Done.  Num 1: {}, num 2: {}, num 3: {}",
                num_one, num_two, sorted_collection[itx]
            );
            println!("Product: {}", num_one * num_two * sorted_collection[itx]);
        }
    }
}
