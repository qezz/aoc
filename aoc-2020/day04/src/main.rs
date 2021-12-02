use std::{collections::HashMap, fmt::format, str::FromStr};

use utils::data::multiline::{FromKVs, Invalid, MultilineParser, Sep};

#[derive(Clone, Debug)]
pub struct PassportStr {
    byr: String, // (Birth Year)
    iyr: String, // (Issue Year)
    eyr: String, // (Expiration Year)
    hgt: String, // (Height)
    hcl: String, // (Hair Color)
    ecl: String, // (Eye Color)
    pid: String, // (Passport ID)
    cid: Option<String>, // (Country ID)
}

impl FromKVs for PassportStr {
    fn from_kvs(hm: &HashMap<String, String>) -> Result<Self, Invalid> {
        let pass = Self {
            byr: hm.get("byr").ok_or(Invalid)?.into(),
            iyr: hm.get("iyr").ok_or(Invalid)?.into(),
            eyr: hm.get("eyr").ok_or(Invalid)?.into(),
            hgt: hm.get("hgt").ok_or(Invalid)?.into(),
            hcl: hm.get("hcl").ok_or(Invalid)?.into(),
            ecl: hm.get("ecl").ok_or(Invalid)?.into(),
            pid: hm.get("pid").ok_or(Invalid)?.into(),
            cid: hm.get("cid").map(|x| x.into()),
        };

        Ok(pass)
    }
}

#[derive(Clone, Debug)]
pub enum Height {
    In(usize),
    Cm(usize),
}

impl FromStr for Height {
    type Err = Invalid;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.ends_with("cm") {
            let x = s.rsplit_once("cm")
                .ok_or(Invalid)?
                .0
                .parse::<usize>()
                .unwrap();
            if x < 150 || x > 193 {
                return Err(Invalid);
            }
            Ok(Height::Cm(x))
        } else if s.ends_with("in") {
            let x = s.rsplit_once("in")
                .ok_or(Invalid)?
                .0
                .parse::<usize>()
                .unwrap();
            if x < 59 || x > 76 {
                return Err(Invalid);
            }
            Ok(Height::In(x))
        } else {
            Err(Invalid)
        }
    }
}

const EYES: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

#[derive(Clone, Debug)]
pub struct Passport {
    byr: usize, // (Birth Year)
    iyr: usize, // (Issue Year)
    eyr: usize, // (Expiration Year)
    hgt: Height, // (Height)
    hcl: String, // (Hair Color)
    ecl: String, // (Eye Color)
    pid: String, // (Passport ID)
    cid: Option<String>, // (Country ID)
}

// ugly
impl FromKVs for Passport {
    fn from_kvs(hm: &HashMap<String, String>) -> Result<Self, Invalid> {
        let byr = hm.get("byr").ok_or(Invalid)?.parse::<usize>().map_err(|_| Invalid)?;
        let byr_str = format!("{}", byr);
        if byr < 1920 || byr > 2002 || &byr_str != hm.get("byr").ok_or(Invalid)? {
            return Err(Invalid);
        }

        let iyr = hm.get("iyr").ok_or(Invalid)?.parse::<usize>().map_err(|_| Invalid)?;
        let iyr_str = format!("{}", iyr);
        if iyr < 2010 || iyr > 2020 || &iyr_str != hm.get("iyr").ok_or(Invalid)? {
            return Err(Invalid);
        }

        let eyr = hm.get("eyr").ok_or(Invalid)?.parse::<usize>().map_err(|_| Invalid)?;
        let eyr_str = format!("{}", eyr);
        if eyr < 2020 || eyr > 2030 || &eyr_str != hm.get("eyr").ok_or(Invalid)? {
            return Err(Invalid);
        }

        let hgt = hm.get("hgt").ok_or(Invalid)?.parse::<Height>().map_err(|_| Invalid)?;

        let hcl: String = hm.get("hcl").ok_or(Invalid)?.into();
        let hcl_color: String = hcl.clone()[1..].into();
        if !(hcl.starts_with("#") && hcl.len() == 7 && hcl_color.chars().all(|c| c.is_digit(16))) {
            return Err(Invalid);
        }

        let ecl: String = hm.get("ecl").ok_or(Invalid)?.into();
        if !EYES.contains(&ecl.as_str()) {
            return Err(Invalid);
        }

        let pid: String = hm.get("pid").ok_or(Invalid)?.into();
        if !(pid.len() == 9 && pid.chars().all(|c| c.is_digit(10))) {
            return Err(Invalid);
        }

        let pass = Self {
            byr,
            iyr,
            eyr,
            hgt,
            hcl,
            ecl,
            pid,
            cid: hm.get("cid").map(|x| x.into()),
        };

        Ok(pass)
    }
}


fn main() {
    let contents = std::fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    println!("part1: {}", solution1(&contents));
    println!("part2: {}", solution2(&contents));
}

pub fn solution1(input: &str) -> usize {
    let mlp: MultilineParser<PassportStr> = MultilineParser::new(Sep::EmptyLine);
    let res = mlp.parse(input);
    res.len()
}

pub fn solution2(input: &str) -> usize {
    let mlp: MultilineParser<Passport> = MultilineParser::new(Sep::EmptyLine);
    let res = mlp.parse(input);
    res.len()
}

#[cfg(test)]
mod tests {
    use utils::simple_test;

    use super::*;

    const SAMPLE_INPUT: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    const INVALID: &str = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007

iyr:2010 hgt:158cm hcl:#xxxyyy ecl:blu byr:1944 eyr:2021 pid:093154719
";

    const VALID: &str = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

    simple_test!(part1, solution1, SAMPLE_INPUT, 2);

    simple_test!(__part2_1, solution2, INVALID, 0);
    simple_test!(__part2_2, solution2, VALID, 4);
}
