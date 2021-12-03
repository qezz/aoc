use std::collections::VecDeque;

use utils::data::lines::input_to_vecdeque;

mod math;
use math::count_options;

fn the_data(input: &str) -> Vec<usize> {
    let mut v: VecDeque<usize> = input_to_vecdeque(input);
    let maximum = v.iter().cloned().max().unwrap();

    v.push_front(0);
    v.push_back(maximum + 3);

    let mut v = Vec::from(v);
    v.sort_unstable();
    v
}

fn solution1(input: &str) -> usize {
    let v = the_data(input);

    let mut diffs = [0; 3];

    for win in v.windows(2) {
        let diff = win[1] - win[0];
        diffs[diff - 1] += 1;
    }

    println!("diffs: {:?}", diffs);
    diffs[0] * diffs[2]
}

/// Wrong assumption:
/// Options:
///
/// 123: [13, 123] (2)
/// 1234: [14, 124, 134, 1234] (4)
/// 12345: same {1234}5 + 1{2345} == 8
/// ...
/// basically 2.pow(len_of_sequence - 2)
///
/// for sequences of seqs, consider:
///
/// seq X has 4 options, Y - 3
/// then the sequence X - Y will have 4 * 3 options
#[allow(dead_code)]
fn solution2_wrong(input: &str) -> usize {
    let v = the_data(input);

    let mut curr_seq_len = 0;
    let mut total_options = 1;
    for win in v.windows(2) {
        println!("seq: {:?}", win);
        if win[1] - win[0] == 1 {
            curr_seq_len += 1;
            println!("   curr: {:?}", curr_seq_len);
        } else {
            if curr_seq_len > 1 {
                let n_opts = 2_usize.pow(curr_seq_len - 1);
                println!("   n_opt: {:?}", n_opts);
                total_options *= n_opts;
                println!("\n> total: {:?}\n", total_options);
            }

            curr_seq_len = 0;
        }
    }

    total_options
}

fn solution2(input: &str) -> usize {
    let v = the_data(input);

    let mut curr_seq_len = 0;
    let mut total_options = 1;
    for win in v.windows(2) {
        if win[1] - win[0] == 1 {
            curr_seq_len += 1;
        } else {
            let n_opts = count_options(curr_seq_len+1);
            total_options *= n_opts;

            curr_seq_len = 0;
        }
    }

    total_options
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

    const SAMPLE_INPUT: &str = "16
10
15
5
1
11
7
19
6
12
4";

    simple_test!(part1, solution1, SAMPLE_INPUT, 35);
    simple_test!(part2, solution2, SAMPLE_INPUT, 8);

    const SAMPLE_INPUT2: &str = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    simple_test!(part1_2, solution1, SAMPLE_INPUT2, 220);
    simple_test!(part2_2, solution2, SAMPLE_INPUT2, 19208);

    const SAMPLE_INPUT_SHORT1: &str = "3
4
5
6";
    simple_test!(part2_2_1, solution2, SAMPLE_INPUT_SHORT1, 4);

    const SAMPLE_INPUT_SHORT2: &str = "1
2
3";
    simple_test!(part2_2_2, solution2, SAMPLE_INPUT_SHORT2, 4);

    const SAMPLE_INPUT_SHORT3: &str = "3
4
5
6
9";
    simple_test!(part2_2_3, solution2_wrong, SAMPLE_INPUT_SHORT3, 4);

    const SAMPLE_INPUT_SHORT4: &str = "3
4
5
6
9
10";
    simple_test!(part2_2_4, solution2_wrong, SAMPLE_INPUT_SHORT4, 4);

    const SAMPLE_INPUT_SHORT5: &str = "3
4
5
6
9
10
11";
    simple_test!(part2_2_5, solution2_wrong, SAMPLE_INPUT_SHORT5, 8);

}
