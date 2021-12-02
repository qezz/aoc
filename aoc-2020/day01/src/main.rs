use std::collections::HashMap;

use utils::input_to_data;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hm: HashMap<i32, usize> = HashMap::new();

    for i in 0..nums.len() {
        let complement = target - nums[i];

        match hm.get(&complement) {
            Some(_) => {
                return vec![complement, 2020 - complement];
            },
            None => {
                hm.insert(nums[i], i);
            }
        }
    }

    unimplemented!();
}

fn solution1(input: &str) -> i32 {
    let data = input_to_data(input);
    let vals = two_sum(data, 2020);
    vals[0] * vals[1]
}

// Let's just brute force
fn three_sum(nums: Vec<i32>, target: i32) -> (i32, i32, i32) {
    for i in 0..nums.len() {
        for j in i..nums.len() {
            for k in j..nums.len() {
                if nums[i] + nums[j] + nums[k] == target {
                    return (nums[i], nums[j], nums[k]);
                }
            }
        }
    }

    unimplemented!();
}

fn solution2(input: &str) -> i32 {
    let data = input_to_data(input);
    let vals = three_sum(data, 2020);
    vals.0 * vals.1 * vals.2
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

    const SAMPLE_INPUT: &str = "1721
979
366
299
675
1456";

    simple_test!(part1, solution1, SAMPLE_INPUT, 514579);
    simple_test!(part2, solution2, SAMPLE_INPUT, 241861950);
}
