use std::{
    fmt::{self, Display, Formatter},
    io::{stdin, Read},
};

use indicatif::ProgressBar;
use itertools::Itertools;
use num_bigint::BigUint;
use num_traits::{ToPrimitive, Zero};
use regex::Regex;

enum Operation {
    Add,
    Mul,
    Sqr,
}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Add => write!(f, "+"),
            Self::Mul => write!(f, "*"),
            Self::Sqr => write!(f, "*old"),
        }
    }
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
    items: Vec<BigUint>,
    inspects: usize,
    operation: Operation,
    oparg: Option<BigUint>,
    testarg: BigUint,
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
            self.oparg.clone().unwrap_or(Zero::zero()),
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
        let test = input[3]
            .split(" ")
            .last()
            .unwrap()
            .parse::<BigUint>()
            .unwrap();
        let if_true = input[4].split(" ").last().unwrap().parse().unwrap();
        let if_false = input[5].split(" ").last().unwrap().parse().unwrap();
        Mokey {
            number,
            inspects: 0,
            items,
            operation: Operation::from(&operation, &oparg),
            oparg: match oparg.parse::<BigUint>() {
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
    let modulus = BigUint::from(
        mokeys
            .iter()
            .map(|m| m.testarg.clone())
            .reduce(|a, b| a * b)
            .unwrap(),
    );
    let pb = ProgressBar::new_spinner();
    for round in 0..10000 {
        pb.set_message(format!("Round {}", round));
        pb.inc(1);
        for mi in 0..mokeys.len() {
            let mokey = &mut mokeys[mi];
            mokey.inspect();
            let items = mokey
                .items
                .iter()
                .cloned()
                .map(|i| {
                    let new = match mokey.operation {
                        Operation::Add => i + mokey.oparg.clone().unwrap_or(Zero::zero()),
                        Operation::Mul => i * mokey.oparg.clone().unwrap_or(Zero::zero()),
                        Operation::Sqr => i.pow(2u32),
                    };

                    let idx = new.modpow(&BigUint::from(1u32), &mokey.testarg);

                    let new = new.modpow(&BigUint::from(1u32), &modulus);

                    if idx == Zero::zero() {
                        (mokey.if_true, new)
                    } else {
                        (mokey.if_false, new)
                    }
                })
                .collect::<Vec<_>>();

            mokey.items.clear();

            for (target, item) in items {
                mokeys[target].items.push(item);
            }
        }
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
