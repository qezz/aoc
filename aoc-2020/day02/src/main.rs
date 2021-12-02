mod part1;
use part1::solution1;

mod part2;
use part2::solution2;

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

    const SAMPLE_INPUT: &str = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

    simple_test!(part1, solution1, SAMPLE_INPUT, 2);
    simple_test!(part2, solution2, SAMPLE_INPUT, 1);
}
