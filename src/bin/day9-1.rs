use std::{
    collections::HashSet,
    io::{stdin, Read},
};

enum Direction {
    Up,
    Down,
    Left,
    Right,
}
struct State {
    pub hx: i32,
    pub hy: i32,
    pub tx: i32,
    pub ty: i32,
    pub maxx: i32,
    pub maxy: i32,
    pub visited: HashSet<(i32, i32)>,
}

impl State {
    fn new() -> Self {
        Self {
            hx: 0,
            hy: 0,
            tx: 0,
            ty: 0,
            maxx: 0,
            maxy: 0,
            visited: HashSet::new(),
        }
    }
    fn moveh(&mut self, direction: Direction, steps: usize) {
        for _ in 0..steps {
            match direction {
                Direction::Up => {
                    self.hy += 1;
                }
                Direction::Down => {
                    self.hy -= 1;
                }
                Direction::Left => {
                    self.hx -= 1;
                }
                Direction::Right => {
                    self.hx += 1;
                }
            }
            // Check if we are already touching the tail
            // There are 8 positions where this is the case (4 corners and 4 edges)
            if self.hx - 1 == self.tx && self.hy == self.ty
                || self.hx + 1 == self.tx && self.hy == self.ty
                || self.hx == self.tx && self.hy - 1 == self.ty
                || self.hx == self.tx && self.hy + 1 == self.ty
                || self.hx - 1 == self.tx && self.hy - 1 == self.ty
                || self.hx - 1 == self.tx && self.hy + 1 == self.ty
                || self.hx + 1 == self.tx && self.hy - 1 == self.ty
                || self.hx + 1 == self.tx && self.hy + 1 == self.ty
                || self.hx == self.tx && self.hy == self.ty
            {
                // They are already touching, no need to move
            } else {
                // Figure out how we need to move the tail to touch the head
                // Check if we are not in the same row *or* column
                if self.hx != self.tx && self.hy != self.ty {
                    // We are not in the same row nor the same column
                    // Move diagonally towards the head
                    if self.hx > self.tx {
                        self.tx += 1;
                    } else {
                        self.tx -= 1;
                    }
                    if self.hy > self.ty {
                        self.ty += 1;
                    } else {
                        self.ty -= 1;
                    }
                } else {
                    // We are in the same row *or* column
                    // Check if we are in the same row
                    if self.hx == self.tx {
                        // We are in the same row
                        // Check if we need to move up or down
                        if self.hy > self.ty {
                            // We need to move down
                            self.ty += 1;
                        } else {
                            // We need to move up
                            self.ty -= 1;
                        }
                    } else {
                        // We are not in the same row
                        // Check if we need to move left or right
                        if self.hx > self.tx {
                            // We need to move right
                            self.tx += 1;
                        } else {
                            // We need to move left
                            self.tx -= 1;
                        }
                    }
                }
            }
            if self.hy > self.maxy {
                self.maxy = self.hy;
            }
            if self.ty > self.maxy {
                self.maxy = self.ty;
            }
            if self.hx > self.maxx {
                self.maxx = self.hx;
            }
            if self.tx > self.maxx {
                self.maxx = self.tx;
            }
            for y in (0..self.maxy + 1).rev() {
                for x in 0..self.maxx + 1 {
                    if x == self.hx && y == self.hy {
                        print!("H");
                    } else if x == self.tx && y == self.ty {
                        print!("T");
                    } else {
                        print!(".");
                    }
                }
                println!();
            }
            println!();
            self.visited.insert((self.tx, self.ty));
        }
    }
}

fn main() {
    let mut input = String::new();
    let mut state = State::new();
    stdin().read_to_string(&mut input).unwrap();
    input.lines().for_each(|l| {
        let (direction, steps) = l.trim().split_at(1);
        println!("'{}' '{}'", direction, steps);
        let direction = match direction.trim() {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Invalid direction"),
        };
        let steps = steps.trim().parse::<usize>().unwrap();
        state.moveh(direction, steps);
    });

    for y in (0..state.maxy + 1).rev() {
        for x in 0..state.maxx + 1 {
            if state.visited.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!("{}", state.visited.len());
}
