use std::io::{stdin, Read};

use itertools::Itertools;

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let (stacks_contents, instructions) = input.split("\n\n").next_tuple().unwrap();

    let mut cols: Vec<Vec<char>> = Vec::new();
    // The columns look like this in text:
    //
    //             [J]         [B]     [T]
    //         [M] [L]     [Q] [L] [R]
    //         [G] [Q]     [W] [S] [B] [L]
    // [D]     [D] [T]     [M] [G] [V] [P]
    // [T]     [N] [N] [N] [D] [J] [G] [N]
    // [W] [H] [H] [S] [C] [N] [R] [W] [D]
    // [N] [P] [P] [W] [H] [H] [B] [N] [G]
    // [L] [C] [W] [C] [P] [T] [M] [Z] [W]
    //  1   2   3   4   5   6   7   8   9

    // We parse those columns from stacks_contents into cols
    stacks_contents.lines().for_each(|l| {
        l.chars().enumerate().for_each(|(i, c)| {
            if cols.len() <= i {
                cols.push(Vec::new());
            }
            cols[i].push(c);
        });
    });

    let mut cols = cols
        .iter()
        .filter(|c| c.iter().filter(|&cc| cc.is_alphabetic()).count() > 0)
        .map(|c| {
            c.iter()
                .filter(|&cc| cc.is_alphabetic())
                .map(|&cc| cc)
                .collect::<Vec<char>>()
        })
        .map(|mut v| {
            v.reverse();
            v
        })
        .collect::<Vec<Vec<char>>>();

    println!("{:?}", cols);
    instructions.lines().for_each(|l| {
        let (n, f, t) = l
            .split(' ')
            .filter(|c| c.chars().filter(|&cc| cc.is_numeric()).count() == c.len())
            .map(|c| c.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();
        let mut top = Vec::new();
        for _ in 0..n {
            top.push(cols[f - 1].pop().unwrap());
        }
        top.reverse();
        cols[t - 1].append(&mut top);
        println!("{:?}", cols);
    });

    println!(
        "{:?}",
        cols.iter()
            .map(|c| c.get(c.len() - 1).unwrap_or(&' '))
            .collect::<String>()
    );
}
