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

fn count_bag_type(input: &str) -> IResult<&str, (usize, BagType)> {
    let (input, d) = digit1(input)?;
    let (input, s) = space1(input)?;
    let (input, bt) = bag_type(input)?;
    let count: usize = d.parse().unwrap();
    Ok((input, (count, bt)))
}

fn no_other_bags(input: &str) -> IResult<&str, Vec<(usize ,BagType)>> {
    let (input, _) = tag("no other bags")(input)?;
    Ok((input, vec![]))
}

fn bags_list(input: &str) -> IResult<&str, Vec<(usize, BagType)>> {
    separated_list1(tag(", "), count_bag_type)(input)
}

pub fn bag_definition(input: &str) -> IResult<&str, BagD> {
    let (input, bag_t) = bag_type(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = tag("contain")(input)?;
    let (input, _) = space1(input)?;
    let (input, bags_list) =
        alt((
            no_other_bags,
            bags_list,
        ))(input)?;
    let (input, _) = tag(".")(input)?;

    Ok((input, BagD {
        typ: bag_t,
        contains: bags_list,
    }))
}

// fn extract_bag()

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
                (1, BagType::colored("bright white")),
                (2, BagType::colored("muted yellow")),
            ],
        })));
    }
    #[test]
    fn parse_bag2() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.";
        let bag = bag_definition(input);
        assert_eq!(bag.unwrap().1, BagD {
            typ: BagType::colored("light red"),
            contains: vec![
                (1, BagType::colored("bright white")),
                (2, BagType::colored("muted yellow")),
            ],
        });
    }

    #[test]
    fn parse_no_other_bags() {
        let input = "pale lavender bags contain no other bags.";
        let bag = bag_definition(input);
        assert_eq!(bag.unwrap().1, BagD {
            typ: BagType::colored("pale lavender"),
            contains: vec![],
        });
    }
}
