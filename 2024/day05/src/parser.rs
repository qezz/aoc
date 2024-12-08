use std::fmt::Display;

use nom::{
    bytes::complete::{tag, take_until1, take_while},
    character::complete::{digit1, space1},
    combinator::opt,
    multi::many1,
    IResult,
};

#[derive(Debug, PartialEq)]
pub struct PageOrderRule {
    pub before: u64,
    pub after: u64,
}

impl Display for PageOrderRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} | {}", self.before, self.after)
    }
}

pub fn page_order_rule_line(input: &str) -> IResult<&str, PageOrderRule> {
    let (input, before) = take_until1("|")(input)?;
    let (input, _) = tag("|")(input)?;
    let (input, after) = take_while(|_| true)(input)?;

    Ok((
        input,
        PageOrderRule {
            before: before.parse().unwrap(),
            after: after.parse().unwrap(),
        },
    ))
}

#[derive(Clone, Debug, PartialEq)]
pub struct PagesPrinted {
    pub pages: Vec<u64>,
}

pub fn one_page(input: &str) -> IResult<&str, u64> {
    let (input, page) = digit1(input)?;
    let (input, _) = opt(tag(","))(input)?;
    Ok((input, page.parse().unwrap()))
}

pub fn pages_printed_line(input: &str) -> IResult<&str, PagesPrinted> {
    let (input, pages) = many1(one_page)(input)?;

    Ok((input, PagesPrinted { pages }))
}

#[derive(Debug, PartialEq)]
pub enum Line {
    PageOrderRule(PageOrderRule),
    PagesPrinted(PagesPrinted),
    Empty,
}

pub fn parse_line(input: &str) -> IResult<&str, Line> {
    if let Ok((_, rule)) = page_order_rule_line(input) {
        return Ok((input, Line::PageOrderRule(rule)));
    }

    if let Ok((_, pages)) = pages_printed_line(input) {
        return Ok((input, Line::PagesPrinted(pages)));
    }

    if input.is_empty() {
        return Ok((input, Line::Empty));
    }

    panic!("Failed to parse line: {}", input);
}
