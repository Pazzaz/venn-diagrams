//! Venn diagram for seven sets.
//!
//! The diagram itself is [`SEVEN`], and the premade layout is
//! [`LAYOUT_OPTIMIZED_SEVEN`].
//!
//! # Example
//! <div align="center">
//!
//! ![Two sets][example]
//!
//! </div>
//!
//! # Source
//! Sourced from [\[2006\]](crate#ref-2006).
#![doc = embed_doc_image::embed_image!("example", "tests/common/snapshots/render_optimize__common__seven.snap.svg")]
use crate::{
    diagram::DiagramConst,
    direction::DirectedEdge::{Horizontal, Vertical},
    svg::LayoutConst,
};

/// A Venn diagram for 7 sets.
///
/// For more info, see [`d7`][super::d7].
pub const SEVEN: DiagramConst<7, 14, 14> = DiagramConst::from_binary_str(GRIDS);

#[rustfmt::skip]
const GRIDS: [[&str; 14]; 7] = [
    [
        "00000000000000",
        "00000010011110",
        "00000011111110",
        "00000111111100",
        "00000011100000",
        "00000011100000",
        "00000011100000",
        "01111110000000",
        "00000011000000",
        "00001111111111",
        "00001111111110",
        "00001111110100",
        "00000011000000",
        "00000000000000",
    ],
    [
        "00000000000000",
        "00000000000000",
        "00000010011100",
        "00000011111100",
        "00001111100000",
        "00001111111110",
        "00001111111110",
        "00111111111110",
        "00111101111100",
        "00110001001100",
        "00000001001100",
        "00000001001000",
        "00000000001000",
        "00000000000000",
    ],
    [
        "00000000000000",
        "00000000000000",
        "00000100000000",
        "00000110000000",
        "00000111111000",
        "00000011111100",
        "00000011100000",
        "00000011111100",
        "00011111111000",
        "00001111101000",
        "01111111101000",
        "00111110101100",
        "00111110100100",
        "00010010000000",
    ],
    [
        "00000000011110",
        "00000000011110",
        "00000000011000",
        "00000000011000",
        "00000010011100",
        "00001110111000",
        "00001110111110",
        "11111111111000",
        "00001111000000",
        "01001011110000",
        "01111010110000",
        "00010010010000",
        "00010000010000",
        "00010000000000",
    ],
    [
        "00000000000010",
        "00000001000010",
        "00000101000111",
        "00000101001100",
        "00001101011100",
        "00000111010000",
        "00000011011100",
        "00011111110000",
        "00000011111111",
        "01111110011111",
        "00100110010000",
        "00100100010000",
        "00110100010000",
        "00000000000000",
    ],
    [
        "00000000010100",
        "00000000110100",
        "00000000110111",
        "00000000110100",
        "00000000111100",
        "00000001100000",
        "00001111111000",
        "00001111100000",
        "00111110110001",
        "01110110110001",
        "00010100111111",
        "00110000101100",
        "00100000101100",
        "00000000000000",
    ],
    [
        "00000000001110",
        "00000000001110",
        "00000100001000",
        "00000100011100",
        "00001100010000",
        "00001100011110",
        "00000101110000",
        "00000111000000",
        "00000111100011",
        "01011101111111",
        "01111001001111",
        "00101001001100",
        "00001001001100",
        "00000001000000",
    ],
];

