use std::{collections::{HashMap, HashSet}, io::BufRead, str::FromStr};

use utils::data::lines::input_to_data;

pub struct Entry {
    unique: Vec<String>,
    output: Vec<String>,
}

impl FromStr for Entry {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(())
        }
        let sp: Vec<String> = s.trim().split('|').map(|part| part.trim().into()).collect();
        let unique = sp[0].split(' ').map(Into::into).collect();
        let output = sp[1].split(' ').map(Into::into).collect();

        Ok(Entry {
            unique,
            output,
        })
    }
}

fn count_1478(v: &[String]) -> usize {
    v.iter().filter(|&x| [2, 3, 4, 7].contains(&x.len())).count()
}

fn solution1(input: &str) -> usize {
    let data: Vec<Entry> = input_to_data(input);
    let mut sum = 0;
    for entry in data {
        sum += count_1478(&entry.output);
    }

    sum
}

type Number = HashSet<char>;

fn get_with_len(v: &[String], l: usize) -> HashSet<char> {
    let s: String = v.iter().find(|&n| n.len() == l).unwrap().to_owned();
    s.chars().collect::<HashSet<_>>()
}

fn guess_numbers(v: &Vec<String>) -> Vec<HashSet<char>> {
    let mut numbers = vec![Number::new(); 10];

    // 0 - top segment
    // 1 - top left
    // 2 - top right
    // 3 - middle
    // 4 - bottom left
    // 5 - bottom right
    // 6 - bottom
    let mut segments: Vec<char> = vec![' '; 10];

    // guess 1
    {
        let s: String = v.iter().find(|&n| n.len() == 2).unwrap().to_owned();
        let n = s.chars().collect::<HashSet<_>>();
        numbers[1] = n;
    }

    // guess 7 and top-segment
    {
        let s: String = v.iter().find(|&n| n.len() == 3).unwrap().to_owned();
        let n = s.chars().collect::<HashSet<_>>();

        assert!(n.is_superset(&numbers[1]));
        let top_segment: HashSet<_> = n.difference(&numbers[1]).collect();
        segments[0] = *top_segment.iter().next().unwrap().to_owned();
        numbers[7] = n;
    }

    // 1, 7

    // guess 3
    {
        let n: HashSet<char> = v.iter()
            .filter(|&n| n.len() == 5)
            .map(|n| n.chars().collect::<HashSet<_>>())
            .find(|n| n.is_superset(&numbers[7]))
            .unwrap();
        numbers[3] = n;
    }

    // 1, 3, 7

    // guess 8
    {
        let n: HashSet<char> = v.iter()
            .filter(|&n| n.len() == 7)
            .map(|n| n.chars().collect::<HashSet<_>>())
            .find(|n| n.is_superset(&numbers[3]))
            .unwrap();
        numbers[8] = n;
    }

    // 1, 3, 7, 8

    // guess 4 and top left
    {
        let n = get_with_len(v, 4);
        let top_left = &n.difference(&numbers[3]).next().unwrap();
        let top_left = **top_left;
        segments[1] = top_left;
        numbers[4] = n;
    }

    // 1, 3, 4, 7, 8

    // guess 0, 6 and 9
    {
        let ns = v.iter()
            .filter(|&n| n.len() == 6)
            .map(|n| n.chars().collect::<HashSet<_>>());
        //     .find(|n| n.is_superset(&numbers[3]))
        //     .unwrap();
        // numbers[9] = n;
        // assert_eq!(ns.collect::<Vec<_>>().len(), 2);
        for (i, item) in ns.enumerate() {
            if item.is_superset(&numbers[3]) {
                numbers[9] = item;
            } else if item.is_superset(&numbers[1]) {
                numbers[0] = item;
            } else {
                numbers[6] = item;
            }

            assert!(i < 3);
        }
    }

    // 0, 1, 3, 4, 6, 7, 8, 9

    // guess top right and bottom right based on 1 and 6
    {
        let intersect = numbers[1].intersection(&numbers[6]);
        let ch_sh = intersect.map(|c| *c).collect::<HashSet<char>>(); // bottom right
        let ch = ch_sh.iter().next().unwrap(); // bottom right
        segments[5] = *ch;
        let mut top_right = numbers[1].difference(&ch_sh);
        segments[2] = *top_right.next().unwrap();
    }

    // guess 2 and 5
    {
        let ns = v.iter()
            .filter(|&n| n.len() == 5)
            .map(|n| n.chars().collect::<HashSet<_>>());
        for (i, item) in ns.enumerate() {
            if item.is_superset(&numbers[1]) && item.is_superset(&numbers[7]) {
                // do nothing, it's 3
            } else if item.contains(&segments[2]) {
                numbers[2] = item;
            } else {
                numbers[5] = item;
            }

            assert!(i < 3);
        }
    }

    numbers
}

fn find_number(known: &Vec<HashSet<char>>, s: &str) -> usize {
    let h = s.chars().collect::<HashSet<char>>();
    for (i, n) in known.iter().enumerate() {
        if n == &h {
            return i;
        }
    }

    //     known.iter().find(|&n| n == &h).unwrap()

    todo!();
}

fn final_number(known: &Vec<HashSet<char>>, out: &Vec<String>) -> usize {
    let mut res = 0;
    for digit in out {
        res *= 10;
        let val = find_number(known, digit);
        res += val;
    }

    res
}

fn solution2(input: &str) -> usize {
    let data: Vec<Entry> = input_to_data(input);
    let mut sum = 0;
    for entry in data {
        let numbers = guess_numbers(&entry.unique);
        let val = final_number(&numbers, &entry.output);
        sum += val;
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
    use super::*;
    use utils::simple_test;

    const SAMPLE_INPUT: &str = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    simple_test!(part1, solution1, SAMPLE_INPUT, 26);
    simple_test!(part2, solution2, SAMPLE_INPUT, 61229);
}
