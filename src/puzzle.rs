use super::ai;

use ai::node::State;

pub enum MoveDirection {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

pub use MoveDirection::*;

impl MoveDirection {
    pub fn ij(&self) -> (isize, isize) {
        match self {
            UP => (-1, 0),
            DOWN => (1, 0),
            LEFT => (0, -1),
            RIGHT => (0, 1),
        }
    }
}

#[derive(Eq, Clone, Hash, Debug)]
pub struct PuzzleState<const S: usize> {
    i: usize,
    j: usize,
    numbers: [[i32; S]; S],
}

/* impl PuzzleState<4> {
    pub fn new() -> Self {
        Self::new_with_numbers([[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12], [13, 14, 15, 0]])
    }
} */

impl<const S: usize> PuzzleState<S> {
    pub fn new() -> Self {
        let mut numbers = [[0; S]; S];
        for i in 0..S {
            for j in 0..S {
                numbers[i][j] = (i * S + j) as i32;
            }
        }
        numbers[S - 1][S - 1] = 0;
        Self {
            i: S - 1,
            j: S - 1,
            numbers,
        }
    }

    pub fn new_with_numbers(numbers: [[i32; S]; S]) -> PuzzleState<S> {
        let zero_position = find_number(0, &numbers);
        match zero_position {
            Some((i, j)) => return Self { i, j, numbers },
            None => panic!("missing 0"),
        }
    }

    pub fn move_tile(&self, move_direction: MoveDirection) -> Option<PuzzleState<S>> {
        let (di, dj) = move_direction.ij();
        let (i, j) = (self.i as isize + di, self.j as isize + dj);
        if Self::valid_pos(i, j) {
            let (i, j) = (i as usize, j as usize);
            let mut moved = Self {
                numbers: self.numbers.clone(),
                i: i as usize,
                j: j as usize,
            };
            moved.numbers[i][j] = self.numbers[self.i][self.j];
            moved.numbers[self.i][self.j] = self.numbers[i][j];
            return Some(moved);
        } else {
            None
        }
    }

    pub fn valid_pos(i: isize, j: isize) -> bool {
        i >= 0 && j >= 0 && i < S as isize && j < S as isize
    }
}

impl<const S: usize> PartialEq for PuzzleState<S> {
    fn eq(&self, other: &Self) -> bool {
        self.numbers == other.numbers
    }
}

impl<const S: usize> State for PuzzleState<S> {}

pub fn find_number<const S: usize>(number: i32, numbers: &[[i32; S]; S]) -> Option<(usize, usize)> {
    for (i, row) in numbers.iter().enumerate() {
        for (j, n) in row.iter().enumerate() {
            if *n == number {
                return Some((i, j));
            }
        }
    }
    return None;
}

pub fn puzzle_state_expander<const S: usize>(state: &PuzzleState<S>) -> Vec<PuzzleState<S>> {
    let options = vec![
        state.move_tile(UP),
        state.move_tile(DOWN),
        state.move_tile(LEFT),
        state.move_tile(RIGHT),
    ];
    options
        .into_iter()
        .filter(|s| s.is_some())
        .map(|s| s.unwrap())
        .collect()
}
