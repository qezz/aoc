mod parser;
mod bag;

use std::collections::{HashMap, HashSet};

use bag::BagD;
use parser::bag_definition;

use crate::bag::BagType;

/// Idea is to build a reversed mapping of the input data
///
/// Consider you need to find B, consider input:
/// ```
/// A: B, C
/// D: A
/// F: G, C
/// ```
///
/// The answer is: A (directly), D (indirectly)
///
/// Intermediate representation:
/// ```
/// A: D (reads: 'A could be reached from D')
/// B: A
/// C: A, F
/// G: F
/// ```
///
/// Then reduce it to the final result:
/// ```
/// B: A (first iteration)
/// (now go find the same for bag 'A', it's 'D':)
/// B: A, D
/// (next iterations won't change the input, 'break' here)
/// ```
pub fn solution1(input: &str) -> usize {
    let mut v = vec![];
    for line in input.lines() {
        let bag = bag_definition(line).unwrap().1;
        v.push(bag);
    }

    let mut hm: HashMap<BagType, HashSet<BagType>> = HashMap::new();

    for bag in &v {
        for nested in &bag.contains {
            let nested = nested.clone().1;
            let ent = hm.entry(nested.clone()).or_insert_with(HashSet::new);
            (*ent).insert(bag.clone().typ);
        }
    }

    let shiny = hm.get(&BagType::colored("shiny gold"));
    let mut res: HashSet<BagType> = shiny.unwrap().clone();
    let mut cheched: HashSet<BagType> = HashSet::new();

    for _ in 0..input.lines().count() {
        let mut changed = false;
        let _res = res.clone();

        for b in _res {
            // allows to skip already seen
            if cheched.get(&b).is_some() {
                continue;
            }

            if let Some(s) = hm.get(&b) {
                res = res.union(s).cloned().collect();
            }
            cheched.insert(b.clone());
            changed = true;
        }

        // allows to break once no changes are done to the result
        if !changed {
            break;
        }
    }

    res.len()
}

fn get_inner_bag_count(data: &HashMap<BagType, Vec<(usize, BagType)>>, color: &str) -> usize { //  Option<usize> {
    let bag = BagType::colored(color);
    let inner = data.get(&bag).unwrap();

    let mut sum = 0; // = Some(0);

    for b in inner {
        let multiplier = b.0;
        let count = 1 + get_inner_bag_count(data, &b.1.color);
        sum += multiplier * count;
    }

    sum
}

pub fn solution2(input: &str) -> usize {
    let mut hm = HashMap::new();
    for line in input.lines() {
        let bag = bag_definition(line).unwrap().1;
        hm.insert(bag.typ, bag.contains);
    }

    get_inner_bag_count(&hm, "shiny gold")
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

    const SAMPLE_INPUT: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    const SAMPLE_INPUT2: &str = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

    simple_test!(part1, solution1, SAMPLE_INPUT, 4);
    // simple_test!(part2, solution2, SAMPLE_INPUT2, 1);

    const SAMPLE_INPUT3: &str = "shiny gold bags contain 2 dark red bags.
dark red bags contain no other bags.";
    simple_test!(part2_1, solution2, SAMPLE_INPUT3, 2);

    const SAMPLE_INPUT2_2: &str = "shiny gold bags contain 1 dark red bags.
dark red bags contain 1 dark orange bags.
dark orange bags contain no other bags.";
    simple_test!(part2_two_bags, solution2, SAMPLE_INPUT2_2, 2);

    const SAMPLE_INPUT2_3: &str = "shiny gold bags contain 1 dark red bag.
dark red bags contain no other bags.";
    simple_test!(part2_one_bag, solution2, SAMPLE_INPUT2_3, 1);

    const SAMPLE_INPUT2_4: &str = "shiny gold bags contain 2 dark red bags.
dark red bags contain no other bags.";
    simple_test!(part2_4, solution2, SAMPLE_INPUT2_4, 2);

    const SAMPLE_INPUT2_5: &str = "shiny gold bags contain 7 dark red bags.
dark red bags contain no other bags.";
    simple_test!(part2_seven, solution2, SAMPLE_INPUT2_5, 7);
    const SAMPLE_INPUT2_6: &str = "shiny gold bags contain 7 dark red bags.
dark red bags contain 1 dark orange bags.
dark orange bags contain no other bags.";
    simple_test!(part2_seven2, solution2, SAMPLE_INPUT2_6, 14);

    const SAMPLE_INPUT2_7: &str = "shiny gold bags contain 7 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain no other bags.";
    simple_test!(part2_14bag, solution2, SAMPLE_INPUT2_7, 21);

}


