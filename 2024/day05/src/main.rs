use parser::{Line, PageOrderRule, PagesPrinted};

pub mod parser;

#[derive(Debug)]
pub struct All {
    pub rules: Vec<PageOrderRule>,
    pub pages: Vec<PagesPrinted>,
}

fn do1(all: &All) -> Vec<PagesPrinted> {
    // I'm sure it's possible to have it more optimal, but I don't
    // know how.  Probably I can track the rules and have efficient
    // lookup if order is correct on the page.

    let mut total = 0;
    let mut invalid: Vec<PagesPrinted> = Vec::new();

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
                }
                _ => {}
            }
        }

        if order_is_valid {
            let mid = page.pages.len() / 2;
            total += page.pages[mid];
        } else {
            invalid.push(page.clone())
        }
    }

    println!("part1: {}", total);

    invalid
}

fn patch(rule: &PageOrderRule, printed: &PagesPrinted) -> (PagesPrinted, bool) {
    let mut patched = printed.clone();
    let mut changed = false;

    let page1: Option<_> = printed.pages.iter().position(|val| *val == rule.before);
    let page2: Option<_> = printed.pages.iter().position(|val| *val == rule.after);

    if let (Some(p1), Some(p2)) = (page1, page2) {
        if p1 > p2 {
            println!("orig: {:?}", printed);
            patched.pages.swap(p1, p2);
            changed = true;
            println!("upd:  {:?}", patched);
            println!();
        }
    }

    (patched, changed)
}

fn do2(rules: &[PageOrderRule], invalid: &[PagesPrinted]) -> u64 {
    let mut fixed_prints: Vec<PagesPrinted> = Vec::new();
    let mut total = 0;

    for print in invalid {
        let mut ever_changed = true;
        let mut maybe_patched = print.clone();

        while ever_changed {
            // let mut changed = false;
            ever_changed = false;
            for rule in rules {
                println!("rule: {}", rule);

                let (res, changed) = patch(rule, &maybe_patched);
                maybe_patched = res;
                if changed {
                    ever_changed = true;
                }
            }


            if ever_changed {
                continue
            } else {
                fixed_prints.push(maybe_patched.clone());
                break
            }
        }
    }

    println!();

    for print in fixed_prints {
        println!("fixed: {:?}", print.pages);
        let mid = print.pages.len() / 2;
        total += print.pages[mid];
    }

    total
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
                Line::PagesPrinted(page) => all.pages.push(page),
                Line::Empty => {}
            },
            Err(_) => todo!(),
        }
    }

    let invalid_prints = do1(&all);
    let part2 = do2(&all.rules, &invalid_prints);

    println!("part2: {}", part2)
}
