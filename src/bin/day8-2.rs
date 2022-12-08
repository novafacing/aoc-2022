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

    // Make an all-zero copy of the forest
    let mut scores = vec![vec![0; forest.get(0).unwrap().len()]; forest.len()];

    for y in 0..forest.len() {
        for x in 0..forest.get(0).unwrap().len() {
            // Go each direction from the position until either
            // an edge is reached or a taller tree is found
            let mut left = 0;
            for i in (0..x).rev() {
                left += 1;
                if forest.get(y).unwrap().get(i).unwrap() >= forest.get(y).unwrap().get(x).unwrap()
                {
                    break;
                }
            }
            let mut right = 0;
            for i in (x + 1)..forest.get(0).unwrap().len() {
                right += 1;
                if forest.get(y).unwrap().get(i).unwrap() >= forest.get(y).unwrap().get(x).unwrap()
                {
                    break;
                }
            }
            let mut up = 0;
            for i in (0..y).rev() {
                up += 1;
                if forest.get(i).unwrap().get(x).unwrap() >= forest.get(y).unwrap().get(x).unwrap()
                {
                    break;
                }
            }
            let mut down = 0;
            for i in (y + 1)..forest.len() {
                down += 1;
                if forest.get(i).unwrap().get(x).unwrap() >= forest.get(y).unwrap().get(x).unwrap()
                {
                    break;
                }
            }
            let score = left * right * up * down;
            scores
                .get_mut(y)
                .unwrap()
                .get_mut(x)
                .unwrap()
                .clone_from(&score);
        }
    }

    let max = scores.iter().flat_map(|r| r.iter()).max().unwrap();
    println!("{}", max);
}
