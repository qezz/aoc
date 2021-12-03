mod parser;
mod bag;

pub fn solution1(input: &str) -> usize {
    // let data: Vec<BagDefinition> = input_to_data(input);
    42
}

pub fn solution2(input: &str) -> usize {
    42
}

fn main() {
    let contents = std::fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    println!("part1: {}", solution1(&contents));
    println!("part2: {}", solution2(&contents));
}


