pub struct Counter {
    inner: Vec<i32>,
    _data: Vec<String>,
}

pub fn get_bit(data: &[String], pos: usize) -> i32 {
    let mut c = Counter::new();
    for line in data {
        c.apply(line);
    }

    c.inner[pos]
}

pub fn get_major_bit(data: &[String], pos: usize) -> i32 {
    let bit = get_bit(data, pos);
    if bit >= 0 {
        1
    } else {
        0
    }
}

pub fn get_minor_bit(data: &[String], pos: usize) -> i32 {
    let bit = get_bit(data, pos);
    if bit < 0 {
        1
    } else {
        0
    }
}

impl Counter {
    pub fn new() -> Self {
        Self {
            inner: vec![],
            _data: vec![],
        }
    }

    pub fn apply(&mut self, s: &str) {
        self._data.push(s.into());

        if self.inner.is_empty() {
            self.inner = vec![0; s.len()];
        }

        for (i, ch) in s.char_indices() {
            match ch {
                '0' => {
                    self.inner[i] -= 1;
                },
                '1' => {
                    self.inner[i] += 1;
                },
                _ => {}
            }
        }
    }

    pub fn get_major(&self) -> usize {
        let mut res = 0;
        for i in self.inner.iter() {
            res *= 2;
            if *i > 0 {
                res += 1;
            }
        }

        res
    }

    pub fn get_minor(&self) -> usize {
        self.get_major() ^ (2_usize.pow(self.inner.len() as u32) - 1)
    }

    /// Replace `f_get_bit` with one of the functions
    ///
    /// `get_major_bit` -> oxygen rating
    /// `get_minor_bit` -> co2 rating
    pub fn find_rating<G>(&self, f_get_bit: G) -> String
    where G: Fn(&[String], usize) -> i32
    {
        let _width = self.inner.len();
        let mut the_data: Vec<String> = self._data.clone();

        let mut prefix = "".to_string();

        for i in 1..=self.inner.len() {
            let bit = f_get_bit(&the_data, i-1);

            if bit > 0 {
                prefix.push('1');
            } else {
                prefix.push('0');
            }

            let filtered: Vec<String> =
                self._data
                .iter()
                .filter(|&entry| entry.starts_with(&prefix))
                .map(|e| e.to_string())
                .collect();

            the_data = filtered;
            if the_data.len() == 1 {
                return the_data.first().unwrap().to_string();
            }
        }

        todo!()
    }
}

impl Default for Counter {
    fn default() -> Self {
        Self::new()
    }
}

pub fn solution1(input: &str) -> usize {
    let mut c = Counter::new();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        c.apply(line);
    }

    c.get_major() * c.get_minor()
}

pub fn solution2(input: &str) -> usize {
    let mut c = Counter::new();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        c.apply(line);
    }

    let oxy = c.find_rating(get_major_bit); // c.find_oxy();
    let oxy = usize::from_str_radix(&oxy, 2).unwrap();

    let co2 = c.find_rating(get_minor_bit); // c.find_co2();
    let co2 = usize::from_str_radix(&co2, 2).unwrap();

    oxy * co2
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

    #[test]
    fn woot() {
        let mut c = Counter::new();
        c.apply("10110");
        assert_eq!(c.get_major(), 22);
        assert_eq!(c.get_minor(), 9);
    }

    use utils::simple_test;

    const SAMPLE_INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    simple_test!(part1, solution1, SAMPLE_INPUT, 198);
    simple_test!(part2, solution2, SAMPLE_INPUT, 230);
}
