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
    pub knots: [(i32, i32); 10],
    pub maxx: i32,
    pub maxy: i32,
    pub visited: HashSet<(i32, i32)>,
}

impl State {
    fn new() -> Self {
        Self {
            knots: [(0, 0); 10],
            maxx: 0,
            maxy: 0,
            visited: HashSet::new(),
        }
    }
    fn moveh(&mut self, direction: Direction, steps: usize) {
        for _ in 0..steps {
            match direction {
                Direction::Up => {
                    self.knots[0].1 += 1;
                }
                Direction::Down => {
                    self.knots[0].1 -= 1;
                }
                Direction::Left => {
                    self.knots[0].0 -= 1;
                }
                Direction::Right => {
                    self.knots[0].0 += 1;
                }
            }
            if self.knots[0].0 > self.maxx {
                self.maxx = self.knots[0].0;
            }
            if self.knots[0].1 > self.maxy {
                self.maxy = self.knots[0].1;
            }
            for i in 1..self.knots.len() {
                // Check if we are already touching the tail
                // There are 8 positions where this is the case (4 corners and 4 edges)
                if self.knots[i - 1].0 - 1 == self.knots[i].0
                    && self.knots[i - 1].1 == self.knots[i].1
                    || self.knots[i - 1].0 + 1 == self.knots[i].0
                        && self.knots[i - 1].1 == self.knots[i].1
                    || self.knots[i - 1].0 == self.knots[i].0
                        && self.knots[i - 1].1 - 1 == self.knots[i].1
                    || self.knots[i - 1].0 == self.knots[i].0
                        && self.knots[i - 1].1 + 1 == self.knots[i].1
                    || self.knots[i - 1].0 - 1 == self.knots[i].0
                        && self.knots[i - 1].1 - 1 == self.knots[i].1
                    || self.knots[i - 1].0 - 1 == self.knots[i].0
                        && self.knots[i - 1].1 + 1 == self.knots[i].1
                    || self.knots[i - 1].0 + 1 == self.knots[i].0
                        && self.knots[i - 1].1 - 1 == self.knots[i].1
                    || self.knots[i - 1].0 + 1 == self.knots[i].0
                        && self.knots[i - 1].1 + 1 == self.knots[i].1
                    || self.knots[i - 1].0 == self.knots[i].0
                        && self.knots[i - 1].1 == self.knots[i].1
                {
                    // They are already touching, no need to move
                } else {
                    // Figure out how we need to move the tail to touch the head
                    // Check if we are not in the same row *or* column
                    if self.knots[i - 1].0 != self.knots[i].0
                        && self.knots[i - 1].1 != self.knots[i].1
                    {
                        // We are not in the same row nor the same column
                        // Move diagonally towards the head
                        if self.knots[i - 1].0 > self.knots[i].0 {
                            self.knots[i].0 += 1;
                        } else {
                            self.knots[i].0 -= 1;
                        }
                        if self.knots[i - 1].1 > self.knots[i].1 {
                            self.knots[i].1 += 1;
                        } else {
                            self.knots[i].1 -= 1;
                        }
                    } else {
                        // We are in the same row *or* column
                        // Check if we are in the same row
                        if self.knots[i - 1].0 == self.knots[i].0 {
                            // We are in the same row
                            // Check if we need to move up or down
                            if self.knots[i - 1].1 > self.knots[i].1 {
                                // We need to move down
                                self.knots[i].1 += 1;
                            } else {
                                // We need to move up
                                self.knots[i].1 -= 1;
                            }
                        } else {
                            // We are not in the same row
                            // Check if we need to move left or right
                            if self.knots[i - 1].0 > self.knots[i].0 {
                                // We need to move right
                                self.knots[i].0 += 1;
                            } else {
                                // We need to move left
                                self.knots[i].0 -= 1;
                            }
                        }
                    }
                }
                if self.knots[i].0 > self.maxx {
                    self.maxx = self.knots[i].0;
                }
                if self.knots[i].1 > self.maxy {
                    self.maxy = self.knots[i].1;
                }
            }
            for y in (0..self.maxy + 1).rev() {
                for x in 0..self.maxx + 1 {
                    for i in 0..self.knots.len() {
                        if self.knots[i].0 == x && self.knots[i].1 == y {
                            print!("{}", i);
                            break;
                        } else if i == self.knots.len() - 1 {
                            print!(".");
                        }
                    }
                }
                println!();
            }
            println!();
            self.visited.insert((self.knots[9].0, self.knots[9].1));
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
