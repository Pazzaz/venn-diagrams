//! # Venn Diagrams
//!
//! A library to work with [Venn diagrams][wp-venn], specifically those
//! consisting of [polyominos][wp-poly] on a grid.
//!
//! # Example
//!
//! Consider three sets which consist of 30%, 30% and 40%.
//!
//! ```
//! use venn_diagrams::svg::DiagramConfig;
//! use venn_diagrams::constants::d3::THREE;
//! use venn_diagrams::diagram::Diagram;
//!
//! // First we have a Venn diagram
//! let diagram: Diagram = THREE.into();
//!
//! // Then we can calculate the layout of the edges
//! let paths = diagram.layout_greedy();
//!
//! // Then we can render the SVG
//! let values = &[0.3, 0.3, 0.4];
//! let colors = &["MediumVioletRed", "DarkOrange", "DeepSkyBlue"];
//! let svg = paths.to_svg(values, colors, &DiagramConfig::default());
//! ```
//!
//! Then the SVG looks like this:
//!
//! <div align="center">
//!
//! ![Three sets][greedy]
//!
//! </div>
//!
//! If we want to minimize the number of edge crossings we can use an optimized
//! layout.
//!
//! ```
//! use venn_diagrams::constants::d3::PATHLAYOUT_THREE_OPTIMIZING;
//! use venn_diagrams::svg::{DiagramConfig, PathLayout};
//!
//! let paths: PathLayout = PATHLAYOUT_THREE_OPTIMIZING.into();
//!
//! // Then we can render the SVG
//! let values = &[0.3, 0.3, 0.4];
//! let colors = &["MediumVioletRed", "DarkOrange", "DeepSkyBlue"];
//! let svg = paths.to_svg(values, colors, &DiagramConfig::default());
//! ```
//!
//! The resulting SVG looks like this:
//!
//! <div align="center">
//!
//! ![Three sets, optimized][optimized]
//!
//! </div>
//!
//! # Research
//! Parts of this library is based on the following research:
//! - "Minimum Area Venn Diagrams Whose Curves are Polyominoes"; Stirling Chow, Frank Ruskey; [arxiv:0603068](https://arxiv.org/abs/math/0603068)
//! - "Minimum area polyomino Venn diagrams"; Bette Bultena, Matthew Klimesh, Frank Ruskey; [doi:10.20382/jocg.v3i1a8](https://doi.org/10.20382/jocg.v3i1a8)
//!
//! [wp-venn]: https://en.wikipedia.org/wiki/Venn_diagram
//! [wp-poly]: https://en.wikipedia.org/wiki/Polyomino
#![doc = embed_doc_image::embed_image!("greedy", "tests/snapshots/normal__three_docs.snap.svg")]
#![doc = embed_doc_image::embed_image!("optimized", "tests/snapshots/render_optimize__three_docs.snap.svg")]
#![warn(missing_docs)]

mod direction;

pub mod constants;
pub mod polyomino;
pub mod svg;

mod matrix;

pub mod diagram;
