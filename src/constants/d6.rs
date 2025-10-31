//! Venn diagram for six sets.
//!
//! The diagram itself is [`SIX`], and the premade layout is
//! [`PATHLAYOUT_SIX_OPTIMIZING`].
//!
//! # Example
#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/common/snapshots/render_optimize__common__six.snap.svg"))]

use crate::{
    diagram::DiagramConst,
    direction::DirectedEdge::{Horizontal, Vertical},
    svg::PathLayoutConst,
};

/// A Venn diagram for 6 sets.
///
/// For more info, see [`d6`][super::d6].
pub const SIX: DiagramConst<6, 11, 11> = DiagramConst::from_binary_str(GRIDS);

#[rustfmt::skip]
const GRIDS: [[&str; 11]; 6] = [
    [
        "00000100000",
        "00000100000",
        "00010111000",
        "00010100000",
        "00110111110",
        "01111100000",
        "00000110000",
        "00011110000",
        "00011110000",
        "00000110000",
        "00000000000",
    ],
    [
        "00000000000",
        "00000110000",
        "00001110000",
        "00011111100",
        "00001111100",
        "00111111111",
        "00111010100",
        "00000010000",
        "00000010000",
        "00000000000",
        "00000000000",
    ],
    [
        "00000000000",
        "00000000000",
        "00000100000",
        "00101111000",
        "01111111000",
        "00000111110",
        "00000111000",
        "00111111000",
        "00000101000",
        "00001100000",
        "00000100000",
    ],
    [
        "00000000000",
        "00000010000",
        "00000011100",
        "00000110000",
        "00010100000",
        "11111111100",
        "00111111000",
        "00000101000",
        "00111101000",
        "00011000000",
        "00000000000",
    ],
    [
        "00000000000",
        "00010000000",
        "00011000000",
        "00111000000",
        "00001111111",
        "00011111000",
        "00011101100",
        "00111101000",
        "00111000000",
        "00010000000",
        "00000000000",
    ],
    [
        "00000000000",
        "00010010000",
        "00010011100",
        "00110011100",
        "01111010000",
        "00001110000",
        "00001111100",
        "00001010000",
        "00001010000",
        "00011010000",
        "00000010000",
    ],
];

