use std::collections::HashSet;

use utils::data::grid_traits::{ImmutableGrid, MutableGrid};

pub struct Grid {
    inner: Vec<Vec<usize>>,
    width: usize,
    height: usize,
}

impl ImmutableGrid<usize> for Grid {
    fn get(&self, row: usize, col: usize) -> Option<&usize> {
        self.inner.get(row).and_then(|row| row.get(col))
    }
}

impl Grid {
    pub fn from_lines(input: &str) -> Self {
        let mut v: Vec<Vec<usize>> = Vec::with_capacity(input.lines().count());

        for line in input.lines() {
            if line.is_empty() {
                continue;
            }

            let x = line.chars().map(|x| x.to_string().parse::<usize>().unwrap()).collect::<Vec<_>>();
            v.push(x);
        }

        let width = v[0].len();
        let height = v.len();

        Self {
            inner: v,
            width,
            height,
        }
    }

    pub fn get_i64(&self, row: i64, col: i64) -> Option<&usize> {
        if row < 0 || col < 0 {
            return None;
        }

        if row > self.height as i64 {
            return None;
        }

        if col > self.width  as i64 {
            return None;
        }

        self.get(row as usize, col as usize)
    }

    pub fn is_minimum(&self, row: i64, col: i64) -> Option<usize> {
        let top = self.get_i64(row - 1, col).unwrap_or(&9);
        let bottom = self.get_i64(row + 1, col).unwrap_or(&9);
        let left = self.get_i64(row, col - 1).unwrap_or(&9);
        let right = self.get_i64(row, col + 1).unwrap_or(&9);

        let x = self.get(row as usize, col as usize).unwrap();

        if x < top && x < bottom && x < left && x < right {
            Some(*x)
        } else {
            None
        }
    }

    pub fn _add_point_maybe(&self, row: i64, col: i64, res: &mut Vec<(usize, usize)>) {
        let point = self.get_i64(row, col);
        if let Some(&point) = point {
            if point < 9 {
                res.push((row as usize, col as usize));
            }
        }
    }

    pub fn close_points(&self, row: i64, col: i64) -> Vec<(usize, usize)> {
        // let top = self.get_i64(row - 1, col);
        // let bottom = self.get_i64(row + 1, col);
        // let left = self.get_i64(row, col - 1);
        // let right = self.get_i64(row, col + 1);
        let mut res = vec![];
        // if let Some(&top) = top {
        //     if top < 9 {
        //         res.push((row as usize, col as usize));
        //     }
        // }

        self._add_point_maybe(row - 1, col, &mut res);
        self._add_point_maybe(row + 1, col, &mut res);
        self._add_point_maybe(row, col - 1, &mut res);
        self._add_point_maybe(row, col + 1, &mut res);

        res
    }

    pub fn _explore_next(&self, seen: &mut HashSet<(usize, usize)>, row: i64, col: i64) {
        for point in self.close_points(row, col) {
            if !seen.contains(&point) {
                seen.insert((point.0, point.1));
                self._explore_next(seen, point.0 as i64, point.1 as i64)
            }
        }
    }

    pub fn explore_basin(&self, row: usize, col: usize) -> usize {
        let mut seen: HashSet<(usize, usize)> = HashSet::new();
        self._explore_next(&mut seen, row as i64, col as i64);

        seen.len()
    }

}

fn solution1(input: &str) -> usize {
    let g = Grid::from_lines(input);
    let mut sum = 0;

    for row in 0..g.height {
        for col in 0..g.width {
            let x = g.is_minimum(row as i64, col as i64);
            if let Some(z) = x {
                sum += z + 1;
            }
        }
    }

    sum
}



fn solution2(input: &str) -> usize {
    let g = Grid::from_lines(input);
    let mut vals = vec![];

    for row in 0..g.height {
        for col in 0..g.width {
            let x = g.is_minimum(row as i64, col as i64);
            if let Some(z) = x {
                let size = g.explore_basin(row, col);
                // println!("basin size: {}", size);
                vals.push(size);
            }
        }
    }

    vals.sort_unstable();
    vals.iter().rev().take(3).product()
}

fn main() {
    let contents = std::fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    println!("part1: {}", solution1(&contents));
    println!("part2: {}", solution2(&contents));
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::simple_test;

    const SAMPLE_INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    simple_test!(part1, solution1, SAMPLE_INPUT, 15);
    simple_test!(part2, solution2, SAMPLE_INPUT, 1134);
}
