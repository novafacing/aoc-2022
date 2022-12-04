use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let contained: u32 = input
        .lines()
        .map(|l| {
            let ranges: Vec<Vec<u32>> = l
                .split(',')
                .map(|s| {
                    s.trim()
                        .split('-')
                        .map(|s| s.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>()
                })
                .take(2)
                .collect();
            (ranges[0].clone(), ranges[1].clone())
        })
        .map(|(lr, rr)| {
            if lr[1] >= rr[0] && lr[1] <= rr[1] {
                1
            } else if lr[0] >= rr[0] && lr[0] <= rr[1] {
                1
            } else if lr[0] <= rr[0] && lr[1] >= rr[1] {
                1
            } else {
                0
            }
        })
        .sum();

    println!("{}", contained);
}
