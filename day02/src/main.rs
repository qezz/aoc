mod common;

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

    const SAMPLE_INPUT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    simple_test!(part1, solution1, SAMPLE_INPUT, 150);
    simple_test!(part2, solution2, SAMPLE_INPUT, 900);
}

