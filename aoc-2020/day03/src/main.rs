use std::str::FromStr;

use utils::data::grid::{GridTrait, InfiniteRightGrid, input_to_grid};

#[derive(Clone, Debug, PartialEq)]
pub enum Cell {
    Empty,
    Tree,
}

#[derive(Clone, Debug)]
pub struct ParseError(String);

impl FromStr for Cell {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = match s {
            "#" => Cell::Tree,
            "." => Cell::Empty,
            _ => Err(ParseError(s.into()))?
        };

        Ok(res)
    }
}

pub struct Point {
    x: usize,
    y: usize,
}

pub fn inner_solution(g: &InfiniteRightGrid<Cell>, step: (usize, usize)) -> usize {
    let mut n_trees = 0;
    let mut pos = Point { x: 0, y: 0 };

    loop {
        let square = g.get(pos.y, pos.x);
        if let Some(cell) = square {
            if cell == Cell::Tree {
                n_trees += 1;
            }
        } else {
            return n_trees;
        }

        pos.x += step.0;
        pos.y += step.1;
    }
}

pub fn solution1(input: &str) -> usize {
    let g: InfiniteRightGrid<Cell> = input_to_grid(input);

    inner_solution(&g, (3, 1))
}

pub fn solution2(input: &str) -> usize {
    let g: InfiniteRightGrid<Cell> = input_to_grid(input);

    let steps = vec![
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    ];

    let mut product = 1;

    for step in steps {
        product *= inner_solution(&g, step);
    }

    product
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
    fn test_grid() {
        let input = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
        let _g: InfiniteRightGrid<usize> = input_to_grid(input);
    }

    const SAMPLE_INPUT: &str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    simple_test!(part1, solution1, SAMPLE_INPUT, 7);
    simple_test!(part2, solution2, SAMPLE_INPUT, 336);
}
