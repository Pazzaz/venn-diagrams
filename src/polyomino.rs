//! ## Polyominos
//!
//! A Polyomino is a collection of cells on a square grid.
//!
//! There are two versions:
//! - [`Polyomino`], dynamic version
//! - [`ConstPolyomino`], static version

use std::ops::{Index, IndexMut};

/// A statically allocated polyomino.
///
/// It has two generic parameters:
/// - `X`, maximum width of the polyomino
/// - `Y`, maximum height of the polyomino
#[derive(Debug, Clone, Copy)]
pub struct ConstPolyomino<const X: usize, const Y: usize> {
    pub(crate) values: [[bool; X]; Y],
}

impl<const X: usize, const Y: usize> ConstPolyomino<X, Y> {
    /// Create an empty polyomino of width `X` and height `Y`.
    #[must_use]
    pub const fn empty() -> Self {
        Self { values: [[false; X]; Y] }
    }

    #[must_use]
    pub const fn from_binary_str(grid: [&str; Y]) -> Self {
        let mut out = Self::empty();

        let mut y = 0;
        while y != Y {
            let row = grid[y].as_bytes();
            let mut x = 0;
            while x != X {
                if row[x] == b'1' {
                    out.values[y][x] = true;
                }
                x += 1;
            }
            y += 1;
        }
        out
    }
}

impl<const X: usize, const Y: usize> Index<(usize, usize)> for ConstPolyomino<X, Y> {
    type Output = bool;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.values[y][x]
    }
}

impl<const X: usize, const Y: usize> IndexMut<(usize, usize)> for ConstPolyomino<X, Y> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.values[y][x]
    }
}

/// A dynamically allocated polyomino.
#[derive(Debug, PartialEq, Eq)]
pub struct Polyomino {
    width: usize,
    height: usize,
    values: Vec<bool>,
}

impl Polyomino {
    /// Maximum width of the polyomino.
    #[must_use]
    pub fn width(&self) -> usize {
        self.width
    }

    /// Maximum height of the polyomino.
    #[must_use]
    pub fn height(&self) -> usize {
        self.height
    }

    /// Create an empty polyomino of width `x` and height `y`.
    #[must_use]
    pub fn empty(x: usize, y: usize) -> Self {
        Self { width: x, height: y, values: vec![false; x * y] }
    }
}

impl Clone for Polyomino {
    fn clone(&self) -> Self {
        Self { width: self.width, height: self.height, values: self.values.clone() }
    }

    fn clone_from(&mut self, source: &Self) {
        self.width = source.width;
        self.height = source.height;
        self.values.clone_from(&source.values);
    }
}

impl Index<(usize, usize)> for Polyomino {
    type Output = bool;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.values[y * self.width + x]
    }
}

impl IndexMut<(usize, usize)> for Polyomino {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.values[y * self.width + x]
    }
}

impl<const X: usize, const Y: usize> From<ConstPolyomino<X, Y>> for Polyomino {
    fn from(value: ConstPolyomino<X, Y>) -> Self {
        Self {
            width: X,
            height: Y,
            values: value.values.into_iter().flat_map(IntoIterator::into_iter).collect(),
        }
    }
}
