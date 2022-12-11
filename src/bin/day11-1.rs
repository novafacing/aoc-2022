use std::{
    fmt::{self, Display, Formatter},
    io::{stdin, Read},
};

use itertools::Itertools;
use regex::Regex;

enum Operation {
    Add,
    Mul,
    Sqr,
}

impl Operation {
    fn from(ops: &str, args: &str) -> Self {
        match (ops, args) {
            ("*", "old") => Self::Sqr,
            ("*", _) => Self::Mul,
            ("+", _) => Self::Add,
            (_, _) => panic!("Unknown operation"),
        }
    }
}

struct Mokey {
    number: usize,
    items: Vec<i64>,
    inspects: usize,
    operation: Operation,
    oparg: Option<i64>,
    testarg: i64,
    if_true: usize,
    if_false: usize,
}

impl Display for Mokey {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Monkey {}: inspects: {} [{}] op: {} T-> {}, F-> {}",
            self.number,
            self.inspects,
            self.items
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<_>>()
                .join(", "),
            self.oparg.unwrap_or(0),
            self.if_true,
            self.if_false
        )
    }
}

impl Mokey {
    fn from(input: Vec<&str>) -> Self {
        let number = Regex::new(r"Monkey (\d+):")
            .unwrap()
            .captures(input[0])
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse()
            .unwrap();
        let items = input[1]
            .split(": ")
            .nth(1)
            .unwrap()
            .split(", ")
            .map(|s| s.parse().unwrap())
            .collect();
        let (operation, oparg) = Regex::new(r"Operation: new = old (.) (.*)")
            .unwrap()
            .captures(input[2])
            .unwrap()
            .iter()
            .skip(1)
            .filter_map(|m| m)
            .map(|m| m.as_str().to_string())
            .collect_tuple()
            .unwrap();
        let test = input[3].split(" ").last().unwrap().parse::<i64>().unwrap();
        let if_true = input[4].split(" ").last().unwrap().parse().unwrap();
        let if_false = input[5].split(" ").last().unwrap().parse().unwrap();
        Mokey {
            number,
            inspects: 0,
            items,
            operation: Operation::from(&operation, &oparg),
            oparg: match oparg.parse::<i64>() {
                Ok(oparg) => Some(oparg),
                Err(_) => None,
            },
            testarg: test,
            if_true,
            if_false,
        }
    }
    fn inspect(&mut self) {
        self.inspects += self.items.len();
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut monkeyctr = 0;
    let mut mokeys = input
        .lines()
        .group_by(|l| match l.starts_with("Monkey") {
            true => {
                monkeyctr += 1;
                monkeyctr
            }
            false => monkeyctr,
        })
        .into_iter()
        .map(|(_, group)| Mokey::from(group.collect::<Vec<_>>()))
        .collect::<Vec<_>>();
    for _ in 0..20 {
        for mi in 0..mokeys.len() {
            let mokey = &mut mokeys[mi];
            mokey.inspect();
            let items = mokey
                .items
                .iter()
                .cloned()
                .map(|i| {
                    let mut new = match mokey.operation {
                        Operation::Add => i + mokey.oparg.unwrap_or(0),
                        Operation::Mul => i * mokey.oparg.unwrap_or(0),
                        Operation::Sqr => i * i,
                    };
                    new /= 3;
                    match new % mokey.testarg {
                        0 => (mokey.if_true, new),
                        _ => (mokey.if_false, new),
                    }
                })
                .collect::<Vec<_>>();

            mokey.items.clear();

            for (target, item) in items {
                mokeys[target].items.push(item);
            }
        }
    }
    for mokey in &mokeys {
        println!("{}", mokey);
    }
    let monkey_business = mokeys
        .iter()
        .map(|m| m.inspects)
        .sorted()
        .rev()
        .take(2)
        .product::<usize>();
    println!("{}", monkey_business);
}
