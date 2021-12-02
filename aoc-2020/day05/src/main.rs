use std::ops::{
    RangeInclusive,
    Sub,
};

use utils::data::lines::input_to_data;

#[derive(Clone, Debug)]
pub enum Half {
    Front,
    Back,
}

pub trait RangeBinarySplit<T> {
    fn half(self, half: Half) -> Self;
    fn get_exact_one(self) -> Option<T>;
}

// impl<Idx: Sub<Output=Idx>> RangeBinarySplit for RangeInclusive<Idx> {
impl RangeBinarySplit<usize> for RangeInclusive<usize> {
    fn half(self, half: Half) -> Self {
        let count = self.clone().count();
        let start = *self.start();
        let end = *self.end();
        // println!("{:?}, {:?}, count: {}", self, half, count);
        if count == 2 {
            match half {
                Half::Front => {
                    start..=start
                },
                Half::Back => {
                    end..=end
                },
            }
        } else {
            match half {
                Half::Front => {
                    let end = (end + start) / 2;
                    start..=end
                },
                Half::Back => {
                    let start = (end + start) / 2 + 1;
                    start..=end
                },
            }
        }
    }

    fn get_exact_one(self) -> Option<usize> {
        // println!("one: {:?}", self);
        if self.start() == self.end() {
            return Some(*self.start());
        }

        None
    }
}

struct Splitter;

impl Splitter {
    fn find(&self, seat: &str) -> (usize, usize) {
        let mut rows: RangeInclusive<usize> = 0..=127;
        let mut cols: RangeInclusive<usize> = 0..=7;
        let (rr, cc) = seat.split_at(7);
        for r in rr.chars() {
            let r_str = &r.to_string();
            if r_str == "F" {
                rows = rows.half(Half::Front);
            } else if r_str == "B" {
                rows = rows.half(Half::Back);
            }
        }

        for c in cc.chars() {
            let c_str = &c.to_string();
            if c_str == "L" {
                cols = cols.half(Half::Front);
            } else if c_str == "R" {
                cols = cols.half(Half::Back);
            }
        }

        (
            rows.get_exact_one().unwrap(),
            cols.get_exact_one().unwrap()
        )
    }
}

pub fn solution1(input: &str) -> usize {
    let data: Vec<String> = input_to_data(input);
    let mut max = 0;

    let splitter = Splitter;
    for entry in data {
        if entry.is_empty() {
            continue;
        }

        let (row, col) = splitter.find(&entry);
        max = max.max(row * 8 + col);
    }

    max
}


pub fn solution2(input: &str) -> usize {
    let data: Vec<String> = input_to_data(input);
    let mut all_seats = Vec::with_capacity(data.len());
    let mut grid = vec![vec![(0, 0); 8]; 128];

    let splitter = Splitter;
    for entry in data {
        if entry.is_empty() {
            continue;
        }

        let (row, col) = splitter.find(&entry);
        let id = row * 8 + col;
        // max = max.max(id);
        all_seats.push(id);
        grid[row][col] = (id, 1);
    }

    // println!("{:#?}", all_seats);
    for row in grid {
        for col in row {
            print!("{:>4?}", col.0);
        }
        println!();
    }

    all_seats.sort_unstable();
    for w in all_seats.windows(2) {
        if w[0] + 2 == w[1] {
            return w[0] + 1
        }
    }

    unreachable!()
}

fn main() {
    let contents = std::fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    println!("part1: {}", solution1(&contents));
    println!("part2: {}", solution2(&contents));
}

#[cfg(test)]
mod tests {
    use utils::simple_test;

    use super::*;

    #[test]
    fn woot() {
        let range = 0..=127;
        println!("range: {:?}", range.half(Half::Front).half(Half::Back));
    }

    #[test]
    fn woot2() {
        let range = 0..=3;
        let val = range.half(Half::Front).half(Half::Back).get_exact_one().unwrap();
        assert_eq!(val, 1);
    }

    const SAMPLE_INPUT: &str = "FBFBBFFRLR";

    simple_test!(part1, solution1, SAMPLE_INPUT, 357);
}
