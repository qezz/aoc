use std::collections::HashMap;

use crate::parser::{BError, CLOSINGS, OPENINGS, Validator};

mod parser;

fn mapping() -> HashMap<char, usize> {
    [(')', 3), (']', 57), ('}', 1197), ('>', 25137)].into_iter().collect()
}

fn mapping2() -> HashMap<char, usize> {
    [(')', 1), (']', 2), ('}', 3), ('>', 4)].into_iter().collect()
}

fn brackets<'a, 'b>(openings: &'a [char], closings: &'b [char]) -> HashMap<char, char> {
    let mut hm = HashMap::new();
    for i in 0..openings.len() {
        hm.insert(openings[i], closings[i]);
    }

    hm
}

fn solution1(input: &str) -> usize {
    let validator = Validator::new(&OPENINGS, &CLOSINGS);
    let m = mapping();
    let mut sum = 0;
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let x = match validator.is_valid(line) {
            Err(BError::Corrupted(c)) => c,
            _ => continue,
        };

        sum += m[&x];
    }

    sum
}

fn solution2(input: &str) -> usize {
    let validator = Validator::new(&OPENINGS, &CLOSINGS);
    let brackets = brackets(&OPENINGS, &CLOSINGS);
    let m = mapping2();
    let mut sums = vec![];
    for line in input.lines() {

        if line.is_empty() {
            continue;
        }

        let mut stack = match validator.is_valid(line) {
            Err(BError::Incomplete(stack)) => stack,
            _ => continue,
        };

        // println!("stack: {:?}", stack);
        // todo!();
        stack.reverse();

        let mut sum = 0;
        for ch in stack {
            let n = brackets[&ch];
            let z = m[&n];

            sum *= 5;
            sum += z;
        }

        sums.push(sum);
    }

    sums.sort_unstable();

    sums[sums.len() / 2]
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

    const SAMPLE_INPUT: &str = "
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    simple_test!(part1, solution1, SAMPLE_INPUT, 26397);
    simple_test!(part2, solution2, SAMPLE_INPUT, 288957);
}
