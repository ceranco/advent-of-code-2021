const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Clone, Copy, PartialEq)]
enum Bit {
    Zero,
    One,
}

impl Bit {
    pub fn not(&self) -> Self {
        match self {
            Bit::Zero => Bit::One,
            Bit::One => Bit::Zero,
        }
    }
}

impl From<i32> for Bit {
    fn from(num: i32) -> Self {
        if num == 0 {
            Bit::Zero
        } else {
            Bit::One
        }
    }
}

impl From<Bit> for i32 {
    fn from(bit: Bit) -> Self {
        match bit {
            Bit::Zero => 0,
            Bit::One => 1,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct BitCounter {
    num_ones: i32,
    num_zeroes: i32,
}

impl BitCounter {
    pub fn new() -> Self {
        Self {
            num_ones: 0,
            num_zeroes: 0,
        }
    }

    pub fn add(&mut self, bit: Bit) {
        match bit {
            Bit::Zero => self.num_zeroes += 1,
            Bit::One => self.num_ones += 1,
        }
    }

    pub fn get_most(&self) -> Bit {
        if self.num_zeroes > self.num_ones {
            Bit::Zero
        } else {
            Bit::One
        }
    }
}

trait ToDecimal {
    fn to_decimal(&self) -> i32;
}

impl ToDecimal for Vec<Bit> {
    fn to_decimal(&self) -> i32 {
        let mut sum = 0;
        let mut factor = 1 << (self.len() - 1);

        for bit in self {
            sum += i32::from(*bit) * factor;
            factor >>= 1;
        }

        sum
    }
}

fn main() {
    let numbers = INPUT
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).expect("Expected only numbers") as i32)
                .map(Bit::from)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    part1(&numbers);
    part2(numbers);
}

fn part1(numbers: &Vec<Vec<Bit>>) {
    let counters = get_counters(numbers);

    let gamma_rate = counters
        .iter()
        .map(|counter| counter.get_most())
        .collect::<Vec<_>>()
        .to_decimal();

    let espilon_rate = counters
        .iter()
        .map(|counter| counter.get_most().not())
        .collect::<Vec<_>>()
        .to_decimal();

    println!(
        "The power consumption of the submarine is: {}",
        gamma_rate * espilon_rate
    )
}

fn get_counters(numbers: &Vec<Vec<Bit>>) -> Vec<BitCounter> {
    let mut counters = Vec::new();
    for bits in numbers {
        for (bit_counter, bit) in bits.iter().enumerate() {
            if counters.len() == bit_counter {
                counters.push(BitCounter::new());
            }

            counters[bit_counter].add(*bit);
        }
    }

    counters
}

fn part2(numbers: Vec<Vec<Bit>>) {
    let oxygen_generator_rating = filter_by_bits(numbers.clone(), Bit::One).to_decimal();
    let co2_scrubber_rating = filter_by_bits(numbers, Bit::Zero).to_decimal();

    println!(
        "The life support rating of the submarine is: {}",
        oxygen_generator_rating * co2_scrubber_rating
    )
}

fn filter_by_bits(mut numbers: Vec<Vec<Bit>>, preferred: Bit) -> Vec<Bit> {
    let mut bit_number = 0;
    loop {
        let bits = get_counters(&numbers)
            .iter()
            .map(|counter| counter.get_most())
            .map(|bit| {
                if preferred == Bit::One {
                    bit
                } else {
                    bit.not()
                }
            })
            .collect::<Vec<_>>();

        numbers = numbers
            .into_iter()
            .filter(|number| number[bit_number] == bits[bit_number])
            .collect();

        if numbers.len() == 1 {
            break;
        }

        bit_number += 1;
    }

    numbers.swap_remove(0)
}
