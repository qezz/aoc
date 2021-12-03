fn solution1(input: &str) -> String {
    todo!()
}

fn solution2(input: &str) -> String {
    todo!()
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

    const SAMPLE_INPUT: &str = "";

    simple_test!(part1, solution1, SAMPLE_INPUT, todo!());
    simple_test!(part2, solution2, SAMPLE_INPUT, todo!());
}
