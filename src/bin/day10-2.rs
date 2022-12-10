use std::{
    fmt::{self, Display, Formatter},
    io::{self, stdin, Read},
    str::FromStr,
};

struct CPU {
    pub x: i64,
    pub pc: i64,
    pub screen: [[char; 40]; 6],
}

impl CPU {
    pub fn default() -> Self {
        Self {
            x: 1,
            pc: 1,
            screen: [[' '; 40]; 6],
        }
    }

    pub fn sprite_vis(&self, c: i64) -> bool {
        if self.x == c || self.x - 1 == c || self.x + 1 == c {
            true
        } else {
            false
        }
    }

    pub fn inc_pc(&mut self) {
        let sc = (self.pc - 1) % 40;
        let r = (self.pc - 1) / 40;
        println!(
            "During cycle  {}: CRT draws pixel in position {}",
            self.pc, sc
        );
        if self.sprite_vis(sc) {
            self.screen[r as usize][sc as usize] = '#';
        } else {
            self.screen[r as usize][sc as usize] = '.';
        }
        // Print row as string
        self.pc += 1;
    }

    pub fn execute(&mut self, instr: &Instruction) {
        print!("Sprite position: ");
        for c in 0..40 {
            print!(
                "{}",
                match self.sprite_vis(c) {
                    true => '#',
                    false => '.',
                }
            );
        }
        println!();

        println!("Start cycle   {}: begin executing {}", self.pc, instr);
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
        for r in 0..6 {
            for c in 0..40 {
                print!("{}", self.screen[r][c]);
            }
            println!();
        }
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

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Instruction::Noop => write!(f, "noop"),
            Instruction::Addx(arg) => write!(f, "addx {}", arg),
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
