use std::str::FromStr;

pub fn input_to_data<T: FromStr>(input: &str) -> Vec<T> {
    input.split('\n').filter_map(|line| line.trim().parse::<T>().ok()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_to_data() {
        let input = "123
456";
        let v: Vec<usize> = input_to_data(input);
        assert_eq!(v, vec![123, 456]);
    }
}
