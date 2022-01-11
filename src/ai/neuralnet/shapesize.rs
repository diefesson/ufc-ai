pub trait ShapeSize {
    fn size(&self) -> usize;
    fn solve_index(&self, index: &[usize]) -> usize;
}

impl ShapeSize for [usize] {
    fn size(&self) -> usize {
        if self.is_empty() {
            0
        } else {
            self.iter().product()
        }
    }

    fn solve_index(&self, index: &[usize]) -> usize {
        assert_eq!(self.len(), index.len(), "shape and index len not matches");
        let mut dimension_size = 1;
        let mut ri = 0;
        for i in (0..self.len()).rev() {
            ri += index[i] * dimension_size;
            dimension_size *= self[i];
        }
        ri
    }
}
