//! Venn diagram for eight sets.
//!
//! The set itself is [`EIGHT`], and the premade layout is
//! [`PATHLAYOUT_EIGHT_OPTIMIZING`].

use crate::{
    diagram::DiagramConst,
    direction::DirectedEdge::{Horizontal, Vertical},
    svg::PathLayoutConst,
};

/// A Venn diagram for 8 groups.
///
/// # Example
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/common/snapshots/render_optimize__common__eight.snap.svg"))]
pub const EIGHT: DiagramConst<8, 17, 15> = DiagramConst::from_binary_str(GRIDS);

#[rustfmt::skip]
const GRIDS: [[&str; 15]; 8] = [
    [
        "00000000111111110",
        "00000000111111110",
        "00000000111111110",
        "00000000111111110",
        "00000000111111110",
        "00000000111111110",
        "00000000111111110",
        "00000000111111111",
        "00000000111111111",
        "00000000111111111",
        "00000000111111111",
        "00000000111111111",
        "00000000111111111",
        "00000000111111111",
        "00000000111111111",
    ],
    [
        "00000000000000000",
        "00000000000000000",
        "00000000000000000",
        "00000000000000000",
        "00000000000000000",
        "00000000000000000",
        "00000000000000000",
        "11111111111111110",
        "11111111111111110",
        "11111111111111110",
        "11111111111111110",
        "11111111111111110",
        "11111111111111110",
        "11111111111111110",
        "11111111111111110",
    ],
    [
        "00001111000000000",
        "00001111000000000",
        "00001111000000000",
        "00001111111111111",
        "00001111111111111",
        "00001111111111111",
        "00001111111111111",
        "11111111111100000",
        "11111111111100000",
        "11111111111100000",
        "11111111111100000",
        "00000000111100000",
        "00000000111100000",
        "00000000111100000",
        "00000000111100000",
    ],
    [
        "00000000111100000",
        "00000000111100000",
        "00000000111100000",
        "11111111111100000",
        "11111111111100000",
        "11111111111100000",
        "11111111111100000",
        "00001111111111111",
        "00001111111111111",
        "00001111111111111",
        "00001111111111111",
        "00001111000000000",
        "00001111000000000",
        "00001111000000000",
        "00001111000000000",
    ],
    [
        "00110000110000000",
        "00111111110011111",
        "00111111110011111",
        "00000011000011000",
        "00000011000011000",
        "11110011111111000",
        "11110011111111000",
        "00111111110011111",
        "00111111110011111",
        "00110000110000000",
        "00110000110000000",
        "11110011111111000",
        "11110011111111000",
        "00000011000011000",
        "00000011000011000",
    ],
    [
        "00000011000011000",
        "11110011111111000",
        "11110011111111000",
        "00110000110000000",
        "00110000110000000",
        "00111111110011111",
        "00111111110011111",
        "11110011111111000",
        "11110011111111000",
        "00000011000011000",
        "00000011000011000",
        "00111111110011111",
        "00111111110011111",
        "00110000110000000",
        "00110000110000000",
    ],
    [
        "01110111101110111",
        "00010001001000100",
        "11011101111011100",
        "01000100100010000",
        "01110111101110111",
        "00010001001000100",
        "11011101111011100",
        "01110111101110111",
        "01000100100010000",
        "11011101111011100",
        "00010001001000100",
        "01110111101110111",
        "01000100100010000",
        "11011101111011100",
        "00010001001000100",
    ],
    [
        "11011101111011100",
        "01000100100010000",
        "01110111101110111",
        "00010001001000100",
        "11011101111011100",
        "01000100100010000",
        "01110111101110111",
        "11011101111011100",
        "00010001001000100",
        "01110111101110111",
        "01000100100010000",
        "11011101111011100",
        "00010001001000100",
        "01110111101110111",
        "01000100100010000",
    ],
];

