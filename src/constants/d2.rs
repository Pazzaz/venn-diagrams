use crate::{
    direction::DirectedEdge::{Horizontal, Vertical},
    svg::PathLayoutConst,
    venn_diagram::ConstVennDiagram,
};

/// A Venn diagram for 2 groups.
///
/// # Example
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/common/snapshots/render_optimize__common__two.snap.svg"))]
pub const TWO: ConstVennDiagram<2, 3, 1> = ConstVennDiagram::from_letters(STR);

#[rustfmt::skip]
const STR: [[&str; 3]; 1] = [
    ["A",  "AB", "B",],
];

#[rustfmt::skip]
pub const PATHLAYOUT_TWO_OPTIMIZING: PathLayoutConst<8, 2, 3, 1> = PathLayoutConst {
    combined_paths: [Vertical { x: 0, y_from: 1, y_to: 0 }, Horizontal { y: 0, x_from: 0, x_to: 2 }, Vertical { x: 2, y_from: 0, y_to: 1 }, Horizontal { y: 1, x_from: 2, x_to: 0 }, Vertical { x: 1, y_from: 1, y_to: 0 }, Horizontal { y: 0, x_from: 1, x_to: 3 }, Vertical { x: 3, y_from: 0, y_to: 1 }, Horizontal { y: 1, x_from: 3, x_to: 1 }],
    offsets: [0, -1, 0, -1, 0, 0, 0, 0],
    parts_len: [4, 4],
    diagram: TWO,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_complete() {
        assert!(TWO.complete());
    }
}
