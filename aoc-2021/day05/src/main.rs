use std::str::FromStr;

use utils::data::{
    grid::GridTrait,
    grid_traits::{ArbitraryAccess, ImmutableGrid, MutableGrid},
    lines::input_to_data,
};

pub struct GrowableGrid<T> {
    pub real_height: usize,
    pub real_width: usize,
    pub inner: Vec<Vec<T>>,
}

impl<T> ImmutableGrid<T> for GrowableGrid<T> {
    fn get(&self, row: usize, col: usize) -> Option<&T> {
        self.inner.get(row).and_then(|row| row.get(col))
    }
}

impl<T: Clone> MutableGrid<T> for GrowableGrid<T> {
    fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        self.inner.get_mut(row).and_then(|row| row.get_mut(col))
    }

    fn push_line(&mut self, line: &[T]) {
        self.inner.push(line.to_vec())
    }

    fn insert_line(&mut self, idx: usize, line: &[T]) {
        todo!()
    }
}

impl<T: Clone + Default> ArbitraryAccess<T> for GrowableGrid<T> {
    fn extend_rows_with(&mut self, row: usize, new_len: usize, val: T) {
        let upto = new_len - self.inner[0].len();
        for row in &mut self.inner {
            row.extend(vec![T::default(); upto].iter().cloned());
        }
    }
}

impl GrowableGrid<usize> {
    pub fn new() -> Self {
        Self {
            // this 1000s is a hack to avoid inner resize
            // basically, it's not a 'growable' grid anymore
            inner: vec![vec![0; 1000]; 1000],
            real_height: 0,
            real_width: 0,
        }
    }

    // I don't like the implementation. There are some ideas how to improve this
    pub fn draw_line(&mut self, line: &Line, consider_other: bool) {
        let vert_from = line.start.1.min(line.end.1);
        let vert_to = line.start.1.max(line.end.1);
        let horiz_from = line.start.0.min(line.end.0);
        let horiz_to = line.start.0.max(line.end.0);
        self.real_width = self.real_width.max(horiz_to);
        self.real_height = self.real_height.max(vert_to);

        if line.is_horiz() {
            let row = line.start.1;

            for i in horiz_from..=horiz_to {
                self.inner[row][i] += 1;
            }
        } else if line.is_vert() {
            let col = line.start.0;

            for i in vert_from..=vert_to {
                let v = self.get_mut(i, col).unwrap();
                *v += 1;
            }
        } else if consider_other {
            let mut xr = if line.start.0 < line.end.0 {
                (line.start.0..=line.end.0).collect::<Vec<_>>()
            } else {
                (line.end.0..=line.start.0).rev().collect()
            };

            let mut yr = if line.start.1 < line.end.1 {
                (line.start.1..=line.end.1).collect::<Vec<_>>()
            } else {
                (line.end.1..=line.start.1).rev().collect()
            };

            for (i, j) in yr.iter().zip(xr.iter()) {
                self.inner[*i][*j] += 1;
            }
        }
    }

    pub fn print(&self, pad: usize) {
        for i in 0..=self.real_height {
            for j in 0..=self.real_width {
                let val = *self.get(i, j).unwrap();
                if val == 0 {
                    print!("{:>pad$}", "  .", pad = pad);
                } else {
                    print!("{:>pad$}", val, pad = pad);
                }
            }
            println!();
        }
    }
}

impl Default for GrowableGrid<usize> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub struct Point(pub usize, pub usize);

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sp: Vec<_> = s.split(",").map(|x| x.parse::<usize>().unwrap()).collect();
        Ok(Point(sp[0], sp[1]))
    }
}

#[derive(Clone, Debug)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    pub fn is_horiz(&self) -> bool {
        self.start.1 == self.end.1
    }

    pub fn is_vert(&self) -> bool {
        self.start.0 == self.end.0
    }
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(());
        }
        let sp: Vec<_> = s.split(" -> ").collect();
        let _from = sp[0].parse::<Point>().unwrap();
        let _to = sp[1].parse::<Point>().unwrap();

        Ok(Line {
            start: _from,
            end: _to,
        })
    }
}

fn solution1(input: &str) -> usize {
    let data = input_to_data::<Line>(input);
    // println!("data: {:#?}", data);
    let mut grid = GrowableGrid::new();
    for line in data {
        grid.draw_line(&line, false);
    }

    // grid.print(3);

    let mut count = 0;
    for row in grid.inner.iter() {
        for col in row {
            if *col > 1 {
                count += 1;
            }
        }
    }

    count
}

fn solution2(input: &str) -> usize {
    let data = input_to_data::<Line>(input);
    // println!("data: {:#?}", data);
    let mut grid = GrowableGrid::new();
    for line in data {
        grid.draw_line(&line, true);
    }

    // grid.print(3);

    let mut count = 0;
    for row in grid.inner.iter() {
        for col in row {
            if *col > 1 {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    let contents =
        std::fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    println!("part1: {}", solution1(&contents));
    println!("part2: {}", solution2(&contents));
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::simple_test;

    const SAMPLE_INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    simple_test!(part1, solution1, SAMPLE_INPUT, 5);
    simple_test!(part2, solution2, SAMPLE_INPUT, 12);
}
