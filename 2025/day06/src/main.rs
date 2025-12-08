use std::str::FromStr;

fn read_file(path: &str) -> String {
    let contents = std::fs::read_to_string(path).expect("Should have been able to read the file");

    contents
}

#[derive(Debug)]
enum Op {
    Sum,
    Mul,
}

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Op::Sum),
            "*" => Ok(Op::Mul),
            _ => Err(()),
        }
    }
}

impl Op {
    fn zero_val(&self) -> u64 {
        match self {
            Op::Sum => 0,
            Op::Mul => 1,
        }
    }
}

pub fn part1(name: &str, path: &str) {
    let input = read_file(path);
    let mut lines = input.lines().rev();

    let first_line = lines.next().unwrap();

    let ops = first_line
        .split_whitespace()
        .map(|s| s.parse::<Op>().unwrap())
        .collect::<Vec<_>>();

    let initial = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut sums = initial.clone();
    let mut muls = initial.clone();

    while let Some(line) = lines.next() {
        line.split_whitespace().enumerate().for_each(|(i, x)| {
            let num: u64 = x.parse().unwrap();

            sums[i] += num;
            muls[i] *= num;
        });
    }

    let mut res = vec![0_u64; ops.len()];

    for (i, op) in ops.iter().enumerate() {
        match op {
            Op::Sum => {
                res[i] = sums[i];
            }
            Op::Mul => {
                res[i] = muls[i];
            }
        }
    }

    let s: u64 = res.iter().sum();
    println!("{}: part1: {}", name, s);
}

pub fn print_matrix(v: &Vec<Vec<char>>) {
    for line in v {
        println!(" {:?}", line);
    }
}

pub fn part2(name: &str, path: &str) {
    let input = read_file(path);
    // let n_lines = input.lines().

    let line_count = input.lines().count();
    let column_count = input.lines().next().unwrap().len();

    let mut rotated = vec![vec!['x'; line_count - 1]; column_count];
    let mut ops = Vec::with_capacity(column_count);

    for (i, line) in input.lines().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            if i == line_count - 1 {
                // op
                if ch == '+' {
                    ops.push(Op::Sum);
                }
                if ch == '*' {
                    ops.push(Op::Mul);
                }
            } else {
                rotated[line.len() - 1 - j][i] = ch;
            }
        }
    }

    ops.reverse();
    // dbg!(ops);

    let lines: Vec<String> = rotated
        .into_iter()
        .map(|row| row.iter().collect::<String>())
        .collect();

    // dbg!(lines);

    let sp: Vec<_> = lines
        .split(|row| row.split_whitespace().count() == 0)
        .collect();

    let mut res: Vec<_> = ops.iter().map(|op| op.zero_val()).collect();

    for (i, block) in sp.iter().enumerate() {
        for line in block.iter() {
            let num: u64 = line.trim().parse().unwrap();

            match ops[i] {
                Op::Sum => res[i] += num,
                Op::Mul => res[i] *= num,
            }
        }
    }

    let s: u64 = res.iter().sum();
    println!("{}: part2: {}", name, s);
}

fn main() {
    part1("test", "test.txt");
    part2("test", "test.txt");

    part1("main", "input.txt");
    part2("main", "input.txt");
}
