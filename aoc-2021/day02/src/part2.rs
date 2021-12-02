use utils::data::lines::input_to_data;

use crate::common::*;

pub struct Vm {
    depth: usize,
    forw: usize,
    aim: usize,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            depth: 0,
            forw: 0,
            aim: 0,
        }
    }

    pub fn apply(&mut self, cmd: &Command) -> Result<(), ()> {
        // println!("applying: {:?}", cmd);
        use Direction::*;

        let dir = cmd.direction.clone();
        let val = cmd.value;
        match dir {
            Forward => {
                self.forw += val;
                self.depth += val * self.aim;
            },
            Down => {
                self.aim += val;
            },
            Up => {
                self.aim -= val;
            },
        }

        Ok(())
    }
}


pub fn solution2(input: &str) -> usize {
    let cmds = input_to_data(input);
    let mut vm = Vm::new();

    for cmd in cmds {
        vm.apply(&cmd).unwrap();
    }

    vm.depth * vm.forw
}
