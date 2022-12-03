use std::io::{stdin, Read};

/// Read lines from stdin

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let sacks: u64 = input
        .lines()
        .map(|line| {
            // split line into two equal sized strings
            let (left, right) = line.split_at(line.len() / 2);

            // convert strings to u64 sequences
            // a-z: 1-26
            // A-Z: 27-52

            let left: Vec<u64> = left
                .chars()
                .map(|c| match c.is_ascii_lowercase() {
                    true => c as u64 - b'a' as u64 + 1,
                    false => c as u64 - b'A' as u64 + 27,
                })
                .collect();

            let right: Vec<u64> = right
                .chars()
                .map(|c| match c.is_ascii_lowercase() {
                    true => c as u64 - b'a' as u64 + 1,
                    false => c as u64 - b'A' as u64 + 27,
                })
                .collect();

            // iterate over common items in both sequences
            // and sum their values
            let common = left
                .iter()
                .filter(|&x| right.contains(x))
                .take(1)
                .next()
                .unwrap_or(&0);

            common.clone()
        })
        .sum();
    println!("{}", sacks);
}
