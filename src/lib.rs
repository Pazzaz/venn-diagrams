//! # Venn Diagrams
//!
//! A library to work with [Venn diagrams][wp-venn], specifically those
//! consisting of [polyominos][wp-poly].
//!
//! # Research
//! Parts of this library is based on the following research:
//! - "Minimum Area Venn Diagrams Whose Curves are Polyominoes"; Stirling Chow, Frank Ruskey; [arxiv:0603068](https://arxiv.org/abs/math/0603068)
//! - "Minimum area polyomino Venn diagrams"; Bette Bultena, Matthew Klimesh, Frank Ruskey; [doi:10.20382/jocg.v3i1a8](https://doi.org/10.20382/jocg.v3i1a8)
//!
//! [wp-venn]: https://en.wikipedia.org/wiki/Venn_diagram
//! [wp-poly]: https://en.wikipedia.org/wiki/Polyomino

mod direction;

pub mod diagram;
pub mod polyomino;
pub mod venn;
