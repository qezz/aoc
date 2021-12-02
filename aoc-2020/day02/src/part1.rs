use utils::input_to_data;

use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct Rule {
    pub ch: char,
    pub at_most: usize,
    pub at_least: usize,
}

#[derive(Clone, Debug)]
pub struct PasswordEntry {
    pub rule: Rule,
    pub value: String,
}

#[derive(Debug)]
pub struct ParseError(String);

impl FromStr for Rule {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // println!("parsing rule: {}", s);
        let sp: Vec<String> = s.split(' ').map(|x| x.to_string()).collect();
        if sp.len() < 2 {
            return Err(ParseError(s.into()));
        }
        let (range, ch) = (
            sp[0].clone(),
            sp[1].chars().next().unwrap(),
        );

        let range_sp: Vec<String> = range.split('-').map(|x| x.to_string()).collect();
        let (at_least, at_most) = (
            range_sp[0].parse::<usize>().map_err(|_| ParseError(s.into()))?,
            range_sp[1].parse::<usize>().map_err(|_| ParseError(s.into()))?,
        );

        Ok(Rule {
            ch,
            at_most,
            at_least,
        })
    }
}

impl FromStr for PasswordEntry {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // println!("parsing: {}", s);
        let sp: Vec<String> = s.split(':').map(|x| x.trim().to_string()).collect();
        if sp.len() < 2 {
            return Err(ParseError(s.into()));
        }
        let (rule, pass) = (
            sp[0].parse::<Rule>().unwrap(),
            sp[1].clone(),
        );

        Ok(PasswordEntry {
            rule,
            value: pass,
        })
    }
}

pub fn solution1(input: &str) -> i32 {
    let data: Vec<PasswordEntry> = input_to_data(input);
    // println!("data: {:?}", &data[..10]);
    let mut valid = 0;
    for entry in data {
        let count = entry.value.matches(entry.rule.ch).count();
        if count >= entry.rule.at_least && count <= entry.rule.at_most {
            valid += 1;
        }
    }

    valid
}
