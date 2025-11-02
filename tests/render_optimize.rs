use venn_diagrams::{
    constants,
    svg::{DiagramConfig, Layout},
};

use crate::common::{COLORS, VALUES, normalize};

#[macro_use]
mod common;

pub fn test_render_paths(name: &str, path_layout: Layout, config: &DiagramConfig) {
    let n = path_layout.n();
    let colors = &COLORS[0..n];
    let values = normalize(&VALUES[0..n]);
    let svg = path_layout.to_svg(&values, &colors, config);
    compare_snapshot!(name, svg);
}

#[test]
fn two() {
    let paths = constants::d2::LAYOUT_OPTIMIZED_TWO;
    test_render_paths("two.svg", paths.into(), &DiagramConfig::default())
}

#[test]
fn three() {
    let paths = constants::d3::LAYOUT_OPTIMIZED_THREE;
    test_render_paths("three.svg", paths.into(), &DiagramConfig::default())
}

#[test]
fn four() {
    let paths = constants::d4::LAYOUT_OPTIMIZED_FOUR;
    test_render_paths("four.svg", paths.into(), &DiagramConfig::default())
}

#[test]
fn five() {
    let paths = constants::d5::LAYOUT_OPTIMIZED_FIVE;
    test_render_paths("five.svg", paths.into(), &DiagramConfig::default())
}

#[test]
fn six() {
    let paths = constants::d6::LAYOUT_OPTIMIZED_SIX;
    test_render_paths("six.svg", paths.into(), &DiagramConfig::default())
}

#[test]
fn seven() {
    let paths = constants::d7::LAYOUT_OPTIMIZED_SEVEN;
    test_render_paths("seven.svg", paths.into(), &DiagramConfig::default())
}

#[test]
fn eight() {
    let paths = constants::d8::LAYOUT_OPTIMIZED_EIGHT;
    test_render_paths("eight.svg", paths.into(), &DiagramConfig::default())
}

// Used on the frontpage of the docs
#[test]
fn three_docs() {
    let paths: Layout = constants::d3::LAYOUT_OPTIMIZED_THREE.into();
    let values = &[0.3, 0.3, 0.4];
    let colors = &["MediumVioletRed", "DarkOrange", "DeepSkyBlue"];

    let svg = paths.to_svg(values, colors, &DiagramConfig::default());
    compare_snapshot!("three_docs.svg", svg);
}
