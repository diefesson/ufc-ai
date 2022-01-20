pub trait SplitAtRatio {
    fn split_at_ratio(&self, ratio: f64) -> (&Self, &Self);
}

impl<T> SplitAtRatio for [T] {
    fn split_at_ratio(&self, ratio: f64) -> (&Self, &Self) {
        let mid = (self.len() as f64 * ratio) as usize;
        self.split_at(mid)
    }
}
