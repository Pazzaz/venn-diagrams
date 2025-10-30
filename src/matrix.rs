use std::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
pub struct Matrix<T> {
    // We don't need to store height, because we have the width and total size
    width: usize,
    values: Vec<T>,
}

impl<T: Clone> Matrix<T> {
    pub fn new(width: usize, height: usize, default: T) -> Self {
        Self { width, values: vec![default; width * height] }
    }

    pub fn row(&self, i: usize) -> &[T] {
        &self.values[i * self.width..((i + 1) * self.width)]
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.values[y * self.width + x]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.values[y * self.width + x]
    }
}
