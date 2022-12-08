use std::{
    collections::HashSet,
    io::{stdin, Read},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let forest = input
        .lines()
        .map(|l| {
            l.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut visible = HashSet::new();
    // Check the left and right sides
    for i in 0..forest.len() {
        let mut tallest = -1;
        // Left to right along each row
        for j in 0..forest.get(0).unwrap().len() {
            if forest.get(i).unwrap().get(j).unwrap() > &tallest {
                tallest = *forest.get(i).unwrap().get(j).unwrap();
                visible.insert((i, j));
            }
        }
        // Right to left along each row
        tallest = -1;
        for j in (0..forest.get(0).unwrap().len()).rev() {
            if forest.get(i).unwrap().get(j).unwrap() > &tallest {
                tallest = *forest.get(i).unwrap().get(j).unwrap();
                visible.insert((i, j));
            }
        }
    }
    // Check the top and bottom sides
    for i in 0..forest.get(0).unwrap().len() {
        let mut tallest = -1;
        // Top to bottom along each column
        for j in 0..forest.len() {
            if forest.get(j).unwrap().get(i).unwrap() > &tallest {
                tallest = *forest.get(j).unwrap().get(i).unwrap();
                visible.insert((j, i));
            }
        }
        // Bottom to top along each column
        tallest = -1;
        for j in (0..forest.len()).rev() {
            if forest.get(j).unwrap().get(i).unwrap() > &tallest {
                tallest = *forest.get(j).unwrap().get(i).unwrap();
                visible.insert((j, i));
            }
        }
    }

    for i in 0..forest.len() {
        for j in 0..forest.get(0).unwrap().len() {
            if visible.contains(&(i, j)) {
                print!("\x1b[32m{}\x1b[0m ", forest.get(i).unwrap().get(j).unwrap());
            } else {
                print!("\x1b[31m{}\x1b[0m ", forest.get(i).unwrap().get(j).unwrap());
            }
        }
        println!();
    }

    println!("{}", visible.len());
}
