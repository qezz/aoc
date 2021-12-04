use std::{collections::HashSet, fmt::Display};

use utils::data::{grid::GridTrait, multiline::{MultilineParser, group_rows}};

fn sum_all(v: &[Vec<usize>]) -> usize {
    let mut sum = 0;
    for row in v {
        sum += row.iter().sum::<usize>();
    }

    sum
}

fn print_grid<T: Display>(grid: &[Vec<T>], pad: usize) {
    for row in grid {
        for col in row {
            print!("{:>pad$}", col, pad=pad);
        }

        println!();
    }
}

fn transpose<T: Clone>(v: &[Vec<T>]) -> Vec<Vec<T>> {
    let n_rows = v.len();
    let n_cols = v[0].len();
    let _item = v[0][0].clone();
    let mut res = vec![vec![_item; n_rows]; n_cols]; // Vec::with_capacity(n_cols);

    for i in 0..n_rows {
        for j in 0..n_cols {
            res[j][i] = v[i][j].clone()
        }
    }

    res
}

#[derive(Clone, Debug)]
pub struct BingoBoard {
    _side: usize,
    inner: Vec<Vec<usize>>,
    view: Vec<Vec<bool>>,
}

#[derive(Clone, Debug)]
pub enum BingoResult {
    Col(usize),
    Row(usize),
}

impl BingoBoard {
    // -> (row, col)
    fn find(&self, val: usize) -> Option<(usize, usize)> {
        for (i, row) in self.inner.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                if *col == val {
                    return Some((i, j))
                }
            }
        }

        None
    }

    fn apply(&mut self, val: usize) -> bool {
        let p = self.find(val);
        if let Some(pos) = p {
            self.view[pos.0][pos.1] = true;
            return true;
        }

        false
    }

    fn bingo(&self) -> Option<BingoResult> {
        for (i, row) in self.view.iter().enumerate() {
            if row.iter().all(|x| *x) {
                return Some(BingoResult::Row(i))
            }
        }

        let t_view = transpose(&self.view);
        for (i, row) in t_view.iter().enumerate() {
            if row.iter().all(|x| *x) {
                return Some(BingoResult::Col(i))
            }
        }

        None
    }

    fn print(&self) {
        let grid = &self.inner;
        let view = &self.view;
        for (i, row) in grid.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                if view[i][j] {
                    print!("{:>pad$}", col, pad=4);
                } else {
                    print!("{:>pad$}", ".", pad=4);
                }
            }

            println!();
        }
    }
}

impl GridTrait for BingoBoard {
    type Item = usize;

    fn fixed(width: usize, height: usize) -> Self {
        Self {
            _side: width,
            inner: Vec::with_capacity(height),
            view: vec![vec![false; width]; width],
        }
    }

    fn insert_row(&mut self, row: &Vec<Self::Item>) {
        self.inner.push(row.to_vec());
    }

    fn get(&self, row: usize, col: usize) -> Option<Self::Item> {
        todo!()
    }
}

fn read_data(input: &str) -> (Vec<usize>, Vec<BingoBoard>) {
    let data = group_rows(input);
    let first_line: String = data[0][0].clone();
    let numbers: Vec<usize> = first_line
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let mut bingos: Vec<BingoBoard> = Vec::with_capacity(data.len());

    for bingo_lines in data.iter().skip(1) {
        let mut bingo = BingoBoard::fixed(5, 5);
        for line in bingo_lines {
            let v: Vec<usize> =
                line.split_whitespace() // .split(' ')
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            bingo.insert_row(&v);
        }
        bingos.push(bingo);
    }

    (numbers, bingos)
}

fn solution1(input: &str) -> usize {
    let (numbers, mut bingos) = read_data(input);

    let mut res = None;

    'outer: for number in numbers {
        for bingo in bingos.iter_mut() {
            if bingo.apply(number) {
                if bingo.bingo().is_some() {
                    res = Some((bingo.clone(), number));
                    break 'outer;
                }
            }
        }
    }

    let res = res.unwrap();
    let res_bingo = res.0;

    let mut unmarked_sum = 0;
    for (i, row) in res_bingo.inner.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if !res_bingo.view[i][j] {
                unmarked_sum += col;
            }
        }
    }

    res_bingo.print();

    let last_num = res.1;

    println!("unmarked_sum: {:?}", unmarked_sum);
    println!("    last_num: {:?}", last_num);

    unmarked_sum * last_num
}

fn solution2(input: &str) -> usize {
    let (numbers, mut bingos) = read_data(input);

    let mut active: HashSet<_> = (0..bingos.len()).collect();
    let mut last_num = 0;

    'outer: for number in numbers {
        for (i, bingo) in bingos.iter_mut().enumerate() {
            if bingo.apply(number) {
                if bingo.bingo().is_some() {
                    // if it is the last active board
                    // and it is bingo
                    // return this board
                    if active.len() == 1 {
                        let idx: &usize = active.iter().collect::<Vec<_>>().first().unwrap();
                        if *idx == i {
                            last_num = number;
                            break 'outer;
                        }
                    }
                    active.remove(&i);
                }
            }
        }
    }

    let idx: &usize = active.iter().collect::<Vec<_>>().first().unwrap();
    let last = bingos[*idx].clone();

    let mut unmarked_sum = 0;
    for (i, row) in last.inner.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if !last.view[i][j] {
                unmarked_sum += col;
            }
        }
    }

    // println!("LAST: {}", idx);
    last.print();

    println!("unmarked_sum: {:?}", unmarked_sum);
    println!("    last_num: {:?}", last_num);

    unmarked_sum * last_num
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

    const SAMPLE_INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    simple_test!(part1, solution1, SAMPLE_INPUT, 4512);
    simple_test!(part2, solution2, SAMPLE_INPUT, 1924);

    #[test]
    fn test_transpose() {
        let input = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
        ];

        let expected = vec![
            vec![1, 4],
            vec![2, 5],
            vec![3, 6],
        ];

        assert_eq!(expected, transpose(&input));
    }
}
