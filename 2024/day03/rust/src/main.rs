use regex::Regex;

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let input = std::fs::read_to_string(path).unwrap();

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut muls = vec![];
    for (_, [left, right]) in re.captures_iter(&input).map(|c| c.extract()) {
        let l = left.parse::<u64>().unwrap();
        let r = right.parse::<u64>().unwrap();
        muls.push((l, r));
    }

    // println!("{:?}", muls);
    let s = muls.iter().fold(0, |acc, (l, r)| acc + l * r);
    println!("{}", s);
}
