pub struct Vm {
    ticks: usize,
    inner: Vec<usize>,
}

impl Vm {
    pub fn new(data: Vec<usize>) -> Self {
        Self {
            ticks: 0,
            inner: data,
        }
    }

    pub fn tick(&mut self) {
        let mut tmp = vec![];
        for item in &mut self.inner {
            if *item == 0 {
                *item = 6;
                tmp.push(8);
            } else {
                *item -= 1;
            }
        }
        self.inner.extend(tmp.iter());
    }

    pub fn run(&mut self, days: usize) -> Vec<usize> {
        self.take(days).collect::<()>();
        self.inner.clone()
    }
}

impl Iterator for Vm {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        self.tick();
        Some(())
    }
}

pub struct Vm2 {
    inner: Vec<usize>,
}

impl Vm2 {
    pub fn new(data: Vec<usize>) -> Self {
        let mut res = vec![0; 9];
        for item in data {
            res[item] += 1;
        }
        Self {
            inner: res
        }
    }

    pub fn tick(&mut self) {
        let new = self.inner[0];
        // shift
        for i in 0..(self.inner.len() - 1) {
            // let tmp = self.inner[i + 1];
            self.inner[i] = self.inner[i + 1];
        }

        let l = self.inner.len();
        self.inner[l - 1] = new;
        self.inner[l - 3] += new;
    }

    pub fn run(&mut self, days: usize) -> Vec<usize> {
        self.take(days).collect::<()>();
        self.inner.clone()
    }
}

impl Iterator for Vm2 {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        // println!("pre tick: {:?}", self.inner);
        self.tick();
        Some(())
    }
}

fn solution1(input: &str, epochs: usize) -> usize {
    let data: Vec<usize> = input.trim().split(',').map(|x| x.parse::<usize>().unwrap()).collect();
    let mut vm = Vm::new(data);
    vm.run(epochs).len()
}

fn solution2(input: &str, epochs: usize) -> usize {
    let data: Vec<usize> = input.trim().split(',').map(|x| x.parse::<usize>().unwrap()).collect();
    let mut vm = Vm2::new(data);
    vm.run(epochs);
    // rintln!("self.inner: {:?}", vm.inner);
    vm.inner.iter().sum()
}

fn main() {
    let contents = std::fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    println!("part1: {}", solution1(&contents, 80));
    println!("part2: {}", solution2(&contents, 256));
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::{simple_test, simple_test2};

    const SAMPLE_INPUT: &str = "3,4,3,1,2";

    simple_test2!(part1, solution1, (SAMPLE_INPUT, 18), 26);
    simple_test2!(part2, solution2, (SAMPLE_INPUT, 256), 26984457539_usize);
}
