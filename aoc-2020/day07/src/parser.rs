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

use crate::bag::*;

fn till_space(input: &str) -> IResult<&str, &str> {
    take_till(|c| c == ' ')(input)
}

fn number_usize(s: &str) -> IResult<&str, usize> {
    let (input, val_str) = take_till(|c| c == ' ')(s)?;
    let val = val_str.parse::<usize>().unwrap();

    Ok((input, val))
}

fn bag_color2(input: &str) -> IResult<&str, String> {
    let (input, first) = till_space(input)?;
    let (input, _) = space1(input)?;
    let (input, second) = till_space(input)?;

    // Ok(format!("{} {}", first, second))
    Ok((input, format!("{} {}", first, second)))
}

// wavy green bags
fn bag_type(input: &str) -> IResult<&str, BagType> {
    let (input, color) = bag_color2(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = alt((tag("bags"), tag("bag")))(input)?;

    Ok((input, BagType::colored(&color)))
}

fn count_bag_type(input: &str) -> IResult<&str, BagType> {
    let (input, d) = digit1(input)?;
    let (input, s) = space1(input)?;
    let (input, bt) = bag_type(input)?;
    Ok((input, bt))
}

fn bags_list(input: &str) -> IResult<&str, Vec<BagType>> {
    separated_list1(tag(", "), count_bag_type)(input)
}

fn bag_definition(input: &str) -> IResult<&str, BagD> {
    let (input, bag_t) = bag_type(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = tag("contain")(input)?;
    let (input, _) = space1(input)?;
    let (input, bags_list) = bags_list(input)?;
    let (input, _) = tag(".")(input)?;

    Ok((input, BagD {
        typ: bag_t,
        contains: bags_list,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_bag() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.";
        let bag = bag_definition(input);
        assert_eq!(bag, Ok(("", BagD {
            typ: BagType::colored("light red"),
            contains: vec![
                BagType::colored("bright white"),
                BagType::colored("muted yellow"),
            ],
        })));
    }
}
