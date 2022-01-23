#[derive(Clone, Debug)]
pub struct Node {
    pub distance: f64,
    pub heuristic: f64,
}

impl Node {
    pub fn total(&self) -> f64 {
        self.distance + self.heuristic
    }
}
