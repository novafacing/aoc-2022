use std::io::{stdin, Read};

use itertools::Itertools;

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut sum = 0;
    input
        .lines()
        .filter_map(|l| {
            if !l.trim().is_empty() {
                Some(l.trim())
            } else {
                None
            }
        })
        .tuples()
        .inspect(|(a, b, c)| {
            let common = a
                .chars()
                .filter(|&x| b.contains(x) && c.contains(x))
                .take(1)
                .next()
                .unwrap();

            sum += match common.is_ascii_lowercase() {
                true => common as u64 - b'a' as u64 + 1,
                false => common as u64 - b'A' as u64 + 27,
            };
        })
        .for_each(drop);
    println!("{}", sum);
}
