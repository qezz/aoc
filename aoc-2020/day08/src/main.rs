use vm::Vm;

mod parser;
mod vm;

fn solution1(input: &str) -> i64 {
    let mut vm = Vm::default();
    vm.load(input);
    vm.run_interrupt(|vm| {
        !vm._seen.contains(&vm.ps)
    })
}

fn solution2(input: &str) -> i64 {
    let mut vm = Vm::default();
    vm.load(input);
    vm.run_fix()
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

    const SAMPLE_INPUT: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    simple_test!(part1, solution1, SAMPLE_INPUT, 5);
    simple_test!(part2, solution2, SAMPLE_INPUT, 8);
}
