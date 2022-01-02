use std::hash::{Hash, Hasher};

use crate::ai::data::State;
use crate::demo::puzzle::Movement;

#[derive(Eq, Clone, Debug)]
pub struct PuzzleState<const S: usize> {
    i: usize,
    j: usize,
    numbers: [[i32; S]; S],
}

impl<const S: usize> PuzzleState<S> {
    #[allow(clippy::needless_range_loop)]
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

    pub fn with_numbers(numbers: [[i32; S]; S]) -> PuzzleState<S> {
        let zero_position = find_number((S * S) as i32, &numbers);
        match zero_position {
            Some((i, j)) => Self { i, j, numbers },
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

    pub fn move_tile(&self, move_direction: Movement) -> Option<PuzzleState<S>> {
        let (di, dj) = move_direction.ij();
        let (i, j) = (self.i as isize + di, self.j as isize + dj);
        if Self::valid_pos(i, j) {
            let (i, j) = (i as usize, j as usize);
            let mut moved = Self {
                numbers: self.numbers,
                i: i as usize,
                j: j as usize,
            };
            moved.numbers[i][j] = self.numbers[self.i][self.j];
            moved.numbers[self.i][self.j] = self.numbers[i][j];
            Some(moved)
        } else {
            None
        }
    }

    pub fn solveable(&self) -> bool {
        let even_inversions = self.inversion_count() % 2 == 0;
        let even_row = self.i % 2 == 0;
        if S % 2 == 0 {
            if even_inversions {
                !even_row
            } else {
                even_row
            }
        } else {
            even_inversions
        }
    }

    pub fn correct_pos(number: i32) -> (usize, usize) {
        let linear_pos = number - 1;
        let i = (linear_pos / 4) as usize;
        let j = (linear_pos % 4) as usize;
        (i, j)
    }

    #[allow(clippy::collapsible_if)]
    pub fn inversion_count(&self) -> i32 {
        let numbers = self.numbers.iter().flatten().copied().collect::<Vec<_>>();
        let mut inversions = 0;
        for i in 0..numbers.len() {
            for j in i..numbers.len() {
                if numbers[i] != (S * S) as i32 && numbers[j] != (S * S) as i32 {
                    if numbers[i] > numbers[j] {
                        inversions += 1;
                    }
                }
            }
        }
        inversions
    }

    pub fn valid_pos(i: isize, j: isize) -> bool {
        i >= 0 && j >= 0 && i < S as isize && j < S as isize
    }
}

impl<const S: usize> Hash for PuzzleState<S> {
    fn hash<H>(&self, hasher: &mut H)
    where
        H: Hasher,
    {
        self.numbers.hash(hasher);
    }
}

impl<const S: usize> PartialEq for PuzzleState<S> {
    fn eq(&self, other: &Self) -> bool {
        self.numbers == other.numbers
    }
}

impl<const S: usize> State for PuzzleState<S> {}

impl<const S: usize> Default for PuzzleState<S> {
    fn default() -> Self {
        Self::new()
    }
}

pub fn find_number<const S: usize>(number: i32, numbers: &[[i32; S]; S]) -> Option<(usize, usize)> {
    for (i, row) in numbers.iter().enumerate() {
        for (j, n) in row.iter().enumerate() {
            if *n == number {
                return Some((i, j));
            }
        }
    }
    None
}
