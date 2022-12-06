use std::io::{stdin, Read};

use itertools::Itertools;

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let code = input
        .chars()
        .tuple_windows()
        .filter(|(a, b, c, d)| {
            // Check if a, b, c, d all differ
            a != b && a != c && a != d && b != c && b != d && c != d
        })
        .next()
        .map(|(a, b, c, d)| {
            // If they do, return the first three
            let code = format!("{}{}{}{}", a, b, c, d);
            println!("{}", code);
            code
        })
        .unwrap();

    let idx = input.find(&code).unwrap() + 4;
    println!("{:?}", idx);
}
