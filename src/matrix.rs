use std::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
pub(crate) struct Matrix<T: Clone> {
    x: usize,
    // y: usize,
    values: Vec<T>,
}

impl<T: Clone> Matrix<T> {
    pub fn new(x: usize, y: usize, default: T) -> Self {
        Self {
            x,
            // y,
            values: vec![default; x * y],
        }
    }

    pub fn row(&self, i: usize) -> &[T] {
        &self.values[i * self.x..((i + 1) * self.x)]
    }
}

impl<T: Clone> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.values[y * self.x + x]
    }
}

impl<T: Clone> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.values[y * self.x + x]
    }
}
