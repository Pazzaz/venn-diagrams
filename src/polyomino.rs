#[derive(Debug, Clone, Copy)]
pub struct ConstPolyomino<const X: usize, const Y: usize> {
    pub(crate) values: [[bool; X]; Y],
}

impl<const X: usize, const Y: usize> ConstPolyomino<X, Y> {
    pub const fn empty() -> Self {
        Self { values: [[false; X]; Y] }
    }

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
