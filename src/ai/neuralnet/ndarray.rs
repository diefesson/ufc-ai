use std::iter::Iterator;
use std::ops::{Index, IndexMut};
use std::slice::{Iter, IterMut};

use crate::neuralnet::ShapeSize;

pub struct NDArray<T> {
    shape: Vec<usize>,
    data: Vec<T>,
}

impl<T> NDArray<T> {
    pub fn with_data(data: Vec<T>) -> Self {
        Self::with_shape_and_data(vec![data.len()], data)
    }

    pub fn with_shape_and_data(shape: Vec<usize>, data: Vec<T>) -> Self {
        assert_eq!(shape.size(), data.len(), "shape and data size mismatch");
        Self {
            shape: shape,
            data: data,
        }
    }

    pub fn get_shape(&self) -> &[usize] {
        &self.shape
    }

    pub fn get_data(&self) -> &[T] {
        &self.data
    }

    pub fn reshape(self, new_shape: &[usize]) -> Self {
        assert_eq!(
            self.shape.size(),
            new_shape.size(),
            "new_shape size not matches"
        );
        Self {
            shape: new_shape.into(),
            data: self.data,
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn iter(&self) -> Iter<T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.data.iter_mut()
    }
}

impl<T: Default> NDArray<T> {
    pub fn with_shape(shape: &[usize]) -> Self {
        Self {
            shape: shape.into(),
            data: (0..shape.size()).map(|_| Default::default()).collect(),
        }
    }
}

impl<T> From<Vec<T>> for NDArray<T> {
    fn from(data: Vec<T>) -> Self {
        Self::with_data(data)
    }
}

impl<T, const S: usize> Index<[usize; S]> for NDArray<T> {
    type Output = T;
    fn index(&self, index: [usize; S]) -> &Self::Output {
        &self[&index[..]]
    }
}

impl<T, const S: usize> IndexMut<[usize; S]> for NDArray<T> {
    fn index_mut(&mut self, index: [usize; S]) -> &mut Self::Output {
        &mut self[&index[..]]
    }
}

impl<T> Index<&[usize]> for NDArray<T> {
    type Output = T;
    fn index(&self, index: &[usize]) -> &Self::Output {
        let ri = self.shape.solve_index(index);
        &self.data[ri]
    }
}

impl<T> IndexMut<&[usize]> for NDArray<T> {
    fn index_mut(&mut self, index: &[usize]) -> &mut Self::Output {
        let ri = self.shape.solve_index(index);
        &mut self.data[ri]
    }
}

impl<T> Index<usize> for NDArray<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<usize> for NDArray<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
