use venn_diagrams::{
    constants::{self, d3::PATHLAYOUT_THREE_OPTIMIZING},
    svg::{DiagramConfig, PathLayout},
};

use crate::common::test_render_paths;

mod common;

#[test]
fn two() {
    let paths = constants::d2::PATHLAYOUT_TWO_OPTIMIZING;
    test_render_paths("two.svg", paths.into(), &DiagramConfig::default())
}

#[test]
fn three() {
    let paths = constants::d3::PATHLAYOUT_THREE_OPTIMIZING;
    test_render_paths("three.svg", paths.into(), &DiagramConfig::default())
}

#[test]
fn four() {
    let paths = constants::d4::PATHLAYOUT_FOUR_OPTIMIZING;
    test_render_paths("four.svg", paths.into(), &DiagramConfig::default())
}

#[test]
fn five() {
    let paths = constants::d5::PATHLAYOUT_FIVE_OPTIMIZING;
    test_render_paths("five.svg", paths.into(), &DiagramConfig::default())
}

#[test]
fn six() {
    let paths = constants::d6::PATHLAYOUT_SIX_OPTIMIZING;
    test_render_paths("six.svg", paths.into(), &DiagramConfig::default())
}

#[test]
fn seven() {
    let paths = constants::d7::PATHLAYOUT_SEVEN_OPTIMIZING;
    test_render_paths("seven.svg", paths.into(), &DiagramConfig::default())
}

#[test]
fn eight() {
    let paths = constants::d8::PATHLAYOUT_EIGHT_OPTIMIZING;
    test_render_paths("eight.svg", paths.into(), &DiagramConfig::default())
}

// Used on the frontpage of the docs
#[test]
fn three_docs() {
    let paths: PathLayout = PATHLAYOUT_THREE_OPTIMIZING.into();
    let values = &[0.3, 0.3, 0.4];
    let colors = &["MediumVioletRed", "DarkOrange", "DeepSkyBlue"];
    let svg = paths.to_svg(values, colors, &DiagramConfig::default());
    insta::assert_binary_snapshot!(
        "three_docs.svg",
        svg.to_string().as_bytes().into_iter().cloned().collect()
    );
}
