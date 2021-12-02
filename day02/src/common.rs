use std::str::FromStr;

#[derive(Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Forward,
}

#[derive(Clone, Debug)]
pub struct Command {
    pub direction: Direction,
    pub value: usize,
}

impl Command {
    fn new(dir: Direction, val: usize) -> Self {
        Self {
            direction: dir,
            value: val,
        }
    }
}

pub struct ParseError(String);

impl FromStr for Command {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // println!("parsing: {}", s);
        let sp: Vec<String> = s.split(' ').map(|x| x.to_string()).collect();
        if sp.len() < 2 {
            return Err(ParseError(s.into()));
        }
        let (dir, value) = (
            sp[0].as_str(),
            sp[1].parse::<usize>().map_err(|_| ParseError(s.into()))?
        );
        let dir = match dir {
            "forward" => Direction::Forward,
            "up" => Direction::Up,
            "down" => Direction::Down,
            _ => { unimplemented!() }
        };

        Ok(Command::new(dir, value))
    }
}
