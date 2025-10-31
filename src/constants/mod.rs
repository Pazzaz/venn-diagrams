//! Premade Venn diagrams
//!
//! For 2 to 8 sets: [2][d2],
//! [3][d3], [4][d4], [5][d5], [6][d6], [7][d7],
//! [8][d8].

pub mod d2;
pub mod d3;
pub mod d4;
pub mod d5;
pub mod d6;
pub mod d7;
pub mod d8;

use std::fmt::Write;

use crate::svg::PathLayout;

// Function used to create a `PathLayoutConst` from a `PathLayout`.
#[doc(hidden)]
pub fn path_layout_to_const(
    variable_name: &str,
    path_layout: PathLayout,
    diagram_name: &str,
) -> Option<String> {
    let l: usize = path_layout.combined_paths.iter().map(Vec::len).sum();
    let k: usize = path_layout.combined_paths.len();
    let x: usize = path_layout.width;
    let y: usize = path_layout.height;

    let combined_paths: Vec<_> = path_layout.combined_paths.iter().flatten().collect();
    let offsets: Vec<_> = path_layout.offsets.iter().flatten().collect();
    let parts_len: Vec<_> = path_layout.offsets.iter().map(Vec::len).collect();

    let mut out = String::new();

    writeln!(&mut out, "#[rustfmt::skip]").ok()?;
    writeln!(
        &mut out,
        "pub const {variable_name}: PathLayoutConst<{l}, {k}, {x}, {y}> = PathLayoutConst {{"
    )
    .ok()?;
    writeln!(&mut out, "    combined_paths: {combined_paths:?},").ok()?;
    writeln!(&mut out, "    offsets: {offsets:?},").ok()?;
    writeln!(&mut out, "    parts_len: {parts_len:?},").ok()?;
    writeln!(&mut out, "    diagram: {diagram_name},").ok()?;
    writeln!(&mut out, "}};").ok()?;

    Some(out)
}
