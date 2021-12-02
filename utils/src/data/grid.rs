use std::str::FromStr;
use std::fmt::Debug;

pub trait GridTrait {
    type Item: Debug;

    fn fixed(width: usize, height: usize) -> Self;
    fn insert_row(&mut self, row: &Vec<Self::Item>);
    fn get(&self, row: usize, col: usize) -> Option<Self::Item>;
}

pub struct InfiniteRightGrid<T> {
    width: usize,
    height: usize,
    inner: Vec<Vec<T>>
}

impl<T: Debug + Clone> GridTrait for InfiniteRightGrid<T> {
    type Item = T;

    fn fixed(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            inner: Vec::with_capacity(height),
        }
    }

    fn insert_row(&mut self, row: &Vec<Self::Item>) {
        self.inner.push(row.to_vec());
    }

    fn get(&self, row: usize, col: usize) -> Option<Self::Item> {
        let the_row: &Vec<T> = self.inner.get(row)?;

        let col = col % self.width;
        the_row.get(col).cloned()
    }
}


/// converts multiline intput into a grid
///
/// inconvenient to use
///
/// `T` should be an enum
/// `G<T>` is grid type of `T`s
pub fn input_to_grid<T: FromStr, G: GridTrait<Item=T>>(input: &str) -> G {
    let lines: Vec<String> = input.split('\n').map(|s| s.to_string()).collect();

    let width = lines[0].len();
    let height = lines.len();

    let mut grid = G::fixed(width, height);
    for line in lines {
        let row: Vec<T> =
            line
            .chars()
            .filter_map(|ch| ch.to_string().parse::<T>().ok())
            .collect();
        grid.insert_row(&row);
    }

    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid() {
        let input = "123
456";
        let g: InfiniteRightGrid<usize> = input_to_grid(input);
        assert_eq!(g.get(0, 0).unwrap(), 1);
        assert_eq!(g.get(1, 0).unwrap(), 4);
        assert!(g.get(2, 0).is_none());

        assert_eq!(g.get(0, 3).unwrap(), 1);
        assert_eq!(g.get(0, 4).unwrap(), 2);
        assert_eq!(g.get(0, 5).unwrap(), 3);
        assert_eq!(g.get(0, 6).unwrap(), 1);
    }
}
