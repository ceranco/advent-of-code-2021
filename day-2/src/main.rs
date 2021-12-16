use std::{fmt::Display, str::FromStr};

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Clone, Copy)]
enum Movement {
    Forward(i32),
    Up(i32),
    Down(i32),
}

#[derive(Debug)]
struct ParseMovementError(String);

impl FromStr for Movement {
    type Err = ParseMovementError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const FORWARD_CMD: &str = "forward";
        const UP_CMD: &str = "up";
        const DOWN_CMD: &str = "down";

        let parts = s.split_whitespace().collect::<Vec<_>>();
        if parts.len() != 2 {
            return Err(ParseMovementError(format!("Too many parts: {}", s)));
        }

        match parts[0] {
            FORWARD_CMD => parts[1]
                .parse::<i32>()
                .map(Movement::Forward)
                .map_err(|_| ParseMovementError(format!("Failed to convert int: {}", parts[1]))),
            UP_CMD => parts[1]
                .parse::<i32>()
                .map(Movement::Up)
                .map_err(|_| ParseMovementError(format!("Failed to convert int: {}", parts[1]))),
            DOWN_CMD => parts[1]
                .parse::<i32>()
                .map(Movement::Down)
                .map_err(|_| ParseMovementError(format!("Failed to convert int: {}", parts[1]))),
            _ => Err(ParseMovementError(format!("Urecognized command: {}", s))),
        }
    }
}

struct Submarine {
    horizontal_pos: i32,
    depth: i32,
    aim: i32,
}

impl Submarine {
    fn new() -> Self {
        Self {
            horizontal_pos: 0,
            depth: 0,
            aim: 0,
        }
    }

    fn move_by_v1(&mut self, movement: Movement) {
        match movement {
            Movement::Forward(distance) => self.horizontal_pos += distance,
            Movement::Up(distance) => self.depth -= distance,
            Movement::Down(distance) => self.depth += distance,
        }
    }

    fn move_by_v2(&mut self, movement: Movement) {
        match movement {
            Movement::Forward(distance) => {
                self.horizontal_pos += distance;
                self.depth += self.aim * distance;
            }
            Movement::Up(change) => self.aim -= change,
            Movement::Down(change) => self.aim += change,
        }
    }
}

impl Display for Submarine {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Submarine {{ horizontal_pos: {}, depth: {}, aim: {}, multiplied: {} }}",
            self.horizontal_pos,
            self.depth,
            self.aim,
            self.horizontal_pos * self.depth
        )
    }
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut sub = Submarine::new();

    for movement in INPUT.lines().map(|line| line.parse::<Movement>().unwrap()) {
        sub.move_by_v1(movement);
    }

    println!("{}", sub);
}

fn part2() {
    let mut sub = Submarine::new();

    for movement in INPUT.lines().map(|line| line.parse::<Movement>().unwrap()) {
        sub.move_by_v2(movement);
    }

    println!("{}", sub);
}
