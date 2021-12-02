use utils::data::lines::input_to_data;

use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct Rule {
    pub ch: char,
    pub first: usize,
    pub second: usize,
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
        let (first, second) = (
            range_sp[0].parse::<usize>().map_err(|_| ParseError(s.into()))?,
            range_sp[1].parse::<usize>().map_err(|_| ParseError(s.into()))?,
        );

        Ok(Rule {
            ch,
            first: first - 1,
            second: second - 1,
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

fn is_present_ch(entry: &PasswordEntry, pos: usize, ch: char) -> bool {
    entry.value.chars().nth(pos).map_or(false, |c| c == ch)
}

fn is_valid(entry: &PasswordEntry) -> bool {
    let cond1 = is_present_ch(entry, entry.rule.first, entry.rule.ch);
    let cond2 = is_present_ch(entry, entry.rule.second, entry.rule.ch);

    cond1 ^ cond2
}

pub fn solution2(input: &str) -> i32 {
    let data: Vec<PasswordEntry> = input_to_data(input);
    let mut valid = 0;
    for entry in data {
        if is_valid(&entry) {
            valid += 1;
        }
    }

    valid
}
