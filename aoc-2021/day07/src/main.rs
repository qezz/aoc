fn solution1(input: &str) -> usize {
    let data: Vec<usize> = input.trim().split(',').filter_map(|x| x.parse::<usize>().ok()).collect();
    let maxv = *data.iter().max().unwrap();

    let mut spends = Vec::with_capacity(maxv + 1);
    for i in 0..=maxv {
        spends.push({
            let v: Vec<usize> = data.iter().map(|x| {
                (*x as i64 - i as i64).abs() as usize
            }).collect();

            v.iter().sum()
        });
    }

    *spends.iter().min().unwrap()
}

fn fuel(start: i64, end: i64) -> usize {
    let n_steps = (end - start).abs();
    ((n_steps + 1) * n_steps / 2) as usize
}

fn solution2(input: &str) -> usize {
    let data: Vec<i64> = input.trim().split(',').filter_map(|x| x.parse::<i64>().ok()).collect();
    let maxv = *data.iter().max().unwrap();

    let mut spends = Vec::with_capacity(maxv as usize + 1);
    for i in 0..=maxv {
        spends.push({
            let v: Vec<usize> = data.iter().map(|x| {
                fuel(*x, i)
            }).collect();

            v.iter().sum()
        });
    }

    *spends.iter().min().unwrap()
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

    #[test]
    fn test_fuel() {
        assert_eq!(fuel(1, 1), 0);
        assert_eq!(fuel(1, 2), 1);
        assert_eq!(fuel(1, 3), 3);
        assert_eq!(fuel(1, 5), 10);
        assert_eq!(fuel(16, 5), 66);
    }

    const SAMPLE_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    simple_test!(part1, solution1, SAMPLE_INPUT, 37);
    simple_test!(part2, solution2, SAMPLE_INPUT, 168);
}
