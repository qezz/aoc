

use crate::common::*;

pub struct Vm {
    depth: usize,
    forw: usize,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            depth: 0,
            forw: 0,
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
            },
            Down => {
                self.depth += val;
            },
            Up => {
                self.depth -= val;
            },
        }

        Ok(())
    }
}


pub fn solution1(input: &String) -> usize {
    let cmds = input_to_data(input);
    let mut vm = Vm::new();

    for cmd in cmds {
        vm.apply(&cmd).unwrap();
    }

    vm.depth * vm.forw
}
