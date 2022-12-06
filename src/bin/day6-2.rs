use std::{
    collections::HashSet,
    io::{stdin, Read},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut buffer = Vec::new();
    input.chars().enumerate().for_each(|(i, c)| {
        if buffer.len() >= 14 {
            buffer.remove(0);
        }
        buffer.push(c);
        if buffer.iter().collect::<HashSet<_>>().len() == 14 {
            println!("{:?}", i + 1);
            return;
        }
    });
}
