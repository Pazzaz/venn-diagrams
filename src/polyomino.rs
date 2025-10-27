use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy)]
pub struct ConstPolyomino<const X: usize, const Y: usize> {
    pub(crate) values: [[bool; X]; Y],
}

impl<const X: usize, const Y: usize> ConstPolyomino<X, Y> {
    #[must_use]
    pub const fn empty() -> Self {
        Self { values: [[false; X]; Y] }
    }

    #[must_use]
    pub const fn from_binary_str(grid: [&str; Y]) -> ConstPolyomino<X, Y> {
        let mut out = ConstPolyomino::empty();

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

#[derive(Debug)]
pub struct Polyomino {
    x: usize,
    y: usize,
    values: Vec<bool>,
}

impl Polyomino {
    #[must_use]
    pub fn x(&self) -> usize {
        self.x
    }

    #[must_use]
    pub fn y(&self) -> usize {
        self.y
    }

    #[must_use]
    pub fn empty(x: usize, y: usize) -> Self {
        Self { x, y, values: vec![false; x * y] }
    }
}

impl Clone for Polyomino {
    fn clone(&self) -> Self {
        Self { x: self.x, y: self.y, values: self.values.clone() }
    }

    fn clone_from(&mut self, source: &Self) {
        self.x = source.x;
        self.y = source.y;
        self.values.clone_from(&source.values);
    }
}

impl Index<(usize, usize)> for Polyomino {
    type Output = bool;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.values[y * self.x + x]
    }
}

impl IndexMut<(usize, usize)> for Polyomino {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.values[y * self.x + x]
    }
}

impl<const X: usize, const Y: usize> From<ConstPolyomino<X, Y>> for Polyomino {
    fn from(value: ConstPolyomino<X, Y>) -> Self {
        Polyomino {
            x: X,
            y: Y,
            values: value.values.into_iter().flat_map(IntoIterator::into_iter).collect(),
        }
    }
}