/// A layout for [`SIX`], optimized to minimize gaps and overlaps.
#[rustfmt::skip]
pub const PATHLAYOUT_SIX_OPTIMIZING: PathLayoutConst<184, 6, 11, 11> = PathLayoutConst {
    combined_paths: [Vertical { x: 1, y_from: 6, y_to: 5 }, Horizontal { y: 5, x_from: 1, x_to: 2 }, Vertical { x: 2, y_from: 5, y_to: 4 }, Horizontal { y: 4, x_from: 2, x_to: 3 }, Vertical { x: 3, y_from: 4, y_to: 2 }, Horizontal { y: 2, x_from: 3, x_to: 4 }, Vertical { x: 4, y_from: 2, y_to: 5 }, Horizontal { y: 5, x_from: 4, x_to: 5 }, Vertical { x: 5, y_from: 5, y_to: 0 }, Horizontal { y: 0, x_from: 5, x_to: 6 }, Vertical { x: 6, y_from: 0, y_to: 2 }, Horizontal { y: 2, x_from: 6, x_to: 8 }, Vertical { x: 8, y_from: 2, y_to: 3 }, Horizontal { y: 3, x_from: 8, x_to: 6 }, Vertical { x: 6, y_from: 3, y_to: 4 }, Horizontal { y: 4, x_from: 6, x_to: 10 }, Vertical { x: 10, y_from: 4, y_to: 5 }, Horizontal { y: 5, x_from: 10, x_to: 6 }, Vertical { x: 6, y_from: 5, y_to: 6 }, Horizontal { y: 6, x_from: 6, x_to: 7 }, Vertical { x: 7, y_from: 6, y_to: 10 }, Horizontal { y: 10, x_from: 7, x_to: 5 }, Vertical { x: 5, y_from: 10, y_to: 9 }, Horizontal { y: 9, x_from: 5, x_to: 3 }, Vertical { x: 3, y_from: 9, y_to: 7 }, Horizontal { y: 7, x_from: 3, x_to: 5 }, Vertical { x: 5, y_from: 7, y_to: 6 }, Horizontal { y: 6, x_from: 5, x_to: 1 }, Vertical { x: 2, y_from: 7, y_to: 5 }, Horizontal { y: 5, x_from: 2, x_to: 4 }, Vertical { x: 4, y_from: 5, y_to: 4 }, Horizontal { y: 4, x_from: 4, x_to: 3 }, Vertical { x: 3, y_from: 4, y_to: 3 }, Horizontal { y: 3, x_from: 3, x_to: 4 }, Vertical { x: 4, y_from: 3, y_to: 2 }, Horizontal { y: 2, x_from: 4, x_to: 5 }, Vertical { x: 5, y_from: 2, y_to: 1 }, Horizontal { y: 1, x_from: 5, x_to: 7 }, Vertical { x: 7, y_from: 1, y_to: 3 }, Horizontal { y: 3, x_from: 7, x_to: 9 }, Vertical { x: 9, y_from: 3, y_to: 5 }, Horizontal { y: 5, x_from: 9, x_to: 11 }, Vertical { x: 11, y_from: 5, y_to: 6 }, Horizontal { y: 6, x_from: 11, x_to: 9 }, Vertical { x: 9, y_from: 6, y_to: 7 }, Horizontal { y: 7, x_from: 9, x_to: 8 }, Vertical { x: 8, y_from: 7, y_to: 6 }, Horizontal { y: 6, x_from: 8, x_to: 7 }, Vertical { x: 7, y_from: 6, y_to: 9 }, Horizontal { y: 9, x_from: 7, x_to: 6 }, Vertical { x: 6, y_from: 9, y_to: 6 }, Horizontal { y: 6, x_from: 6, x_to: 5 }, Vertical { x: 5, y_from: 6, y_to: 7 }, Horizontal { y: 7, x_from: 5, x_to: 2 }, Vertical { x: 1, y_from: 5, y_to: 4 }, Horizontal { y: 4, x_from: 1, x_to: 2 }, Vertical { x: 2, y_from: 4, y_to: 3 }, Horizontal { y: 3, x_from: 2, x_to: 3 }, Vertical { x: 3, y_from: 3, y_to: 4 }, Horizontal { y: 4, x_from: 3, x_to: 4 }, Vertical { x: 4, y_from: 4, y_to: 3 }, Horizontal { y: 3, x_from: 4, x_to: 5 }, Vertical { x: 5, y_from: 3, y_to: 2 }, Horizontal { y: 2, x_from: 5, x_to: 6 }, Vertical { x: 6, y_from: 2, y_to: 3 }, Horizontal { y: 3, x_from: 6, x_to: 8 }, Vertical { x: 8, y_from: 3, y_to: 5 }, Horizontal { y: 5, x_from: 8, x_to: 10 }, Vertical { x: 10, y_from: 5, y_to: 6 }, Horizontal { y: 6, x_from: 10, x_to: 8 }, Vertical { x: 8, y_from: 6, y_to: 9 }, Horizontal { y: 9, x_from: 8, x_to: 7 }, Vertical { x: 7, y_from: 9, y_to: 8 }, Horizontal { y: 8, x_from: 7, x_to: 6 }, Vertical { x: 6, y_from: 8, y_to: 11 }, Horizontal { y: 11, x_from: 6, x_to: 5 }, Vertical { x: 5, y_from: 11, y_to: 10 }, Horizontal { y: 10, x_from: 5, x_to: 4 }, Vertical { x: 4, y_from: 10, y_to: 9 }, Horizontal { y: 9, x_from: 4, x_to: 5 }, Vertical { x: 5, y_from: 9, y_to: 8 }, Horizontal { y: 8, x_from: 5, x_to: 2 }, Vertical { x: 2, y_from: 8, y_to: 7 }, Horizontal { y: 7, x_from: 2, x_to: 5 }, Vertical { x: 5, y_from: 7, y_to: 5 }, Horizontal { y: 5, x_from: 5, x_to: 1 }, Vertical { x: 0, y_from: 6, y_to: 5 }, Horizontal { y: 5, x_from: 0, x_to: 3 }, Vertical { x: 3, y_from: 5, y_to: 4 }, Horizontal { y: 4, x_from: 3, x_to: 4 }, Vertical { x: 4, y_from: 4, y_to: 5 }, Horizontal { y: 5, x_from: 4, x_to: 5 }, Vertical { x: 5, y_from: 5, y_to: 3 }, Horizontal { y: 3, x_from: 5, x_to: 6 }, Vertical { x: 6, y_from: 3, y_to: 1 }, Horizontal { y: 1, x_from: 6, x_to: 7 }, Vertical { x: 7, y_from: 1, y_to: 2 }, Horizontal { y: 2, x_from: 7, x_to: 9 }, Vertical { x: 9, y_from: 2, y_to: 3 }, Horizontal { y: 3, x_from: 9, x_to: 7 }, Vertical { x: 7, y_from: 3, y_to: 4 }, Horizontal { y: 4, x_from: 7, x_to: 6 }, Vertical { x: 6, y_from: 4, y_to: 5 }, Horizontal { y: 5, x_from: 6, x_to: 9 }, Vertical { x: 9, y_from: 5, y_to: 6 }, Horizontal { y: 6, x_from: 9, x_to: 8 }, Vertical { x: 8, y_from: 6, y_to: 9 }, Horizontal { y: 9, x_from: 8, x_to: 7 }, Vertical { x: 7, y_from: 9, y_to: 7 }, Horizontal { y: 7, x_from: 7, x_to: 6 }, Vertical { x: 6, y_from: 7, y_to: 9 }, Horizontal { y: 9, x_from: 6, x_to: 5 }, Vertical { x: 5, y_from: 9, y_to: 10 }, Horizontal { y: 10, x_from: 5, x_to: 3 }, Vertical { x: 3, y_from: 10, y_to: 9 }, Horizontal { y: 9, x_from: 3, x_to: 2 }, Vertical { x: 2, y_from: 9, y_to: 8 }, Horizontal { y: 8, x_from: 2, x_to: 5 }, Vertical { x: 5, y_from: 8, y_to: 7 }, Horizontal { y: 7, x_from: 5, x_to: 2 }, Vertical { x: 2, y_from: 7, y_to: 6 }, Horizontal { y: 6, x_from: 2, x_to: 0 }, Vertical { x: 2, y_from: 4, y_to: 3 }, Horizontal { y: 3, x_from: 2, x_to: 3 }, Vertical { x: 3, y_from: 3, y_to: 1 }, Horizontal { y: 1, x_from: 3, x_to: 4 }, Vertical { x: 4, y_from: 1, y_to: 2 }, Horizontal { y: 2, x_from: 4, x_to: 5 }, Vertical { x: 5, y_from: 2, y_to: 4 }, Horizontal { y: 4, x_from: 5, x_to: 11 }, Vertical { x: 11, y_from: 4, y_to: 5 }, Horizontal { y: 5, x_from: 11, x_to: 8 }, Vertical { x: 8, y_from: 5, y_to: 6 }, Horizontal { y: 6, x_from: 8, x_to: 9 }, Vertical { x: 9, y_from: 6, y_to: 7 }, Horizontal { y: 7, x_from: 9, x_to: 8 }, Vertical { x: 8, y_from: 7, y_to: 8 }, Horizontal { y: 8, x_from: 8, x_to: 7 }, Vertical { x: 7, y_from: 8, y_to: 6 }, Horizontal { y: 6, x_from: 7, x_to: 6 }, Vertical { x: 6, y_from: 6, y_to: 8 }, Horizontal { y: 8, x_from: 6, x_to: 5 }, Vertical { x: 5, y_from: 8, y_to: 9 }, Horizontal { y: 9, x_from: 5, x_to: 4 }, Vertical { x: 4, y_from: 9, y_to: 10 }, Horizontal { y: 10, x_from: 4, x_to: 3 }, Vertical { x: 3, y_from: 10, y_to: 9 }, Horizontal { y: 9, x_from: 3, x_to: 2 }, Vertical { x: 2, y_from: 9, y_to: 7 }, Horizontal { y: 7, x_from: 2, x_to: 3 }, Vertical { x: 3, y_from: 7, y_to: 5 }, Horizontal { y: 5, x_from: 3, x_to: 4 }, Vertical { x: 4, y_from: 5, y_to: 4 }, Horizontal { y: 4, x_from: 4, x_to: 2 }, Vertical { x: 1, y_from: 5, y_to: 4 }, Horizontal { y: 4, x_from: 1, x_to: 2 }, Vertical { x: 2, y_from: 4, y_to: 3 }, Horizontal { y: 3, x_from: 2, x_to: 3 }, Vertical { x: 3, y_from: 3, y_to: 1 }, Horizontal { y: 1, x_from: 3, x_to: 4 }, Vertical { x: 4, y_from: 1, y_to: 4 }, Horizontal { y: 4, x_from: 4, x_to: 5 }, Vertical { x: 5, y_from: 4, y_to: 5 }, Horizontal { y: 5, x_from: 5, x_to: 6 }, Vertical { x: 6, y_from: 5, y_to: 1 }, Horizontal { y: 1, x_from: 6, x_to: 7 }, Vertical { x: 7, y_from: 1, y_to: 2 }, Horizontal { y: 2, x_from: 7, x_to: 9 }, Vertical { x: 9, y_from: 2, y_to: 4 }, Horizontal { y: 4, x_from: 9, x_to: 7 }, Vertical { x: 7, y_from: 4, y_to: 6 }, Horizontal { y: 6, x_from: 7, x_to: 9 }, Vertical { x: 9, y_from: 6, y_to: 7 }, Horizontal { y: 7, x_from: 9, x_to: 7 }, Vertical { x: 7, y_from: 7, y_to: 11 }, Horizontal { y: 11, x_from: 7, x_to: 6 }, Vertical { x: 6, y_from: 11, y_to: 7 }, Horizontal { y: 7, x_from: 6, x_to: 5 }, Vertical { x: 5, y_from: 7, y_to: 10 }, Horizontal { y: 10, x_from: 5, x_to: 3 }, Vertical { x: 3, y_from: 10, y_to: 9 }, Horizontal { y: 9, x_from: 3, x_to: 4 }, Vertical { x: 4, y_from: 9, y_to: 5 }, Horizontal { y: 5, x_from: 4, x_to: 1 }],
    offsets: [0, 2, 0, 0, 1, 0, 1, -2, 0, 0, 1, 1, 0, -2, 1, -1, 0, 1, 0, 0, 0, 0, 1, 0, 0, -1, -1, 0, 0, 2, -1, 0, 2, 0, 2, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, -1, 1, -1, 0, -1, 0, 1, -2, 1, 0, 1, 1, 0, -2, -1, 0, 1, 0, 1, -1, 0, 2, 0, 1, 1, 0, -2, 0, 0, 0, 0, 1, 0, -1, -1, 0, 1, 0, 0, 0, 0, -1, 0, -1, 0, -1, 1, 0, -1, -1, 1, -1, 0, 1, 0, 1, 1, 0, 0, -1, 0, -1, 2, 0, -2, 0, 2, 0, 0, -1, 1, 1, -1, 1, -1, 1, 0, 0, 0, 0, -1, 1, -1, 0, 0, -1, 0, -2, 0, 0, -1, 0, -2, 1, 0, 0, 1, 1, 1, 1, -1, 0, 0, -1, 0, -1, -2, 1, 0, -1, -1, -1, -1, -1, 0, 0, 2, 0, 0, 1, -1, 0, -1, 1, 0, 0, -1, -1, 1, 0, 1, 0, 0, -1, 1, 1, 0, 1],
    parts_len: [28, 26, 32, 36, 32, 30],
    diagram: SIX,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_complete() {
        assert!(SIX.complete());
    }
}
