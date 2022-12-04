// Given an input of sets of numbers like so:
// 1000
// 2000
// 3000
//
// 4000
//
// 5000
// 6000
//
// 7000
// 8000
// 9000
//
// 10000

// Find the total sum of the three largest groups of numbers and output the sum

use std::io::{stdin, Read};

use itertools::Itertools;

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    input
        .lines()
        .filter_map(|l| {
            if !l.trim().is_empty() {
                Some(l.trim())
            } else {
                None
            }
        })
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|c| c.iter().map(|l| l.parse::<u64>().unwrap()).sum::<u64>())
        .sorted()
        .rev()
        .take(3)
        .sum::<u64>();
    let mut lines = input.lines();
    let mut max = [0, 0, 0];
    let mut sum = 0;
    while let Some(line) = lines.next() {
        if line.is_empty() {
            if sum > max[0] {
                max[0] = sum;
                max.sort();
            }
            sum = 0;
        } else {
            sum += line.parse::<i32>().unwrap();
        }
    }
    println!("{}", max[0] + max[1] + max[2]);
}
