use crate::{
    direction::DirectedEdge::{Horizontal, Vertical},
    svg::PathLayoutConst,
    venn_diagram::ConstVennDiagram,
};

/// A Venn diagram for 3 groups.
///
/// # Example
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/common/snapshots/render_optimize__common__three.snap.svg"))]
pub const THREE: ConstVennDiagram<3, 5, 2> = ConstVennDiagram::from_letters(STR);

#[rustfmt::skip]
const STR: [[&str; 5]; 2] = [
    ["A", "AB", "ABC", "B",  "",  ],
    ["",  "",   "AC",  "BC", "C", ],
];

#[rustfmt::skip]
pub const PATHLAYOUT_THREE_OPTIMIZING: PathLayoutConst<18, 3, 5, 2> = PathLayoutConst {
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
