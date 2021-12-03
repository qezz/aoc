use nom::{
    IResult,
    branch::alt,
    bytes::complete::{
        tag,
        take_till
    },
    character::{
        complete::{
            digit1,
            space1
        }
    },
    multi::separated_list1
};

use crate::vm::Command;

fn signed_i64(input: &str) -> IResult<&str, i64> {
    let (input, sign_str) = alt((tag("+"), tag("-")))(input)?;
    let (input, number_str) = digit1(input)?;

    let mut num: i64 = number_str.parse().unwrap();
    if sign_str == "-" {
        num = -num;
    }

    Ok((input, num))
}

fn nop(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("nop")(input)?;
    let (input, _) = space1(input)?;
    let (input, n) = signed_i64(input)?;

    Ok((input, Command::Nop(n)))
}

fn acc(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("acc")(input)?;
    let (input, _) = space1(input)?;
    let (input, n) = signed_i64(input)?;

    Ok((input, Command::Acc(n)))
}

fn jmp(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("jmp")(input)?;
    let (input, _) = space1(input)?;
    let (input, n) = signed_i64(input)?;

    Ok((input, Command::Jmp(n)))
}

pub fn op(input: &str) -> IResult<&str, Command> {
    alt((
        nop,
        jmp,
        acc,
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nop() {
        let input = "nop +1";
        let cmd = nop(input).unwrap().1;
        assert_eq!(Command::Nop(1), cmd)
    }

    #[test]
    fn test_acc() {
        let input = "acc -1";
        let cmd = acc(input).unwrap().1;
        assert_eq!(Command::Acc(-1), cmd)
    }

    #[test]
    fn test_jmp() {
        let input = "jmp -1";
        let cmd = jmp(input).unwrap().1;
        assert_eq!(Command::Jmp(-1), cmd)
    }

    #[test]
    fn test_op() {
        let input = "jmp -1";
        let cmd = op(input).unwrap().1;
        assert_eq!(Command::Jmp(-1), cmd)
    }

}
