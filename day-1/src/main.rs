const INPUT: &str = include_str!("input.txt");

struct SlidingWindow {
    first: Option<i64>,
    second: Option<i64>,
    third: Option<i64>,
}

impl SlidingWindow {
    pub fn new() -> Self {
        Self {
            first: None,
            second: None,
            third: None,
        }
    }

    pub fn push(&mut self, value: i64) {
        self.first = self.second;
        self.second = self.third;
        self.third = Some(value);
    }

    pub fn get_sum(&self) -> Option<i64> {
        Some(self.first? + self.second? + self.third?)
    }
}

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let mut counter = 0;

    let mut previous_depth = None;
    for line in INPUT.split_terminator('\n') {
        let depth = line.parse::<i64>().expect("All lines must be valid i64");
        if depth > previous_depth.unwrap_or(depth) {
            counter += 1;
        }

        previous_depth = Some(depth);
    }

    println!("Number of increases: {}", counter);
}

fn part_2() {
    let mut counter = 0;
    let mut window = SlidingWindow::new();
    let mut previous_sum = None;

    for depth in INPUT
        .split_terminator('\n')
        .map(|line| line.parse::<i64>().expect("All lines must be valid i64"))
    {
        window.push(depth);
        let current_sum = window.get_sum();
        if let Some(sum) = current_sum {
            counter += if sum > previous_sum.unwrap_or(sum) {
                1
            } else {
                0
            };
        }

        previous_sum = current_sum
    }

    println!("Number of increases: {}", counter);
}
