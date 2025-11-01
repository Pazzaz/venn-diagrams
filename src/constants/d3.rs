//! Venn diagram for three sets.
//!
//! The diagram itself is [`THREE`], and the premade layout is
//! [`LAYOUT_OPTIMIZED_THREE`].
//!
//! # Example
//! <div align="center">
//!
//! ![Three sets][example]
//!
//! </div>
//!
//! # Source
//! Sourced from [\[2006\]](crate#ref-2006).
#![doc = embed_doc_image::embed_image!("example", "tests/common/snapshots/render_optimize__common__three.snap.svg")]
use crate::{
    diagram::DiagramConst,
    direction::DirectedEdge::{Horizontal, Vertical},
    svg::LayoutConst,
};

/// A Venn diagram for 3 sets.
///
/// For more info, see [`d3`][super::d3].
pub const THREE: DiagramConst<3, 5, 2> = DiagramConst::from_letters(STR);

#[rustfmt::skip]
const STR: [[&str; 5]; 2] = [
    ["A", "AB", "ABC", "B",  "",  ],
    ["",  "",   "AC",  "BC", "C", ],
];

/// A layout for [`THREE`], optimized to minimize gaps and overlaps.
#[rustfmt::skip]
pub const LAYOUT_OPTIMIZED_THREE: LayoutConst<18, 3, 5, 2> = LayoutConst {
    combined_paths: [Vertical { x: 0, y_from: 1, y_to: 0 }, Horizontal { y: 0, x_from: 0, x_to: 3 }, Vertical { x: 3, y_from: 0, y_to: 2 }, Horizontal { y: 2, x_from: 3, x_to: 2 }, Vertical { x: 2, y_from: 2, y_to: 1 }, Horizontal { y: 1, x_from: 2, x_to: 0 }, Vertical { x: 1, y_from: 1, y_to: 0 }, Horizontal { y: 0, x_from: 1, x_to: 4 }, Vertical { x: 4, y_from: 0, y_to: 2 }, Horizontal { y: 2, x_from: 4, x_to: 3 }, Vertical { x: 3, y_from: 2, y_to: 1 }, Horizontal { y: 1, x_from: 3, x_to: 1 }, Vertical { x: 2, y_from: 2, y_to: 0 }, Horizontal { y: 0, x_from: 2, x_to: 3 }, Vertical { x: 3, y_from: 0, y_to: 1 }, Horizontal { y: 1, x_from: 3, x_to: 5 }, Vertical { x: 5, y_from: 1, y_to: 2 }, Horizontal { y: 2, x_from: 5, x_to: 2 }],
    offsets: [0, -1, 0, -1, 1, 0, 0, 0, 0, -1, 1, -1, 0, 1, -1, 0, 0, 0],
    parts_len: [6, 6, 6],
    diagram: THREE,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_complete() {
        assert!(THREE.complete());
    }
}
