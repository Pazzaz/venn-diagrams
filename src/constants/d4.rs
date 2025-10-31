//! Venn diagram for four sets.
//!
//! The set itself is [`FOUR`], and the premade layout is
//! [`PATHLAYOUT_FOUR_OPTIMIZING`].

use crate::{
    direction::DirectedEdge::{Horizontal, Vertical},
    svg::PathLayoutConst,
    venn_diagram::ConstVennDiagram,
};

/// A Venn diagram for 4 groups.
///
/// # Example
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/common/snapshots/render_optimize__common__four.snap.svg"))]
pub const FOUR: ConstVennDiagram<4, 5, 5> = ConstVennDiagram::from_letters(STR);

#[rustfmt::skip]
const STR: [[&str; 5]; 5] = [
    ["",   "A",   "",     "B",   "",  ],
    ["",   "AC",  "ABC",  "BC",  "C", ],
    ["CD", "ACD", "ABCD", "BCD", "",  ],
    ["",   "AD",  "ABD",  "BD",  "D", ],
    ["",   "",    "AB",   "",    "",  ],
];

/// A layout for [`FOUR`], optimized to minimize gaps and overlaps.
#[rustfmt::skip]
pub const PATHLAYOUT_FOUR_OPTIMIZING: PathLayoutConst<32, 4, 5, 5> = PathLayoutConst {
    combined_paths: [Vertical { x: 1, y_from: 4, y_to: 0 }, Horizontal { y: 0, x_from: 1, x_to: 2 }, Vertical { x: 2, y_from: 0, y_to: 1 }, Horizontal { y: 1, x_from: 2, x_to: 3 }, Vertical { x: 3, y_from: 1, y_to: 5 }, Horizontal { y: 5, x_from: 3, x_to: 2 }, Vertical { x: 2, y_from: 5, y_to: 4 }, Horizontal { y: 4, x_from: 2, x_to: 1 }, Vertical { x: 2, y_from: 5, y_to: 1 }, Horizontal { y: 1, x_from: 2, x_to: 3 }, Vertical { x: 3, y_from: 1, y_to: 0 }, Horizontal { y: 0, x_from: 3, x_to: 4 }, Vertical { x: 4, y_from: 0, y_to: 4 }, Horizontal { y: 4, x_from: 4, x_to: 3 }, Vertical { x: 3, y_from: 4, y_to: 5 }, Horizontal { y: 5, x_from: 3, x_to: 2 }, Vertical { x: 0, y_from: 3, y_to: 2 }, Horizontal { y: 2, x_from: 0, x_to: 1 }, Vertical { x: 1, y_from: 2, y_to: 1 }, Horizontal { y: 1, x_from: 1, x_to: 5 }, Vertical { x: 5, y_from: 1, y_to: 2 }, Horizontal { y: 2, x_from: 5, x_to: 4 }, Vertical { x: 4, y_from: 2, y_to: 3 }, Horizontal { y: 3, x_from: 4, x_to: 0 }, Vertical { x: 0, y_from: 3, y_to: 2 }, Horizontal { y: 2, x_from: 0, x_to: 4 }, Vertical { x: 4, y_from: 2, y_to: 3 }, Horizontal { y: 3, x_from: 4, x_to: 5 }, Vertical { x: 5, y_from: 3, y_to: 4 }, Horizontal { y: 4, x_from: 5, x_to: 1 }, Vertical { x: 1, y_from: 4, y_to: 3 }, Horizontal { y: 3, x_from: 1, x_to: 0 }],
    offsets: [0, 0, 0, -1, 0, -1, 1, -1, 0, 1, 0, 0, 0, 1, 1, 0, 0, -1, -1, 0, 0, 0, -1, 0, 1, 0, 1, 0, 0, 0, -1, -1],
    parts_len: [8, 8, 8, 8],
    diagram: FOUR,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_complete() {
        assert!(FOUR.complete());
    }
}
