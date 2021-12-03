use std::collections::HashMap;

use utils::data::lines::input_to_data;

fn is_two_sum(nums: &[usize], target: i32) -> bool {
    let mut hm: HashMap<i32, usize> = HashMap::new();

    for i in 0..nums.len() {
        let complement = target - nums[i] as i32;

        match hm.get(&complement) {
            Some(_) => {
                return true;
            },
            None => {
                hm.insert(nums[i] as i32, i);
            }
        }
    }

    false
}

fn solution1(input: &str, win: usize) -> usize {
    let v: Vec<usize> = input_to_data(input);
    let wrong =
        v.windows(win + 1)
        // .inspect(|w| println!("{:?}", w))
        .find(|&window| {
            let prefix = &window[..win];
            let target = window[win];

            !is_two_sum(prefix, target as i32)
        });

    wrong.unwrap()[win]
}

fn solution2(input: &str, win: usize) -> usize {
    let v: Vec<usize> = input_to_data(input);
    let wrong = solution1(input, win);

    // large filtered
    let v: Vec<usize> = v.iter().filter(|&x| *x < wrong).copied().collect();

    for i in 2..v.len() {
        for win in v.windows(i) {
            if win.iter().sum::<usize>() == wrong {
                let small = *win.iter().min().unwrap();
                let large = *win.iter().max().unwrap();

                return small + large;
            }
        }
    }
    todo!()
}

fn main() {
    let contents = std::fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    println!("part1: {}", solution1(&contents, 25));
    println!("part2: {}", solution2(&contents, 25));
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::{simple_test, simple_test2};

    const SAMPLE_INPUT: &str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    simple_test2!(part1, solution1, (SAMPLE_INPUT, 5), 127);
    simple_test2!(part2, solution2, (SAMPLE_INPUT, 5), 62);
}
