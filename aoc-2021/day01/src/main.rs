use utils::data::lines::input_to_data;

fn solution1(input: &str) -> String {
    let v = input_to_data::<usize>(input);
    let mut incs = 0;

    for pair in v.windows(2) {
        if pair[1] > pair[0] {
            incs += 1;
        }
    }

    format!("{}", incs)
}

fn solution2(input: &str) -> String {
    let v = input_to_data(input);
    let mut last: Option<usize> = None;
    let mut incs = 0;

    for win in v.windows(3) {
        let sum = win.iter().sum();
        if let Some(val) = last {
            if val < sum {
                incs += 1;
            }
        }
        last = Some(sum);
    }

    format!("{}", incs)
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

    const SAMPLE_INPUT: &str = "199
200
208
210
200
207
240
269
260
263";

    simple_test!(part1, solution1, SAMPLE_INPUT, 7);
    simple_test!(part2, solution2, SAMPLE_INPUT, 5);
}

