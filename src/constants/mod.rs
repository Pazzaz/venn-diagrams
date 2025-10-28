//! ## Venn diagrams
//!
//! A Venn diagram consists of multiple polyominos (see the
//! [polyomino][crate::polyomino] module).
//!
//! There's the static version, [`ConstVennDiagram`], and a dynamic version
//! `TODO`.
//!
//! There are premade Venn diagrams for 2 to 8 groups: [2][TWO], [3][THREE],
//! [4][FOUR], [5][FIVE], [6][SIX], [8][EIGHT].

mod d2;
mod d3;
mod d4;
mod d5;
mod d6;
mod d8;

pub use d2::TWO;
pub use d3::THREE;
pub use d4::FOUR;
pub use d5::FIVE;
pub use d6::SIX;
pub use d8::EIGHT;

mod venn_diagram;
pub use venn_diagram::{ConstVennDiagram, VennDiagram};
