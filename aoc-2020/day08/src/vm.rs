use std::collections::HashSet;

use crate::parser::op;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Command {
    Acc(i64),
    Jmp(i64),
    Nop(i64), // yes
}

#[derive(Debug, Clone)]
pub struct Vm {
    pub accum: i64,
    pub ps: i64,
    pub prog: Vec<Command>,
    pub _seen: HashSet<i64>,

    // pub _history: Vec<>
}

pub fn clone_swap_current_to_end(s: &Vm) -> (bool, i64) {
    let mut vm: Vm = (*s).clone();
    println!("clone swap for {}: {:?}", vm.ps, vm.prog[vm.ps as usize]);
    let curr_cmd = vm.prog[vm.ps as usize].clone();

    match curr_cmd {
        Command::Acc(n) => vm.accum += n,
        Command::Jmp(n) => {
            vm.prog[vm.ps as usize] = Command::Nop(n);
        },
        Command::Nop(n) => {
            vm.prog[vm.ps as usize] = Command::Jmp(n);
        },
    }

    loop {
        if vm._seen.contains(&vm.ps) {
            return (false, 0);
        }
        vm._seen.insert(vm.ps);

        if vm.ps as usize == vm.prog.len() {
            break;
        }

        let cmd = vm.prog[vm.ps as usize].clone();
        match cmd {
            Command::Acc(n) => vm.accum += n,
            Command::Jmp(n) => {
                vm.ps += (n - 1);
                // continue;
            },
            Command::Nop(_) => {},
        }

        vm.ps += 1;
    }

    (true, vm.accum)
}

impl Vm {
    pub fn new() -> Self {
        Self {
            accum: 0,
            ps: 0,
            prog: vec![],
            _seen: HashSet::new(),
        }
    }

    pub fn load(&mut self, input: &str) {
        for line in input.lines() {
            let cmd = op(line).unwrap().1;
            self.prog.push(cmd);
        }
    }

    pub fn run<F>(&mut self, watcher: F) -> i64
    where F: Fn(&Vm) -> bool
    {
        loop {
            if !watcher(&self) {
                break;
            }

            let cmd = self.prog[self.ps as usize].clone();
            match cmd {
                Command::Acc(n) => self.accum += n,
                Command::Jmp(n) => {
                    self.ps += (n - 1);
                    // continue;
                },
                Command::Nop(_) => {},
            }

            self._seen.insert(self.ps);
            self.ps += 1;
        }

        self.accum
    }

    pub fn run_fix(&mut self) -> i64
    {
        loop {
            println!("ps: {}", self.ps);
            if self.prog.len() == self.ps as usize {
                break;
            }
            let cmd = self.prog[self.ps as usize].clone();
            match cmd {
                Command::Acc(n) => self.accum += n,
                Command::Jmp(n) => {
                    let (res, val) = clone_swap_current_to_end(self);
                    if res {
                        return val
                    }

                    self.ps += (n - 1);
                    // continue;
                },
                Command::Nop(_) => {
                    let (res, val) = clone_swap_current_to_end(self);
                    if res {
                        return val
                    }
                },
            }

            self._seen.insert(self.ps);
            self.ps += 1;
        }

        self.accum
    }
}

impl Default for Vm {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn woot() {
        let input = "nop +1
acc +1
jmp -1";
        let mut vm = Vm::default();
        vm.load(input);
        let res = vm.run(|vm| {
            !vm._seen.contains(&vm.ps)
        });

        assert_eq!(1, res)
    }
}
