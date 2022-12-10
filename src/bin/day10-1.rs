use std::{
    io::{stdin, Read},
    str::FromStr,
};

struct CPU {
    pub x: i64,
    pub pc: i64,
    pub intr: i64,
    pub tpoints: Vec<i64>,
}

impl CPU {
    pub fn default() -> Self {
        Self {
            x: 1,
            pc: 0,
            intr: 20,
            tpoints: Vec::new(),
        }
    }

    pub fn inc_pc(&mut self) {
        self.pc += 1;
        if self.pc == self.intr {
            self.tpoints.push(self.pc * self.x);
            self.intr += 40;
        }
    }

    pub fn execute(&mut self, instr: &Instruction) {
        match instr {
            Instruction::Noop => self.inc_pc(),
            Instruction::Addx(arg) => {
                self.inc_pc();
                self.inc_pc();
                self.x += arg;
            }
        }
    }

    pub fn run(&mut self, program: &Vec<Instruction>) {
        program.iter().for_each(|i| {
            self.execute(i);
        });
        println!("{}", self.tpoints.iter().sum::<i64>());
    }
}

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i64),
}

impl FromStr for Instruction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let op = parts.next().unwrap().trim();
        let arg = parts.next().unwrap_or("0").trim().parse::<i64>().unwrap();
        match op {
            "noop" => Ok(Instruction::Noop),
            "addx" => Ok(Instruction::Addx(arg)),
            _ => Err(format!("Unknown op: {}", op)),
        }
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut cpu = CPU::default();
    let program = input
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap())
        .collect::<Vec<_>>();
    cpu.run(&program);
}