/// A layout for [`SEVEN`], optimized to minimize gaps and overlaps.
#[rustfmt::skip]
pub const LAYOUT_OPTIMIZED_SEVEN: LayoutConst<334, 7, 14, 14> = LayoutConst {
    combined_paths: [Vertical { x: 1, y_from: 8, y_to: 7 }, Horizontal { y: 7, x_from: 1, x_to: 6 }, Vertical { x: 6, y_from: 7, y_to: 4 }, Horizontal { y: 4, x_from: 6, x_to: 5 }, Vertical { x: 5, y_from: 4, y_to: 3 }, Horizontal { y: 3, x_from: 5, x_to: 6 }, Vertical { x: 6, y_from: 3, y_to: 1 }, Horizontal { y: 1, x_from: 6, x_to: 7 }, Vertical { x: 7, y_from: 1, y_to: 2 }, Horizontal { y: 2, x_from: 7, x_to: 9 }, Vertical { x: 9, y_from: 2, y_to: 1 }, Horizontal { y: 1, x_from: 9, x_to: 13 }, Vertical { x: 13, y_from: 1, y_to: 3 }, Horizontal { y: 3, x_from: 13, x_to: 12 }, Vertical { x: 12, y_from: 3, y_to: 4 }, Horizontal { y: 4, x_from: 12, x_to: 9 }, Vertical { x: 9, y_from: 4, y_to: 7 }, Horizontal { y: 7, x_from: 9, x_to: 7 }, Vertical { x: 7, y_from: 7, y_to: 8 }, Horizontal { y: 8, x_from: 7, x_to: 8 }, Vertical { x: 8, y_from: 8, y_to: 9 }, Horizontal { y: 9, x_from: 8, x_to: 14 }, Vertical { x: 14, y_from: 9, y_to: 10 }, Horizontal { y: 10, x_from: 14, x_to: 13 }, Vertical { x: 13, y_from: 10, y_to: 11 }, Horizontal { y: 11, x_from: 13, x_to: 12 }, Vertical { x: 12, y_from: 11, y_to: 12 }, Horizontal { y: 12, x_from: 12, x_to: 11 }, Vertical { x: 11, y_from: 12, y_to: 11 }, Horizontal { y: 11, x_from: 11, x_to: 10 }, Vertical { x: 10, y_from: 11, y_to: 12 }, Horizontal { y: 12, x_from: 10, x_to: 8 }, Vertical { x: 8, y_from: 12, y_to: 13 }, Horizontal { y: 13, x_from: 8, x_to: 6 }, Vertical { x: 6, y_from: 13, y_to: 12 }, Horizontal { y: 12, x_from: 6, x_to: 4 }, Vertical { x: 4, y_from: 12, y_to: 9 }, Horizontal { y: 9, x_from: 4, x_to: 6 }, Vertical { x: 6, y_from: 9, y_to: 8 }, Horizontal { y: 8, x_from: 6, x_to: 1 }, Vertical { x: 2, y_from: 10, y_to: 7 }, Horizontal { y: 7, x_from: 2, x_to: 4 }, Vertical { x: 4, y_from: 7, y_to: 4 }, Horizontal { y: 4, x_from: 4, x_to: 6 }, Vertical { x: 6, y_from: 4, y_to: 2 }, Horizontal { y: 2, x_from: 6, x_to: 7 }, Vertical { x: 7, y_from: 2, y_to: 3 }, Horizontal { y: 3, x_from: 7, x_to: 9 }, Vertical { x: 9, y_from: 3, y_to: 2 }, Horizontal { y: 2, x_from: 9, x_to: 12 }, Vertical { x: 12, y_from: 2, y_to: 4 }, Horizontal { y: 4, x_from: 12, x_to: 9 }, Vertical { x: 9, y_from: 4, y_to: 5 }, Horizontal { y: 5, x_from: 9, x_to: 13 }, Vertical { x: 13, y_from: 5, y_to: 8 }, Horizontal { y: 8, x_from: 13, x_to: 12 }, Vertical { x: 12, y_from: 8, y_to: 11 }, Horizontal { y: 11, x_from: 12, x_to: 11 }, Vertical { x: 11, y_from: 11, y_to: 13 }, Horizontal { y: 13, x_from: 11, x_to: 10 }, Vertical { x: 10, y_from: 13, y_to: 9 }, Horizontal { y: 9, x_from: 10, x_to: 8 }, Vertical { x: 8, y_from: 9, y_to: 12 }, Horizontal { y: 12, x_from: 8, x_to: 7 }, Vertical { x: 7, y_from: 12, y_to: 8 }, Horizontal { y: 8, x_from: 7, x_to: 6 }, Vertical { x: 6, y_from: 8, y_to: 9 }, Horizontal { y: 9, x_from: 6, x_to: 4 }, Vertical { x: 4, y_from: 9, y_to: 10 }, Horizontal { y: 10, x_from: 4, x_to: 2 }, Vertical { x: 1, y_from: 11, y_to: 10 }, Horizontal { y: 10, x_from: 1, x_to: 4 }, Vertical { x: 4, y_from: 10, y_to: 9 }, Horizontal { y: 9, x_from: 4, x_to: 3 }, Vertical { x: 3, y_from: 9, y_to: 8 }, Horizontal { y: 8, x_from: 3, x_to: 6 }, Vertical { x: 6, y_from: 8, y_to: 5 }, Horizontal { y: 5, x_from: 6, x_to: 5 }, Vertical { x: 5, y_from: 5, y_to: 2 }, Horizontal { y: 2, x_from: 5, x_to: 6 }, Vertical { x: 6, y_from: 2, y_to: 3 }, Horizontal { y: 3, x_from: 6, x_to: 7 }, Vertical { x: 7, y_from: 3, y_to: 4 }, Horizontal { y: 4, x_from: 7, x_to: 11 }, Vertical { x: 11, y_from: 4, y_to: 5 }, Horizontal { y: 5, x_from: 11, x_to: 12 }, Vertical { x: 12, y_from: 5, y_to: 6 }, Horizontal { y: 6, x_from: 12, x_to: 9 }, Vertical { x: 9, y_from: 6, y_to: 7 }, Horizontal { y: 7, x_from: 9, x_to: 12 }, Vertical { x: 12, y_from: 7, y_to: 8 }, Horizontal { y: 8, x_from: 12, x_to: 11 }, Vertical { x: 11, y_from: 8, y_to: 11 }, Horizontal { y: 11, x_from: 11, x_to: 12 }, Vertical { x: 12, y_from: 11, y_to: 13 }, Horizontal { y: 13, x_from: 12, x_to: 11 }, Vertical { x: 11, y_from: 13, y_to: 12 }, Horizontal { y: 12, x_from: 11, x_to: 10 }, Vertical { x: 10, y_from: 12, y_to: 9 }, Horizontal { y: 9, x_from: 10, x_to: 9 }, Vertical { x: 9, y_from: 9, y_to: 13 }, Horizontal { y: 13, x_from: 9, x_to: 8 }, Vertical { x: 8, y_from: 13, y_to: 11 }, Horizontal { y: 11, x_from: 8, x_to: 7 }, Vertical { x: 7, y_from: 11, y_to: 14 }, Horizontal { y: 14, x_from: 7, x_to: 6 }, Vertical { x: 6, y_from: 14, y_to: 13 }, Horizontal { y: 13, x_from: 6, x_to: 4 }, Vertical { x: 4, y_from: 13, y_to: 14 }, Horizontal { y: 14, x_from: 4, x_to: 3 }, Vertical { x: 3, y_from: 14, y_to: 13 }, Horizontal { y: 13, x_from: 3, x_to: 2 }, Vertical { x: 2, y_from: 13, y_to: 11 }, Horizontal { y: 11, x_from: 2, x_to: 1 }, Vertical { x: 0, y_from: 8, y_to: 7 }, Horizontal { y: 7, x_from: 0, x_to: 4 }, Vertical { x: 4, y_from: 7, y_to: 5 }, Horizontal { y: 5, x_from: 4, x_to: 6 }, Vertical { x: 6, y_from: 5, y_to: 4 }, Horizontal { y: 4, x_from: 6, x_to: 7 }, Vertical { x: 7, y_from: 4, y_to: 7 }, Horizontal { y: 7, x_from: 7, x_to: 8 }, Vertical { x: 8, y_from: 7, y_to: 5 }, Horizontal { y: 5, x_from: 8, x_to: 9 }, Vertical { x: 9, y_from: 5, y_to: 0 }, Horizontal { y: 0, x_from: 9, x_to: 13 }, Vertical { x: 13, y_from: 0, y_to: 2 }, Horizontal { y: 2, x_from: 13, x_to: 11 }, Vertical { x: 11, y_from: 2, y_to: 4 }, Horizontal { y: 4, x_from: 11, x_to: 12 }, Vertical { x: 12, y_from: 4, y_to: 5 }, Horizontal { y: 5, x_from: 12, x_to: 11 }, Vertical { x: 11, y_from: 5, y_to: 6 }, Horizontal { y: 6, x_from: 11, x_to: 13 }, Vertical { x: 13, y_from: 6, y_to: 7 }, Horizontal { y: 7, x_from: 13, x_to: 11 }, Vertical { x: 11, y_from: 7, y_to: 8 }, Horizontal { y: 8, x_from: 11, x_to: 8 }, Vertical { x: 8, y_from: 8, y_to: 9 }, Horizontal { y: 9, x_from: 8, x_to: 10 }, Vertical { x: 10, y_from: 9, y_to: 13 }, Horizontal { y: 13, x_from: 10, x_to: 9 }, Vertical { x: 9, y_from: 13, y_to: 11 }, Horizontal { y: 11, x_from: 9, x_to: 8 }, Vertical { x: 8, y_from: 11, y_to: 10 }, Horizontal { y: 10, x_from: 8, x_to: 7 }, Vertical { x: 7, y_from: 10, y_to: 12 }, Horizontal { y: 12, x_from: 7, x_to: 6 }, Vertical { x: 6, y_from: 12, y_to: 9 }, Horizontal { y: 9, x_from: 6, x_to: 5 }, Vertical { x: 5, y_from: 9, y_to: 11 }, Horizontal { y: 11, x_from: 5, x_to: 4 }, Vertical { x: 4, y_from: 11, y_to: 14 }, Horizontal { y: 14, x_from: 4, x_to: 3 }, Vertical { x: 3, y_from: 14, y_to: 11 }, Horizontal { y: 11, x_from: 3, x_to: 1 }, Vertical { x: 1, y_from: 11, y_to: 9 }, Horizontal { y: 9, x_from: 1, x_to: 2 }, Vertical { x: 2, y_from: 9, y_to: 10 }, Horizontal { y: 10, x_from: 2, x_to: 4 }, Vertical { x: 4, y_from: 10, y_to: 8 }, Horizontal { y: 8, x_from: 4, x_to: 0 }, Vertical { x: 1, y_from: 10, y_to: 9 }, Horizontal { y: 9, x_from: 1, x_to: 6 }, Vertical { x: 6, y_from: 9, y_to: 8 }, Horizontal { y: 8, x_from: 6, x_to: 3 }, Vertical { x: 3, y_from: 8, y_to: 7 }, Horizontal { y: 7, x_from: 3, x_to: 6 }, Vertical { x: 6, y_from: 7, y_to: 6 }, Horizontal { y: 6, x_from: 6, x_to: 5 }, Vertical { x: 5, y_from: 6, y_to: 5 }, Horizontal { y: 5, x_from: 5, x_to: 4 }, Vertical { x: 4, y_from: 5, y_to: 4 }, Horizontal { y: 4, x_from: 4, x_to: 5 }, Vertical { x: 5, y_from: 4, y_to: 2 }, Horizontal { y: 2, x_from: 5, x_to: 6 }, Vertical { x: 6, y_from: 2, y_to: 5 }, Horizontal { y: 5, x_from: 6, x_to: 7 }, Vertical { x: 7, y_from: 5, y_to: 1 }, Horizontal { y: 1, x_from: 7, x_to: 8 }, Vertical { x: 8, y_from: 1, y_to: 7 }, Horizontal { y: 7, x_from: 8, x_to: 9 }, Vertical { x: 9, y_from: 7, y_to: 4 }, Horizontal { y: 4, x_from: 9, x_to: 10 }, Vertical { x: 10, y_from: 4, y_to: 3 }, Horizontal { y: 3, x_from: 10, x_to: 11 }, Vertical { x: 11, y_from: 3, y_to: 2 }, Horizontal { y: 2, x_from: 11, x_to: 12 }, Vertical { x: 12, y_from: 2, y_to: 0 }, Horizontal { y: 0, x_from: 12, x_to: 13 }, Vertical { x: 13, y_from: 0, y_to: 2 }, Horizontal { y: 2, x_from: 13, x_to: 14 }, Vertical { x: 14, y_from: 2, y_to: 3 }, Horizontal { y: 3, x_from: 14, x_to: 12 }, Vertical { x: 12, y_from: 3, y_to: 5 }, Horizontal { y: 5, x_from: 12, x_to: 10 }, Vertical { x: 10, y_from: 5, y_to: 6 }, Horizontal { y: 6, x_from: 10, x_to: 12 }, Vertical { x: 12, y_from: 6, y_to: 7 }, Horizontal { y: 7, x_from: 12, x_to: 10 }, Vertical { x: 10, y_from: 7, y_to: 8 }, Horizontal { y: 8, x_from: 10, x_to: 14 }, Vertical { x: 14, y_from: 8, y_to: 10 }, Horizontal { y: 10, x_from: 14, x_to: 10 }, Vertical { x: 10, y_from: 10, y_to: 13 }, Horizontal { y: 13, x_from: 10, x_to: 9 }, Vertical { x: 9, y_from: 13, y_to: 9 }, Horizontal { y: 9, x_from: 9, x_to: 7 }, Vertical { x: 7, y_from: 9, y_to: 11 }, Horizontal { y: 11, x_from: 7, x_to: 6 }, Vertical { x: 6, y_from: 11, y_to: 13 }, Horizontal { y: 13, x_from: 6, x_to: 5 }, Vertical { x: 5, y_from: 13, y_to: 10 }, Horizontal { y: 10, x_from: 5, x_to: 3 }, Vertical { x: 3, y_from: 10, y_to: 12 }, Horizontal { y: 12, x_from: 3, x_to: 4 }, Vertical { x: 4, y_from: 12, y_to: 13 }, Horizontal { y: 13, x_from: 4, x_to: 2 }, Vertical { x: 2, y_from: 13, y_to: 10 }, Horizontal { y: 10, x_from: 2, x_to: 1 }, Vertical { x: 1, y_from: 10, y_to: 9 }, Horizontal { y: 9, x_from: 1, x_to: 2 }, Vertical { x: 2, y_from: 9, y_to: 8 }, Horizontal { y: 8, x_from: 2, x_to: 4 }, Vertical { x: 4, y_from: 8, y_to: 6 }, Horizontal { y: 6, x_from: 4, x_to: 7 }, Vertical { x: 7, y_from: 6, y_to: 5 }, Horizontal { y: 5, x_from: 7, x_to: 8 }, Vertical { x: 8, y_from: 5, y_to: 1 }, Horizontal { y: 1, x_from: 8, x_to: 9 }, Vertical { x: 9, y_from: 1, y_to: 0 }, Horizontal { y: 0, x_from: 9, x_to: 10 }, Vertical { x: 10, y_from: 0, y_to: 4 }, Horizontal { y: 4, x_from: 10, x_to: 11 }, Vertical { x: 11, y_from: 4, y_to: 0 }, Horizontal { y: 0, x_from: 11, x_to: 12 }, Vertical { x: 12, y_from: 0, y_to: 2 }, Horizontal { y: 2, x_from: 12, x_to: 14 }, Vertical { x: 14, y_from: 2, y_to: 3 }, Horizontal { y: 3, x_from: 14, x_to: 12 }, Vertical { x: 12, y_from: 3, y_to: 5 }, Horizontal { y: 5, x_from: 12, x_to: 9 }, Vertical { x: 9, y_from: 5, y_to: 6 }, Horizontal { y: 6, x_from: 9, x_to: 11 }, Vertical { x: 11, y_from: 6, y_to: 7 }, Horizontal { y: 7, x_from: 11, x_to: 9 }, Vertical { x: 9, y_from: 7, y_to: 8 }, Horizontal { y: 8, x_from: 9, x_to: 10 }, Vertical { x: 10, y_from: 8, y_to: 10 }, Horizontal { y: 10, x_from: 10, x_to: 13 }, Vertical { x: 13, y_from: 10, y_to: 8 }, Horizontal { y: 8, x_from: 13, x_to: 14 }, Vertical { x: 14, y_from: 8, y_to: 11 }, Horizontal { y: 11, x_from: 14, x_to: 12 }, Vertical { x: 12, y_from: 11, y_to: 13 }, Horizontal { y: 13, x_from: 12, x_to: 10 }, Vertical { x: 10, y_from: 13, y_to: 11 }, Horizontal { y: 11, x_from: 10, x_to: 9 }, Vertical { x: 9, y_from: 11, y_to: 13 }, Horizontal { y: 13, x_from: 9, x_to: 8 }, Vertical { x: 8, y_from: 13, y_to: 8 }, Horizontal { y: 8, x_from: 8, x_to: 7 }, Vertical { x: 7, y_from: 8, y_to: 10 }, Horizontal { y: 10, x_from: 7, x_to: 6 }, Vertical { x: 6, y_from: 10, y_to: 11 }, Horizontal { y: 11, x_from: 6, x_to: 5 }, Vertical { x: 5, y_from: 11, y_to: 9 }, Horizontal { y: 9, x_from: 5, x_to: 4 }, Vertical { x: 4, y_from: 9, y_to: 12 }, Horizontal { y: 12, x_from: 4, x_to: 3 }, Vertical { x: 3, y_from: 12, y_to: 13 }, Horizontal { y: 13, x_from: 3, x_to: 2 }, Vertical { x: 2, y_from: 13, y_to: 11 }, Horizontal { y: 11, x_from: 2, x_to: 3 }, Vertical { x: 3, y_from: 11, y_to: 10 }, Horizontal { y: 10, x_from: 3, x_to: 1 }, Vertical { x: 1, y_from: 11, y_to: 9 }, Horizontal { y: 9, x_from: 1, x_to: 2 }, Vertical { x: 2, y_from: 9, y_to: 10 }, Horizontal { y: 10, x_from: 2, x_to: 3 }, Vertical { x: 3, y_from: 10, y_to: 9 }, Horizontal { y: 9, x_from: 3, x_to: 5 }, Vertical { x: 5, y_from: 9, y_to: 6 }, Horizontal { y: 6, x_from: 5, x_to: 4 }, Vertical { x: 4, y_from: 6, y_to: 4 }, Horizontal { y: 4, x_from: 4, x_to: 5 }, Vertical { x: 5, y_from: 4, y_to: 2 }, Horizontal { y: 2, x_from: 5, x_to: 6 }, Vertical { x: 6, y_from: 2, y_to: 7 }, Horizontal { y: 7, x_from: 6, x_to: 7 }, Vertical { x: 7, y_from: 7, y_to: 6 }, Horizontal { y: 6, x_from: 7, x_to: 9 }, Vertical { x: 9, y_from: 6, y_to: 3 }, Horizontal { y: 3, x_from: 9, x_to: 10 }, Vertical { x: 10, y_from: 3, y_to: 0 }, Horizontal { y: 0, x_from: 10, x_to: 13 }, Vertical { x: 13, y_from: 0, y_to: 2 }, Horizontal { y: 2, x_from: 13, x_to: 11 }, Vertical { x: 11, y_from: 2, y_to: 3 }, Horizontal { y: 3, x_from: 11, x_to: 12 }, Vertical { x: 12, y_from: 3, y_to: 4 }, Horizontal { y: 4, x_from: 12, x_to: 10 }, Vertical { x: 10, y_from: 4, y_to: 5 }, Horizontal { y: 5, x_from: 10, x_to: 13 }, Vertical { x: 13, y_from: 5, y_to: 6 }, Horizontal { y: 6, x_from: 13, x_to: 10 }, Vertical { x: 10, y_from: 6, y_to: 7 }, Horizontal { y: 7, x_from: 10, x_to: 8 }, Vertical { x: 8, y_from: 7, y_to: 8 }, Horizontal { y: 8, x_from: 8, x_to: 9 }, Vertical { x: 9, y_from: 8, y_to: 9 }, Horizontal { y: 9, x_from: 9, x_to: 12 }, Vertical { x: 12, y_from: 9, y_to: 8 }, Horizontal { y: 8, x_from: 12, x_to: 14 }, Vertical { x: 14, y_from: 8, y_to: 11 }, Horizontal { y: 11, x_from: 14, x_to: 12 }, Vertical { x: 12, y_from: 11, y_to: 13 }, Horizontal { y: 13, x_from: 12, x_to: 10 }, Vertical { x: 10, y_from: 13, y_to: 10 }, Horizontal { y: 10, x_from: 10, x_to: 8 }, Vertical { x: 8, y_from: 10, y_to: 14 }, Horizontal { y: 14, x_from: 8, x_to: 7 }, Vertical { x: 7, y_from: 14, y_to: 9 }, Horizontal { y: 9, x_from: 7, x_to: 6 }, Vertical { x: 6, y_from: 9, y_to: 10 }, Horizontal { y: 10, x_from: 6, x_to: 5 }, Vertical { x: 5, y_from: 10, y_to: 13 }, Horizontal { y: 13, x_from: 5, x_to: 4 }, Vertical { x: 4, y_from: 13, y_to: 11 }, Horizontal { y: 11, x_from: 4, x_to: 3 }, Vertical { x: 3, y_from: 11, y_to: 12 }, Horizontal { y: 12, x_from: 3, x_to: 2 }, Vertical { x: 2, y_from: 12, y_to: 11 }, Horizontal { y: 11, x_from: 2, x_to: 1 }],
    offsets: [0, 0, -1, 0, 2, 0, 0, 0, -1, 0, 0, 0, -1, -1, -1, -1, 2, 0, 0, -1, 0, 0, -2, -1, 0, -1, -2, 0, -1, 0, 3, 0, 0, 0, 0, 0, 1, -1, -1, 0, 0, -2, -1, -1, 3, 0, 1, 0, 0, 0, 0, 1, 3, 0, 0, -1, 0, 1, 0, -1, 1, 1, 0, 0, 1, 0, 1, 1, -2, -2, 0, 1, -1, -1, 0, -2, 0, 1, 1, 1, -1, 0, 1, 0, 0, 3, 0, -1, 3, -1, 0, 0, 0, 0, -1, -1, 1, 0, 2, 2, 0, 0, -2, 0, 0, 0, 0, 1, 1, 1, -1, 1, -1, 0, 0, -1, 0, 0, 0, 0, 1, -1, -1, 0, -1, 0, 0, -1, 1, 0, 0, -2, 0, 0, -1, 0, 0, 0, 1, -2, -1, 0, 1, 0, -2, 0, -2, 0, 0, 2, 1, 0, 0, 0, 0, -1, 1, 1, -1, -1, 0, 1, 0, 0, 0, -1, 0, 1, -2, -1, 0, -1, 0, 0, -1, -1, 2, 0, 0, 0, 0, -1, 1, -2, -1, 0, -1, -2, 0, -2, 2, -1, 0, 1, 2, 2, 0, 2, 0, 1, 0, 1, -1, 0, 0, 1, -1, -1, 0, 0, -1, 0, 0, 0, 1, 1, -2, 0, 0, -1, -2, -2, 1, 2, 1, 0, 0, 0, 1, 0, 0, 1, 0, -3, 0, 1, -1, 0, -1, 0, 1, 1, 3, 0, 0, 0, 0, -1, 0, -1, 0, -1, 1, 1, 1, 1, -3, 0, -2, -1, -1, 0, 2, 0, 1, 0, 2, 2, 2, 0, -1, -1, 1, 0, 0, 0, -1, -1, 1, -3, 0, -2, 0, -1, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, -1, 1, 1, 2, 0, -2, -2, 0, -1, 1, 1, 0, 1, 0, -1, 0, -1, 1, 0, 0, 0, 0, 0, -2, 0, 1, 0, -1, 0, 1, 0, -1, 0, -1, 0, -1, 0, -2, 1],
    parts_len: [40, 30, 44, 48, 58, 56, 58],
    diagram: SEVEN,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_complete() {
        assert!(SEVEN.complete());
    }
}
