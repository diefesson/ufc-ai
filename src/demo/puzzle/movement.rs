pub enum Movement {
    Up,
    Down,
    Left,
    Right,
}

pub use Movement::*;

impl Movement {
    pub fn ij(&self) -> (isize, isize) {
        match self {
            Up => (-1, 0),
            Down => (1, 0),
            Left => (0, -1),
            Right => (0, 1),
        }
    }
}
