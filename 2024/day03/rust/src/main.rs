use regex::Regex;

fn main() {
    let path1 = std::env::args().nth(1).unwrap();
    let input = std::fs::read_to_string(path1).unwrap();

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut muls = vec![];
    for (_, [left, right]) in re.captures_iter(&input).map(|c| c.extract()) {
        let l = left.parse::<u64>().unwrap();
        let r = right.parse::<u64>().unwrap();
        muls.push((l, r));
    }

    let s = muls.iter().fold(0, |acc, (l, r)| acc + l * r);
    println!("part1: {}", s);


    let path2 = std::env::args().nth(2).unwrap();
    let input = std::fs::read_to_string(path2).unwrap();

    let re2 = Regex::new(r"(?P<do>do\(\))|(?P<dont>don't\(\))|(?P<mul>mul\(\d{1,3},\d{1,3}\))").unwrap();
    let mut muls2 = vec![];

    let mut enabled = true;
    for x in re2.captures_iter(&input) {
        if let Some(_) = x.name("do") {
            enabled = true;
        }

        if let Some(_) = x.name("dont") {
            enabled = false;
        }

        if let Some(m) = x.name("mul") {
            // println!("{}, {:?}", enabled, m);
            if enabled {
                let cap = re.captures(&m.as_str()).unwrap();
                let (full, [left, right]) = cap.extract();
                let l = left.parse::<u64>().unwrap();
                let r = right.parse::<u64>().unwrap();
                muls2.push((l, r));
            }
        }
    }

    // println!("{:?}", muls);
    let s = muls2.iter().fold(0, |acc, (l, r)| acc + l * r);
    println!("part2: {}", s);
}
