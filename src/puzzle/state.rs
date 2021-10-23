use crate::ai::node::State;

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

impl<const S: usize> PuzzleState<S> {
    pub fn new() -> Self {
        let mut numbers = [[0; S]; S];
        for i in 0..S {
            for j in 0..S {
                numbers[i][j] = (i * S + j + 1) as i32;
            }
        }
        Self {
            i: S - 1,
            j: S - 1,
            numbers,
        }
    }

    pub fn new_with_numbers(numbers: [[i32; S]; S]) -> PuzzleState<S> {
        let zero_position = find_number((S * S) as i32, &numbers);
        match zero_position {
            Some((i, j)) => return Self { i, j, numbers },
            None => panic!("missing final number"),
        }
    }

    pub fn i(&self) -> usize {
        self.i
    }

    pub fn j(&self) -> usize {
        self.j
    }

    pub fn numbers(&self) -> &[[i32; S]; S] {
        &self.numbers
    }

    pub fn solveable(&self) -> bool {
        let even_inversions = self.inversion_count() % 2 == 0;
        if S % 2 == 0 {
            even_inversions
        } else {
            if even_inversions {
                self.i % 2 == 1
            } else {
                self.i % 2 == 0
            }
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

    pub fn correct_pos(number: i32) -> (usize, usize){
        let linear_pos = number - 1;
        let i = linear_pos / 4;
        let j = linear_pos % 4;
        return (i as usize, j as usize); 
    }

    pub fn inversion_count(&self) -> i32 {
        let flattened = self.numbers.iter().flatten().copied().collect::<Vec<_>>();
        let mut inversions = 0;
        for i in 0..flattened.len() {
            for j in i..flattened.len() {
                if flattened[i] > flattened[j] {
                    inversions += 1;
                }
            }
        }
        return inversions;
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
