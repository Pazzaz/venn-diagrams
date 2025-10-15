mod direction;

pub mod diagram;
pub mod venn;

#[derive(Debug, Clone, Copy)]
pub struct ConstPolyomino<const X: usize, const Y: usize> {
    values: [[bool; X]; Y],
}

impl<const X: usize, const Y: usize> ConstPolyomino<X, Y> {
    const fn empty() -> Self {
        Self { values: [[false; X]; Y] }
    }
}
