use parser::{Line, PageOrderRule, PagesPrinted};

pub mod parser;

#[derive(Debug)]
pub struct All {
    pub rules: Vec<PageOrderRule>,
    pub pages: Vec<PagesPrinted>,
}

fn do1(all: &All) {
    // I'm sure it's possible to have it more optimal, but I don't
    // know how.  Probably I can track the rules and have efficient
    // lookup if order is correct on the page.

    let mut total = 0;

    for page in &all.pages {
        let mut order_is_valid = false;
        for rule in &all.rules {
            // println!("{:?}", rule);
            let page1: Option<_> = page.pages.iter().position(|val| *val == rule.before);
            let page2: Option<_> = page.pages.iter().position(|val| *val == rule.after);

            match (page1, page2) {
                (Some(p1), Some(p2)) => {
                    if p1 < p2 {
                        order_is_valid = true;
                    } else {
                        order_is_valid = false;
                        break;
                    }
                },
                _ => {}
            }
        }

        if order_is_valid {
            let mid = page.pages.len() / 2;
            total += page.pages[mid];
        }
    }

    println!("part1: {}", total)
}

fn main() {
    let path1 = std::env::args().nth(1).unwrap();
    let input = std::fs::read_to_string(path1).unwrap();

    let mut all = All {
        rules: Vec::new(),
        pages: Vec::new(),
    };

    for line in input.lines() {
        let linet = parser::parse_line(line);

        match linet {
            Ok((_, l)) => match l {
                Line::PageOrderRule(rule) => {
                    all.rules.push(rule);
                }
                Line::PagesPrinted(page) => {
                    all.pages.push(page)
                }
                Line::Empty => {}
            },
            Err(_) => todo!(),
        }
    }

    do1(&all)
}
