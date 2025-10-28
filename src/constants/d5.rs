use super::venn_diagram::ConstVennDiagram;
use crate::{
    direction::DirectedEdge::{Horizontal, Vertical},
    svg::PathLayoutConst,
};

/// A Venn diagram for 5 groups.
///
/// # Example
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/common/snapshots/render_optimize__common__five.snap.svg"))]
pub const FIVE: ConstVennDiagram<5, 7, 7> = ConstVennDiagram::from_letters(STR);

#[rustfmt::skip]
const STR: [[&str; 7]; 7] = [
    ["",  "",    "",      "",     "D",    "CD",   "",   ],
    ["E", "BCE", "AE",    "ACDE", "DE",   "CE",   "",   ],
    ["",  "BC",  "ACE",   "ACBE", "CDE",  "BCDE", "",   ],
    ["",  "BDE", "ABCDE", "ABD",  "BCD",  "ABC",  "",   ],
    ["",  "BD",  "ABDE",  "ADE",  "ABCD", "ACD",  "AD", ],
    ["",  "BE",  "ABE",   "",     "AC",   "C",    "A",  ],
    ["",  "B",   "AB",    "",     "",     "",     "",   ],
];

#[rustfmt::skip]
pub const PATHLAYOUT_FIVE_OPTIMIZING: PathLayoutConst<90, 5, 7, 7> = PathLayoutConst {
    combined_paths: [Vertical { x: 2, y_from: 7, y_to: 1 }, Horizontal { y: 1, x_from: 2, x_to: 4 }, Vertical { x: 4, y_from: 1, y_to: 4 }, Horizontal { y: 4, x_from: 4, x_to: 5 }, Vertical { x: 5, y_from: 4, y_to: 3 }, Horizontal { y: 3, x_from: 5, x_to: 6 }, Vertical { x: 6, y_from: 3, y_to: 4 }, Horizontal { y: 4, x_from: 6, x_to: 7 }, Vertical { x: 7, y_from: 4, y_to: 6 }, Horizontal { y: 6, x_from: 7, x_to: 6 }, Vertical { x: 6, y_from: 6, y_to: 5 }, Horizontal { y: 5, x_from: 6, x_to: 5 }, Vertical { x: 5, y_from: 5, y_to: 6 }, Horizontal { y: 6, x_from: 5, x_to: 4 }, Vertical { x: 4, y_from: 6, y_to: 5 }, Horizontal { y: 5, x_from: 4, x_to: 3 }, Vertical { x: 3, y_from: 5, y_to: 7 }, Horizontal { y: 7, x_from: 3, x_to: 2 }, Vertical { x: 1, y_from: 7, y_to: 1 }, Horizontal { y: 1, x_from: 1, x_to: 2 }, Vertical { x: 2, y_from: 1, y_to: 3 }, Horizontal { y: 3, x_from: 2, x_to: 3 }, Vertical { x: 3, y_from: 3, y_to: 2 }, Horizontal { y: 2, x_from: 3, x_to: 4 }, Vertical { x: 4, y_from: 2, y_to: 3 }, Horizontal { y: 3, x_from: 4, x_to: 5 }, Vertical { x: 5, y_from: 3, y_to: 2 }, Horizontal { y: 2, x_from: 5, x_to: 6 }, Vertical { x: 6, y_from: 2, y_to: 4 }, Horizontal { y: 4, x_from: 6, x_to: 5 }, Vertical { x: 5, y_from: 4, y_to: 5 }, Horizontal { y: 5, x_from: 5, x_to: 4 }, Vertical { x: 4, y_from: 5, y_to: 4 }, Horizontal { y: 4, x_from: 4, x_to: 3 }, Vertical { x: 3, y_from: 4, y_to: 7 }, Horizontal { y: 7, x_from: 3, x_to: 1 }, Vertical { x: 1, y_from: 3, y_to: 1 }, Horizontal { y: 1, x_from: 1, x_to: 2 }, Vertical { x: 2, y_from: 1, y_to: 2 }, Horizontal { y: 2, x_from: 2, x_to: 3 }, Vertical { x: 3, y_from: 2, y_to: 1 }, Horizontal { y: 1, x_from: 3, x_to: 4 }, Vertical { x: 4, y_from: 1, y_to: 2 }, Horizontal { y: 2, x_from: 4, x_to: 5 }, Vertical { x: 5, y_from: 2, y_to: 0 }, Horizontal { y: 0, x_from: 5, x_to: 6 }, Vertical { x: 6, y_from: 0, y_to: 6 }, Horizontal { y: 6, x_from: 6, x_to: 4 }, Vertical { x: 4, y_from: 6, y_to: 3 }, Horizontal { y: 3, x_from: 4, x_to: 3 }, Vertical { x: 3, y_from: 3, y_to: 4 }, Horizontal { y: 4, x_from: 3, x_to: 2 }, Vertical { x: 2, y_from: 4, y_to: 3 }, Horizontal { y: 3, x_from: 2, x_to: 1 }, Vertical { x: 1, y_from: 5, y_to: 3 }, Horizontal { y: 3, x_from: 1, x_to: 4 }, Vertical { x: 4, y_from: 3, y_to: 2 }, Horizontal { y: 2, x_from: 4, x_to: 3 }, Vertical { x: 3, y_from: 2, y_to: 1 }, Horizontal { y: 1, x_from: 3, x_to: 4 }, Vertical { x: 4, y_from: 1, y_to: 0 }, Horizontal { y: 0, x_from: 4, x_to: 6 }, Vertical { x: 6, y_from: 0, y_to: 1 }, Horizontal { y: 1, x_from: 6, x_to: 5 }, Vertical { x: 5, y_from: 1, y_to: 2 }, Horizontal { y: 2, x_from: 5, x_to: 6 }, Vertical { x: 6, y_from: 2, y_to: 3 }, Horizontal { y: 3, x_from: 6, x_to: 5 }, Vertical { x: 5, y_from: 3, y_to: 4 }, Horizontal { y: 4, x_from: 5, x_to: 7 }, Vertical { x: 7, y_from: 4, y_to: 5 }, Horizontal { y: 5, x_from: 7, x_to: 1 }, Vertical { x: 0, y_from: 2, y_to: 1 }, Horizontal { y: 1, x_from: 0, x_to: 6 }, Vertical { x: 6, y_from: 1, y_to: 3 }, Horizontal { y: 3, x_from: 6, x_to: 3 }, Vertical { x: 3, y_from: 3, y_to: 4 }, Horizontal { y: 4, x_from: 3, x_to: 4 }, Vertical { x: 4, y_from: 4, y_to: 5 }, Horizontal { y: 5, x_from: 4, x_to: 3 }, Vertical { x: 3, y_from: 5, y_to: 6 }, Horizontal { y: 6, x_from: 3, x_to: 1 }, Vertical { x: 1, y_from: 6, y_to: 5 }, Horizontal { y: 5, x_from: 1, x_to: 2 }, Vertical { x: 2, y_from: 5, y_to: 4 }, Horizontal { y: 4, x_from: 2, x_to: 1 }, Vertical { x: 1, y_from: 4, y_to: 3 }, Horizontal { y: 3, x_from: 1, x_to: 2 }, Vertical { x: 2, y_from: 3, y_to: 2 }, Horizontal { y: 2, x_from: 2, x_to: 0 }],
    offsets: [0, 1, 1, 0, 1, 2, -1, -1, 0, 0, 1, 1, 0, -1, 1, 1, 1, 1, 0, 1, 1, -1, 0, 0, -1, 0, 0, 1, -2, 1, 0, -1, 1, 1, 0, 0, 1, 2, -1, 0, 0, 2, 0, 0, 0, -1, 0, 0, 0, 2, 0, 0, 1, -1, 1, 0, 0, -1, -1, -1, 0, 0, -1, -1, 1, 0, -1, 0, 0, 0, -1, 0, 0, 0, 1, 1, -1, 0, -1, -1, -1, 0, 1, 1, -1, 0, 2, 1, -1, 0],
    parts_len: [18, 18, 18, 18, 18],
    diagram: FIVE,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_complete() {
        assert!(FIVE.complete());
    }
}
