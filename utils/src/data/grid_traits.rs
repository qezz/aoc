// trait InnerAsVec<T> {
//     fn inner_as_vec(self) -> Vec<Vec<T>>;
//     fn inner_mut_ref(&self) -> Vec<Vec<T>>;
// }

pub trait GetCloned<T: Clone>: ImmutableGrid<T> {
    fn get_cloned(&self, row: usize, col: usize) -> Option<T> {
        let aref = self.get(row, col);
        aref.cloned()
    }
}

pub trait ImmutableGrid<T> {
    fn get(&self, row: usize, col: usize) -> Option<&T>;
}

pub trait MutableGrid<T> { // : InnerAsVec<T> {
    fn set(&mut self, row: usize, col: usize, val: T) -> Option<T> {
        if let Some(x) = self.get_mut(row, col) {
            *x = val;
        }

        None
    }
    fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T>;
// }

// pub trait MutableClone<T> {
    fn push_line(&mut self, line: &[T]);
    fn insert_line(&mut self, idx: usize, line: &[T]);
}

pub trait ArbitraryAccess<T> {
    fn extend_rows_with(&mut self, row: usize, new_len: usize, val: T);
}