/// A layout for [`EIGHT`], optimized to minimize gaps and overlaps.
#[rustfmt::skip]
pub const PATHLAYOUT_EIGHT_OPTIMIZING: PathLayoutConst<418, 8, 17, 15> = PathLayoutConst {
    combined_paths: [Vertical { x: 8, y_from: 15, y_to: 0 }, Horizontal { y: 0, x_from: 8, x_to: 16 }, Vertical { x: 16, y_from: 0, y_to: 7 }, Horizontal { y: 7, x_from: 16, x_to: 17 }, Vertical { x: 17, y_from: 7, y_to: 15 }, Horizontal { y: 15, x_from: 17, x_to: 8 }, Vertical { x: 0, y_from: 15, y_to: 7 }, Horizontal { y: 7, x_from: 0, x_to: 16 }, Vertical { x: 16, y_from: 7, y_to: 15 }, Horizontal { y: 15, x_from: 16, x_to: 0 }, Vertical { x: 0, y_from: 11, y_to: 7 }, Horizontal { y: 7, x_from: 0, x_to: 4 }, Vertical { x: 4, y_from: 7, y_to: 0 }, Horizontal { y: 0, x_from: 4, x_to: 8 }, Vertical { x: 8, y_from: 0, y_to: 3 }, Horizontal { y: 3, x_from: 8, x_to: 17 }, Vertical { x: 17, y_from: 3, y_to: 7 }, Horizontal { y: 7, x_from: 17, x_to: 12 }, Vertical { x: 12, y_from: 7, y_to: 15 }, Horizontal { y: 15, x_from: 12, x_to: 8 }, Vertical { x: 8, y_from: 15, y_to: 11 }, Horizontal { y: 11, x_from: 8, x_to: 0 }, Vertical { x: 0, y_from: 7, y_to: 3 }, Horizontal { y: 3, x_from: 0, x_to: 8 }, Vertical { x: 8, y_from: 3, y_to: 0 }, Horizontal { y: 0, x_from: 8, x_to: 12 }, Vertical { x: 12, y_from: 0, y_to: 7 }, Horizontal { y: 7, x_from: 12, x_to: 17 }, Vertical { x: 17, y_from: 7, y_to: 11 }, Horizontal { y: 11, x_from: 17, x_to: 8 }, Vertical { x: 8, y_from: 11, y_to: 15 }, Horizontal { y: 15, x_from: 8, x_to: 4 }, Vertical { x: 4, y_from: 15, y_to: 7 }, Horizontal { y: 7, x_from: 4, x_to: 0 }, Vertical { x: 0, y_from: 7, y_to: 5 }, Horizontal { y: 5, x_from: 0, x_to: 4 }, Vertical { x: 4, y_from: 5, y_to: 7 }, Horizontal { y: 7, x_from: 4, x_to: 6 }, Vertical { x: 6, y_from: 7, y_to: 3 }, Horizontal { y: 3, x_from: 6, x_to: 2 }, Vertical { x: 2, y_from: 3, y_to: 0 }, Horizontal { y: 0, x_from: 2, x_to: 4 }, Vertical { x: 4, y_from: 0, y_to: 1 }, Horizontal { y: 1, x_from: 4, x_to: 8 }, Vertical { x: 8, y_from: 1, y_to: 0 }, Horizontal { y: 0, x_from: 8, x_to: 10 }, Vertical { x: 10, y_from: 0, y_to: 3 }, Horizontal { y: 3, x_from: 10, x_to: 8 }, Vertical { x: 8, y_from: 3, y_to: 5 }, Horizontal { y: 5, x_from: 8, x_to: 12 }, Vertical { x: 12, y_from: 5, y_to: 1 }, Horizontal { y: 1, x_from: 12, x_to: 17 }, Vertical { x: 17, y_from: 1, y_to: 3 }, Horizontal { y: 3, x_from: 17, x_to: 14 }, Vertical { x: 14, y_from: 3, y_to: 7 }, Horizontal { y: 7, x_from: 14, x_to: 17 }, Vertical { x: 17, y_from: 7, y_to: 9 }, Horizontal { y: 9, x_from: 17, x_to: 12 }, Vertical { x: 12, y_from: 9, y_to: 7 }, Horizontal { y: 7, x_from: 12, x_to: 10 }, Vertical { x: 10, y_from: 7, y_to: 11 }, Horizontal { y: 11, x_from: 10, x_to: 14 }, Vertical { x: 14, y_from: 11, y_to: 15 }, Horizontal { y: 15, x_from: 14, x_to: 12 }, Vertical { x: 12, y_from: 15, y_to: 13 }, Horizontal { y: 13, x_from: 12, x_to: 8 }, Vertical { x: 8, y_from: 13, y_to: 15 }, Horizontal { y: 15, x_from: 8, x_to: 6 }, Vertical { x: 6, y_from: 15, y_to: 11 }, Horizontal { y: 11, x_from: 6, x_to: 8 }, Vertical { x: 8, y_from: 11, y_to: 9 }, Horizontal { y: 9, x_from: 8, x_to: 4 }, Vertical { x: 4, y_from: 9, y_to: 13 }, Horizontal { y: 13, x_from: 4, x_to: 0 }, Vertical { x: 0, y_from: 13, y_to: 11 }, Horizontal { y: 11, x_from: 0, x_to: 2 }, Vertical { x: 2, y_from: 11, y_to: 7 }, Horizontal { y: 7, x_from: 2, x_to: 0 }, Vertical { x: 0, y_from: 3, y_to: 1 }, Horizontal { y: 1, x_from: 0, x_to: 4 }, Vertical { x: 4, y_from: 1, y_to: 5 }, Horizontal { y: 5, x_from: 4, x_to: 8 }, Vertical { x: 8, y_from: 5, y_to: 3 }, Horizontal { y: 3, x_from: 8, x_to: 6 }, Vertical { x: 6, y_from: 3, y_to: 0 }, Horizontal { y: 0, x_from: 6, x_to: 8 }, Vertical { x: 8, y_from: 0, y_to: 1 }, Horizontal { y: 1, x_from: 8, x_to: 12 }, Vertical { x: 12, y_from: 1, y_to: 0 }, Horizontal { y: 0, x_from: 12, x_to: 14 }, Vertical { x: 14, y_from: 0, y_to: 3 }, Horizontal { y: 3, x_from: 14, x_to: 10 }, Vertical { x: 10, y_from: 3, y_to: 7 }, Horizontal { y: 7, x_from: 10, x_to: 12 }, Vertical { x: 12, y_from: 7, y_to: 5 }, Horizontal { y: 5, x_from: 12, x_to: 17 }, Vertical { x: 17, y_from: 5, y_to: 7 }, Horizontal { y: 7, x_from: 17, x_to: 14 }, Vertical { x: 14, y_from: 7, y_to: 11 }, Horizontal { y: 11, x_from: 14, x_to: 17 }, Vertical { x: 17, y_from: 11, y_to: 13 }, Horizontal { y: 13, x_from: 17, x_to: 12 }, Vertical { x: 12, y_from: 13, y_to: 9 }, Horizontal { y: 9, x_from: 12, x_to: 8 }, Vertical { x: 8, y_from: 9, y_to: 11 }, Horizontal { y: 11, x_from: 8, x_to: 10 }, Vertical { x: 10, y_from: 11, y_to: 15 }, Horizontal { y: 15, x_from: 10, x_to: 8 }, Vertical { x: 8, y_from: 15, y_to: 13 }, Horizontal { y: 13, x_from: 8, x_to: 4 }, Vertical { x: 4, y_from: 13, y_to: 15 }, Horizontal { y: 15, x_from: 4, x_to: 2 }, Vertical { x: 2, y_from: 15, y_to: 11 }, Horizontal { y: 11, x_from: 2, x_to: 6 }, Vertical { x: 6, y_from: 11, y_to: 7 }, Horizontal { y: 7, x_from: 6, x_to: 4 }, Vertical { x: 4, y_from: 7, y_to: 9 }, Horizontal { y: 9, x_from: 4, x_to: 0 }, Vertical { x: 0, y_from: 9, y_to: 7 }, Horizontal { y: 7, x_from: 0, x_to: 2 }, Vertical { x: 2, y_from: 7, y_to: 3 }, Horizontal { y: 3, x_from: 2, x_to: 0 }, Vertical { x: 0, y_from: 3, y_to: 2 }, Horizontal { y: 2, x_from: 0, x_to: 2 }, Vertical { x: 2, y_from: 2, y_to: 4 }, Horizontal { y: 4, x_from: 2, x_to: 4 }, Vertical { x: 4, y_from: 4, y_to: 6 }, Horizontal { y: 6, x_from: 4, x_to: 6 }, Vertical { x: 6, y_from: 6, y_to: 7 }, Horizontal { y: 7, x_from: 6, x_to: 7 }, Vertical { x: 7, y_from: 7, y_to: 5 }, Horizontal { y: 5, x_from: 7, x_to: 5 }, Vertical { x: 5, y_from: 5, y_to: 3 }, Horizontal { y: 3, x_from: 5, x_to: 3 }, Vertical { x: 3, y_from: 3, y_to: 1 }, Horizontal { y: 1, x_from: 3, x_to: 1 }, Vertical { x: 1, y_from: 1, y_to: 0 }, Horizontal { y: 0, x_from: 1, x_to: 4 }, Vertical { x: 4, y_from: 0, y_to: 2 }, Horizontal { y: 2, x_from: 4, x_to: 6 }, Vertical { x: 6, y_from: 2, y_to: 4 }, Horizontal { y: 4, x_from: 6, x_to: 8 }, Vertical { x: 8, y_from: 4, y_to: 3 }, Horizontal { y: 3, x_from: 8, x_to: 7 }, Vertical { x: 7, y_from: 3, y_to: 1 }, Horizontal { y: 1, x_from: 7, x_to: 5 }, Vertical { x: 5, y_from: 1, y_to: 0 }, Horizontal { y: 0, x_from: 5, x_to: 9 }, Vertical { x: 9, y_from: 0, y_to: 1 }, Horizontal { y: 1, x_from: 9, x_to: 8 }, Vertical { x: 8, y_from: 1, y_to: 2 }, Horizontal { y: 2, x_from: 8, x_to: 10 }, Vertical { x: 10, y_from: 2, y_to: 0 }, Horizontal { y: 0, x_from: 10, x_to: 13 }, Vertical { x: 13, y_from: 0, y_to: 1 }, Horizontal { y: 1, x_from: 13, x_to: 11 }, Vertical { x: 11, y_from: 1, y_to: 3 }, Horizontal { y: 3, x_from: 11, x_to: 9 }, Vertical { x: 9, y_from: 3, y_to: 5 }, Horizontal { y: 5, x_from: 9, x_to: 8 }, Vertical { x: 8, y_from: 5, y_to: 6 }, Horizontal { y: 6, x_from: 8, x_to: 10 }, Vertical { x: 10, y_from: 6, y_to: 4 }, Horizontal { y: 4, x_from: 10, x_to: 12 }, Vertical { x: 12, y_from: 4, y_to: 2 }, Horizontal { y: 2, x_from: 12, x_to: 14 }, Vertical { x: 14, y_from: 2, y_to: 0 }, Horizontal { y: 0, x_from: 14, x_to: 17 }, Vertical { x: 17, y_from: 0, y_to: 1 }, Horizontal { y: 1, x_from: 17, x_to: 15 }, Vertical { x: 15, y_from: 1, y_to: 3 }, Horizontal { y: 3, x_from: 15, x_to: 13 }, Vertical { x: 13, y_from: 3, y_to: 5 }, Horizontal { y: 5, x_from: 13, x_to: 11 }, Vertical { x: 11, y_from: 5, y_to: 7 }, Horizontal { y: 7, x_from: 11, x_to: 12 }, Vertical { x: 12, y_from: 7, y_to: 6 }, Horizontal { y: 6, x_from: 12, x_to: 14 }, Vertical { x: 14, y_from: 6, y_to: 4 }, Horizontal { y: 4, x_from: 14, x_to: 17 }, Vertical { x: 17, y_from: 4, y_to: 5 }, Horizontal { y: 5, x_from: 17, x_to: 15 }, Vertical { x: 15, y_from: 5, y_to: 7 }, Horizontal { y: 7, x_from: 15, x_to: 17 }, Vertical { x: 17, y_from: 7, y_to: 8 }, Horizontal { y: 8, x_from: 17, x_to: 14 }, Vertical { x: 14, y_from: 8, y_to: 7 }, Horizontal { y: 7, x_from: 14, x_to: 13 }, Vertical { x: 13, y_from: 7, y_to: 9 }, Horizontal { y: 9, x_from: 13, x_to: 15 }, Vertical { x: 15, y_from: 9, y_to: 11 }, Horizontal { y: 11, x_from: 15, x_to: 17 }, Vertical { x: 17, y_from: 11, y_to: 12 }, Horizontal { y: 12, x_from: 17, x_to: 14 }, Vertical { x: 14, y_from: 12, y_to: 10 }, Horizontal { y: 10, x_from: 14, x_to: 12 }, Vertical { x: 12, y_from: 10, y_to: 8 }, Horizontal { y: 8, x_from: 12, x_to: 10 }, Vertical { x: 10, y_from: 8, y_to: 7 }, Horizontal { y: 7, x_from: 10, x_to: 9 }, Vertical { x: 9, y_from: 7, y_to: 9 }, Horizontal { y: 9, x_from: 9, x_to: 11 }, Vertical { x: 11, y_from: 9, y_to: 11 }, Horizontal { y: 11, x_from: 11, x_to: 13 }, Vertical { x: 13, y_from: 11, y_to: 13 }, Horizontal { y: 13, x_from: 13, x_to: 15 }, Vertical { x: 15, y_from: 13, y_to: 15 }, Horizontal { y: 15, x_from: 15, x_to: 14 }, Vertical { x: 14, y_from: 15, y_to: 14 }, Horizontal { y: 14, x_from: 14, x_to: 12 }, Vertical { x: 12, y_from: 14, y_to: 12 }, Horizontal { y: 12, x_from: 12, x_to: 10 }, Vertical { x: 10, y_from: 12, y_to: 10 }, Horizontal { y: 10, x_from: 10, x_to: 8 }, Vertical { x: 8, y_from: 10, y_to: 11 }, Horizontal { y: 11, x_from: 8, x_to: 9 }, Vertical { x: 9, y_from: 11, y_to: 13 }, Horizontal { y: 13, x_from: 9, x_to: 11 }, Vertical { x: 11, y_from: 13, y_to: 15 }, Horizontal { y: 15, x_from: 11, x_to: 10 }, Vertical { x: 10, y_from: 15, y_to: 14 }, Horizontal { y: 14, x_from: 10, x_to: 8 }, Vertical { x: 8, y_from: 14, y_to: 15 }, Horizontal { y: 15, x_from: 8, x_to: 7 }, Vertical { x: 7, y_from: 15, y_to: 13 }, Horizontal { y: 13, x_from: 7, x_to: 8 }, Vertical { x: 8, y_from: 13, y_to: 12 }, Horizontal { y: 12, x_from: 8, x_to: 6 }, Vertical { x: 6, y_from: 12, y_to: 14 }, Horizontal { y: 14, x_from: 6, x_to: 4 }, Vertical { x: 4, y_from: 14, y_to: 15 }, Horizontal { y: 15, x_from: 4, x_to: 3 }, Vertical { x: 3, y_from: 15, y_to: 13 }, Horizontal { y: 13, x_from: 3, x_to: 5 }, Vertical { x: 5, y_from: 13, y_to: 11 }, Horizontal { y: 11, x_from: 5, x_to: 7 }, Vertical { x: 7, y_from: 11, y_to: 9 }, Horizontal { y: 9, x_from: 7, x_to: 8 }, Vertical { x: 8, y_from: 9, y_to: 8 }, Horizontal { y: 8, x_from: 8, x_to: 6 }, Vertical { x: 6, y_from: 8, y_to: 10 }, Horizontal { y: 10, x_from: 6, x_to: 4 }, Vertical { x: 4, y_from: 10, y_to: 12 }, Horizontal { y: 12, x_from: 4, x_to: 2 }, Vertical { x: 2, y_from: 12, y_to: 14 }, Horizontal { y: 14, x_from: 2, x_to: 0 }, Vertical { x: 0, y_from: 14, y_to: 13 }, Horizontal { y: 13, x_from: 0, x_to: 1 }, Vertical { x: 1, y_from: 13, y_to: 11 }, Horizontal { y: 11, x_from: 1, x_to: 3 }, Vertical { x: 3, y_from: 11, y_to: 9 }, Horizontal { y: 9, x_from: 3, x_to: 5 }, Vertical { x: 5, y_from: 9, y_to: 7 }, Horizontal { y: 7, x_from: 5, x_to: 4 }, Vertical { x: 4, y_from: 7, y_to: 8 }, Horizontal { y: 8, x_from: 4, x_to: 2 }, Vertical { x: 2, y_from: 8, y_to: 10 }, Horizontal { y: 10, x_from: 2, x_to: 0 }, Vertical { x: 0, y_from: 10, y_to: 9 }, Horizontal { y: 9, x_from: 0, x_to: 1 }, Vertical { x: 1, y_from: 9, y_to: 7 }, Horizontal { y: 7, x_from: 1, x_to: 0 }, Vertical { x: 0, y_from: 7, y_to: 6 }, Horizontal { y: 6, x_from: 0, x_to: 2 }, Vertical { x: 2, y_from: 6, y_to: 7 }, Horizontal { y: 7, x_from: 2, x_to: 3 }, Vertical { x: 3, y_from: 7, y_to: 5 }, Horizontal { y: 5, x_from: 3, x_to: 1 }, Vertical { x: 1, y_from: 5, y_to: 3 }, Horizontal { y: 3, x_from: 1, x_to: 0 }, Vertical { x: 0, y_from: 1, y_to: 0 }, Horizontal { y: 0, x_from: 0, x_to: 2 }, Vertical { x: 2, y_from: 0, y_to: 2 }, Horizontal { y: 2, x_from: 2, x_to: 4 }, Vertical { x: 4, y_from: 2, y_to: 4 }, Horizontal { y: 4, x_from: 4, x_to: 6 }, Vertical { x: 6, y_from: 4, y_to: 6 }, Horizontal { y: 6, x_from: 6, x_to: 8 }, Vertical { x: 8, y_from: 6, y_to: 5 }, Horizontal { y: 5, x_from: 8, x_to: 7 }, Vertical { x: 7, y_from: 5, y_to: 3 }, Horizontal { y: 3, x_from: 7, x_to: 5 }, Vertical { x: 5, y_from: 3, y_to: 1 }, Horizontal { y: 1, x_from: 5, x_to: 3 }, Vertical { x: 3, y_from: 1, y_to: 0 }, Horizontal { y: 0, x_from: 3, x_to: 6 }, Vertical { x: 6, y_from: 0, y_to: 2 }, Horizontal { y: 2, x_from: 6, x_to: 8 }, Vertical { x: 8, y_from: 2, y_to: 1 }, Horizontal { y: 1, x_from: 8, x_to: 7 }, Vertical { x: 7, y_from: 1, y_to: 0 }, Horizontal { y: 0, x_from: 7, x_to: 11 }, Vertical { x: 11, y_from: 0, y_to: 1 }, Horizontal { y: 1, x_from: 11, x_to: 9 }, Vertical { x: 9, y_from: 1, y_to: 3 }, Horizontal { y: 3, x_from: 9, x_to: 8 }, Vertical { x: 8, y_from: 3, y_to: 4 }, Horizontal { y: 4, x_from: 8, x_to: 10 }, Vertical { x: 10, y_from: 4, y_to: 2 }, Horizontal { y: 2, x_from: 10, x_to: 12 }, Vertical { x: 12, y_from: 2, y_to: 0 }, Horizontal { y: 0, x_from: 12, x_to: 15 }, Vertical { x: 15, y_from: 0, y_to: 1 }, Horizontal { y: 1, x_from: 15, x_to: 13 }, Vertical { x: 13, y_from: 1, y_to: 3 }, Horizontal { y: 3, x_from: 13, x_to: 11 }, Vertical { x: 11, y_from: 3, y_to: 5 }, Horizontal { y: 5, x_from: 11, x_to: 9 }, Vertical { x: 9, y_from: 5, y_to: 7 }, Horizontal { y: 7, x_from: 9, x_to: 10 }, Vertical { x: 10, y_from: 7, y_to: 6 }, Horizontal { y: 6, x_from: 10, x_to: 12 }, Vertical { x: 12, y_from: 6, y_to: 4 }, Horizontal { y: 4, x_from: 12, x_to: 14 }, Vertical { x: 14, y_from: 4, y_to: 2 }, Horizontal { y: 2, x_from: 14, x_to: 17 }, Vertical { x: 17, y_from: 2, y_to: 3 }, Horizontal { y: 3, x_from: 17, x_to: 15 }, Vertical { x: 15, y_from: 3, y_to: 5 }, Horizontal { y: 5, x_from: 15, x_to: 13 }, Vertical { x: 13, y_from: 5, y_to: 7 }, Horizontal { y: 7, x_from: 13, x_to: 14 }, Vertical { x: 14, y_from: 7, y_to: 6 }, Horizontal { y: 6, x_from: 14, x_to: 17 }, Vertical { x: 17, y_from: 6, y_to: 7 }, Horizontal { y: 7, x_from: 17, x_to: 15 }, Vertical { x: 15, y_from: 7, y_to: 9 }, Horizontal { y: 9, x_from: 15, x_to: 17 }, Vertical { x: 17, y_from: 9, y_to: 10 }, Horizontal { y: 10, x_from: 17, x_to: 14 }, Vertical { x: 14, y_from: 10, y_to: 8 }, Horizontal { y: 8, x_from: 14, x_to: 12 }, Vertical { x: 12, y_from: 8, y_to: 7 }, Horizontal { y: 7, x_from: 12, x_to: 11 }, Vertical { x: 11, y_from: 7, y_to: 9 }, Horizontal { y: 9, x_from: 11, x_to: 13 }, Vertical { x: 13, y_from: 9, y_to: 11 }, Horizontal { y: 11, x_from: 13, x_to: 15 }, Vertical { x: 15, y_from: 11, y_to: 13 }, Horizontal { y: 13, x_from: 15, x_to: 17 }, Vertical { x: 17, y_from: 13, y_to: 14 }, Horizontal { y: 14, x_from: 17, x_to: 14 }, Vertical { x: 14, y_from: 14, y_to: 12 }, Horizontal { y: 12, x_from: 14, x_to: 12 }, Vertical { x: 12, y_from: 12, y_to: 10 }, Horizontal { y: 10, x_from: 12, x_to: 10 }, Vertical { x: 10, y_from: 10, y_to: 8 }, Horizontal { y: 8, x_from: 10, x_to: 8 }, Vertical { x: 8, y_from: 8, y_to: 9 }, Horizontal { y: 9, x_from: 8, x_to: 9 }, Vertical { x: 9, y_from: 9, y_to: 11 }, Horizontal { y: 11, x_from: 9, x_to: 11 }, Vertical { x: 11, y_from: 11, y_to: 13 }, Horizontal { y: 13, x_from: 11, x_to: 13 }, Vertical { x: 13, y_from: 13, y_to: 15 }, Horizontal { y: 15, x_from: 13, x_to: 12 }, Vertical { x: 12, y_from: 15, y_to: 14 }, Horizontal { y: 14, x_from: 12, x_to: 10 }, Vertical { x: 10, y_from: 14, y_to: 12 }, Horizontal { y: 12, x_from: 10, x_to: 8 }, Vertical { x: 8, y_from: 12, y_to: 13 }, Horizontal { y: 13, x_from: 8, x_to: 9 }, Vertical { x: 9, y_from: 13, y_to: 15 }, Horizontal { y: 15, x_from: 9, x_to: 8 }, Vertical { x: 8, y_from: 15, y_to: 14 }, Horizontal { y: 14, x_from: 8, x_to: 6 }, Vertical { x: 6, y_from: 14, y_to: 15 }, Horizontal { y: 15, x_from: 6, x_to: 5 }, Vertical { x: 5, y_from: 15, y_to: 13 }, Horizontal { y: 13, x_from: 5, x_to: 7 }, Vertical { x: 7, y_from: 13, y_to: 11 }, Horizontal { y: 11, x_from: 7, x_to: 8 }, Vertical { x: 8, y_from: 11, y_to: 10 }, Horizontal { y: 10, x_from: 8, x_to: 6 }, Vertical { x: 6, y_from: 10, y_to: 12 }, Horizontal { y: 12, x_from: 6, x_to: 4 }, Vertical { x: 4, y_from: 12, y_to: 14 }, Horizontal { y: 14, x_from: 4, x_to: 2 }, Vertical { x: 2, y_from: 14, y_to: 15 }, Horizontal { y: 15, x_from: 2, x_to: 1 }, Vertical { x: 1, y_from: 15, y_to: 13 }, Horizontal { y: 13, x_from: 1, x_to: 3 }, Vertical { x: 3, y_from: 13, y_to: 11 }, Horizontal { y: 11, x_from: 3, x_to: 5 }, Vertical { x: 5, y_from: 11, y_to: 9 }, Horizontal { y: 9, x_from: 5, x_to: 7 }, Vertical { x: 7, y_from: 9, y_to: 7 }, Horizontal { y: 7, x_from: 7, x_to: 6 }, Vertical { x: 6, y_from: 7, y_to: 8 }, Horizontal { y: 8, x_from: 6, x_to: 4 }, Vertical { x: 4, y_from: 8, y_to: 10 }, Horizontal { y: 10, x_from: 4, x_to: 2 }, Vertical { x: 2, y_from: 10, y_to: 12 }, Horizontal { y: 12, x_from: 2, x_to: 0 }, Vertical { x: 0, y_from: 12, y_to: 11 }, Horizontal { y: 11, x_from: 0, x_to: 1 }, Vertical { x: 1, y_from: 11, y_to: 9 }, Horizontal { y: 9, x_from: 1, x_to: 3 }, Vertical { x: 3, y_from: 9, y_to: 7 }, Horizontal { y: 7, x_from: 3, x_to: 2 }, Vertical { x: 2, y_from: 7, y_to: 8 }, Horizontal { y: 8, x_from: 2, x_to: 0 }, Vertical { x: 0, y_from: 8, y_to: 7 }, Horizontal { y: 7, x_from: 0, x_to: 1 }, Vertical { x: 1, y_from: 7, y_to: 5 }, Horizontal { y: 5, x_from: 1, x_to: 0 }, Vertical { x: 0, y_from: 5, y_to: 4 }, Horizontal { y: 4, x_from: 0, x_to: 2 }, Vertical { x: 2, y_from: 4, y_to: 6 }, Horizontal { y: 6, x_from: 2, x_to: 4 }, Vertical { x: 4, y_from: 6, y_to: 7 }, Horizontal { y: 7, x_from: 4, x_to: 5 }, Vertical { x: 5, y_from: 7, y_to: 5 }, Horizontal { y: 5, x_from: 5, x_to: 3 }, Vertical { x: 3, y_from: 5, y_to: 3 }, Horizontal { y: 3, x_from: 3, x_to: 1 }, Vertical { x: 1, y_from: 3, y_to: 1 }, Horizontal { y: 1, x_from: 1, x_to: 0 }],
    offsets: [0, 0, 0, 0, 1, 0, -1, 0, 0, 1, 0, 1, -1, 0, -1, 0, 1, -1, 1, -1, 1, 0, 0, 0, 1, 1, 1, 1, 0, 0, -1, 0, 0, -1, 1, 1, 0, -1, -1, 1, 1, 0, 0, -1, 2, 3, -1, -1, 1, -1, 0, 1, 0, -1, 1, 2, -1, -1, 0, 1, 1, -1, -1, -1, 0, 1, -2, -1, 1, 1, -1, 1, -1, -1, 0, 1, -1, -2, 0, 1, 0, -1, -1, 1, 1, 1, -2, -1, 0, 2, -1, -1, 1, -1, 0, 1, 0, -2, 1, -1, 0, -1, 0, 1, 1, -1, -1, -2, 2, 1, -1, 0, 1, 1, -1, 1, -1, -1, 1, 2, -1, 1, 1, 0, 0, 0, 1, 0, 0, -1, 0, 0, 0, -1, 0, 0, 0, -1, 1, 0, 0, 0, -2, 2, 0, 0, 0, -1, 0, 0, 2, 0, 0, -1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, -1, 0, 0, -1, 0, 0, 0, 1, 0, 0, 0, -2, -1, 0, 0, 0, 0, 0, 0, 3, -2, 0, 0, 2, 0, 0, 0, 1, -1, 0, 0, 0, -1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, -1, 0, 0, -1, 0, 0, 0, 2, -2, 0, 0, 0, -2, 0, 0, -3, -2, 0, 0, -2, 0, 0, 0, -2, -1, 0, 0, 0, -1, 0, 0, -1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0, 2, 1, 0, 0, 0, 1, 0, 0, -3, 2, 0, 0, -2, 0, 0, 0, -1, 0, 0, 0, 0, 1, 0, 0, 0, -1, 0, 0, -1, 0, 0, 0, 1, 0, 0, -2, 0, 0, 2, 0, 0, 0, 1, 2, 0, 0, 0, -1, 1, 0, 0, 0, 1, 0, 0, 0, -1, 0, 0, -1, 0, 0, 0, 1, 1, 0, 0, 0, -2, 0, 0, -1, -3, 0, 0, -1, 0, 0, 0, -1, 2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, -2, 2, 0, 0, 0, 2, 0, 0, -3, 3, 0, 0, -1, 0, 0, 0, -1, -2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 2, 0, 0, 0, 2, 0, 0, 2, 3, 0, 0, 1, 0, 0, 0, 1, -2, 0, 0, 0, -1, 0, 0],
    parts_len: [6, 4, 12, 12, 44, 44, 148, 148],
    diagram: EIGHT,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_complete() {
        assert!(EIGHT.complete());
    }
}
