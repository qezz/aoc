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

type Mas = ((isize, isize), (isize, isize), (isize, isize));

const MAS_NW_SE: Mas = ((-1, -1), (0, 0), (1, 1));
const MAS_SE_NW: Mas = ((1, 1), (0, 0), (-1, -1));
const MAS_NE_SW: Mas = ((1, -1), (0, 0), (-1, 1));
const MAS_SW_NE: Mas = ((-1, 1), (0, 0), (1, -1));

fn diag_mas(v: &[Vec<char>], row: usize, col: usize, pattern: Mas) -> bool {
    let cur = v[row][col];
    if cur != 'A' {
        return false;
    }

    if let Some(x) = v
        .get(row.wrapping_add_signed(pattern.0 .0))
        .and_then(|r| r.get(col.wrapping_add_signed(pattern.0 .1)))
    {
        if *x != 'M' {
            return false;
        }
    } else {
        return false;
    }

    if let Some(x) = v
        .get(row.wrapping_add_signed(pattern.2 .0))
        .and_then(|r| r.get(col.wrapping_add_signed(pattern.2 .1)))
    {
        if *x != 'S' {
            return false;
        }
    } else {
        return false;
    }

    true
}

fn cross_mas(v: &[Vec<char>], row: usize, col: usize) -> bool {
    let items = [
        diag_mas(v, row, col, MAS_NW_SE),
        diag_mas(v, row, col, MAS_SE_NW),
        diag_mas(v, row, col, MAS_NE_SW),
        diag_mas(v, row, col, MAS_SW_NE),
    ];
    // println!("items: {:?}", items);

    let more: Vec<bool> = items.into_iter().filter(|x| *x).collect();
    // println!("more {} {}: {:?}", row, col, more);
    more.len() == 2
}

fn main() {
    let path1 = std::env::args().nth(1).unwrap();
    let input = std::fs::read_to_string(path1).unwrap();

    let m = read_matrix(&input);
    let mut total = 0;
    let mut total2 = 0;

    for i in 0..m.len() {
        for j in 0..m[i].len() {
            total += xmas_at(&m, i, j);
            if cross_mas(&m, i, j) {
                total2 += 1;
            }
        }
    }

    println!("part1: {}", total);
    println!("part2: {}", total2);
}
