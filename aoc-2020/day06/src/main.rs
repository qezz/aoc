use std::collections::{HashMap, HashSet, hash_map::Entry};

use utils::data::multiline::{MultilineParser, Sep};

pub fn solution1(input: &str) -> usize {
    let mlp: MultilineParser<()> = MultilineParser::new(Sep::EmptyLine);
    let groups = mlp.group_rows(input);

    let mut sum = 0;

    for group in groups.iter() {
        let mut hm = HashSet::with_capacity(groups.len());

        for entry in group {
            for ch in entry.chars() {
                hm.insert(ch);
            }
        }

        sum += hm.len();
    }

    sum
}

pub fn solution2(input: &str) -> usize {
    let mlp: MultilineParser<()> = MultilineParser::new(Sep::EmptyLine);
    let groups = mlp.group_rows(input);

    let mut sum = 0;

    for group in groups.iter() {
        let mut hm = HashMap::with_capacity(groups.len());
        let group_size = group.len();

        for entry in group {
            for ch in entry.chars() {
                match hm.entry(ch) {
                    Entry::Occupied(ref mut val) => {
                        *val.get_mut() += 1;
                    }
                    Entry::Vacant(vac) => {
                        vac.insert(1);
                    }
                }
            }
        }
        hm = hm.into_iter().filter(|(k, v)| *v == group_size).collect();
        sum += hm.len();
    }

    sum
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

    const SAMPLE_INPUT: &str = "abc

a
b
c

ab
ac

a
a
a
a

b";

    simple_test!(part1, solution1, SAMPLE_INPUT, 11);
    simple_test!(part2, solution2, SAMPLE_INPUT, 6);
}
