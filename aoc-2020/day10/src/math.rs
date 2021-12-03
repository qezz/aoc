pub fn has_sequential_ones_at_most(number: usize, amount: usize) -> bool {
    let s = format!("{:b}", number);
    let most = "1".repeat(amount);
    s.contains(&most)
}

pub fn count_options(len: usize) -> usize {
    if len <= 2 {
        return 1;
    }
    let inner = len - 2;
    let range = (0.. 2_usize.pow(inner as u32)).collect::<Vec<usize>>();
    range.iter().filter(|&x| !has_sequential_ones_at_most(*x, 3)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sequential_ones() {
        assert!(has_sequential_ones_at_most(7, 3));
        assert!(!has_sequential_ones_at_most(8, 3));
        assert!(has_sequential_ones_at_most(14, 3));
        assert!(has_sequential_ones_at_most(30, 3));
    }

    #[test]
    fn options() {
        assert_eq!(count_options(1), 1);
        assert_eq!(count_options(2), 1);
        assert_eq!(count_options(3), 2);
        assert_eq!(count_options(4), 4);
        assert_eq!(count_options(5), 7);
        assert_eq!(count_options(6), 13);
    }
}
