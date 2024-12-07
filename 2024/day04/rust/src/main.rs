#![allow(non_snake_case)]

fn read_matrix(input: &str) -> Vec<Vec<char>> {
    let mut matrix = Vec::new();
    for line in input.lines() {
        let row = line.chars().collect();
        matrix.push(row);
    }

    matrix
}

fn valid_xmas(v: Vec<Option<char>>) -> bool {
    v.iter().filter_map(|item| *item).collect::<String>() == "XMAS"
}

fn xmas_E(v: &[Vec<char>], row: usize, col: usize) -> bool {
    let mut vals: Vec<Option<char>> = vec![];

    for i in 0..=3 {
        let val = v.get(row).and_then(|r| r.get(col + i));
        vals.push(val.cloned());
    }

    valid_xmas(vals)
}

fn xmas_W(v: &[Vec<char>], row: usize, col: usize) -> bool {
    let mut vals: Vec<Option<char>> = vec![];

    for i in 0..=3 {
        if col < i {
            return false;
        }

        let val = v.get(row).and_then(|r| r.get(col - i));
        vals.push(val.cloned());
    }

    valid_xmas(vals)
}

fn xmas_S(v: &[Vec<char>], row: usize, col: usize) -> bool {
    let mut vals: Vec<Option<char>> = vec![];

    for i in 0..=3 {
        let val = v.get(row + i).and_then(|r| r.get(col));
        vals.push(val.cloned());
    }

    valid_xmas(vals)
}

fn xmas_N(v: &[Vec<char>], row: usize, col: usize) -> bool {
    let mut vals: Vec<Option<char>> = vec![];

    for i in 0..=3 {
        if row < i {
            return false;
        }

        let val = v.get(row - i).and_then(|r| r.get(col));
        vals.push(val.cloned());
    }

    valid_xmas(vals)
}

fn xmas_to_SE(v: &[Vec<char>], row: usize, col: usize) -> bool {
    let mut vals: Vec<Option<char>> = vec![];

    for i in 0..=3 {
        let val = v.get(row + i).and_then(|r| r.get(col + i));
        vals.push(val.cloned());
    }

    valid_xmas(vals)
}

fn xmas_to_NW(v: &[Vec<char>], row: usize, col: usize) -> bool {
    let mut vals: Vec<Option<char>> = vec![];

    for i in 0..=3 {
        if row < i || col < i {
            return false;
        }

        let val = v.get(row - i).and_then(|r| r.get(col - i));
        vals.push(val.cloned());
    }

    valid_xmas(vals)
}

fn xmas_to_NE(v: &[Vec<char>], row: usize, col: usize) -> bool {
    let mut vals: Vec<Option<char>> = vec![];

    for i in 0..=3 {
        if row < i {
            return false;
        }

        let val = v.get(row - i).and_then(|r| r.get(col + i));
        vals.push(val.cloned());
    }

    valid_xmas(vals)
}

fn xmas_to_SW(v: &[Vec<char>], row: usize, col: usize) -> bool {
    let mut vals: Vec<Option<char>> = vec![];

    for i in 0..=3 {
        if col < i {
            return false;
        }

        let val = v.get(row + i).and_then(|r| r.get(col - i));
        vals.push(val.cloned());
    }

    valid_xmas(vals)
}

fn xmas_at(v: &[Vec<char>], row: usize, col: usize) -> usize {
    let total: usize = 0;

    let items = [
        xmas_N(&v, row, col),
        xmas_W(&v, row, col),
        xmas_S(&v, row, col),
        xmas_E(&v, row, col),
        xmas_to_NE(&v, row, col),
        xmas_to_NW(&v, row, col),
        xmas_to_SW(&v, row, col),
        xmas_to_SE(&v, row, col),
    ];

    let more: Vec<bool> = items.into_iter().filter(|x| *x).collect();
    more.len()
}

fn main() {
    let path1 = std::env::args().nth(1).unwrap();
    let input = std::fs::read_to_string(path1).unwrap();

    let m = read_matrix(&input);
    let mut total = 0;

    for i in 0..m.len() {
        for j in 0..m[i].len() {
            total += xmas_at(&m, i,  j)
        }
    }

    println!("part1: {}", total);
}
