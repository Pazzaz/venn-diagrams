use venn_diagrams::{
    constants::{self, d3::THREE},
    diagram::Diagram,
    svg::{CornerStyle, DiagramConfig},
};

use crate::common::{COLORS, VALUES, normalize};

#[macro_use]
mod common;

pub fn test_venn_greedy(name: &str, venn: Diagram, config: &DiagramConfig) {
    let n = venn.n();
    let colors = &COLORS[0..n];
    let values = normalize(&VALUES[0..n]);
    let paths = venn.layout_greedy();

    let svg = paths.to_svg(&values, &colors, config);
    compare_snapshot!(name, svg);
}

#[test]
fn eight() {
    test_venn_greedy("eight.svg", constants::d8::EIGHT.into(), &DiagramConfig::default());
}

#[test]
fn seven() {
    test_venn_greedy("seven.svg", constants::d7::SEVEN.into(), &DiagramConfig::default());
}

#[test]
fn six() {
    test_venn_greedy("six.svg", constants::d6::SIX.into(), &DiagramConfig::default());
}

#[test]
fn five() {
    test_venn_greedy("five.svg", constants::d5::FIVE.into(), &DiagramConfig::default());
}

#[test]
fn four() {
    test_venn_greedy("four.svg", constants::d4::FOUR.into(), &DiagramConfig::default());
}

#[test]
fn three() {
    test_venn_greedy("three.svg", constants::d3::THREE.into(), &DiagramConfig::default());
}

#[test]
fn two() {
    test_venn_greedy("two.svg", constants::d2::TWO.into(), &DiagramConfig::default());
}

#[test]
fn four_wider() {
    let mut config = DiagramConfig::default();
    config.line_width = 0.1;
    test_venn_greedy("four_wide.svg", constants::d4::FOUR.into(), &mut config);
}

#[test]
fn eight_straight() {
    let mut config = DiagramConfig::default();
    config.corner_style = CornerStyle::Straight;
    test_venn_greedy("eight_straight.svg", constants::d8::EIGHT.into(), &mut config);
}

// Used on the frontpage of the docs
#[test]
fn three_docs() {
    let venn: Diagram = THREE.into();
    let paths = venn.layout_greedy();
    let values = &[0.3, 0.3, 0.4];
    let colors = &["MediumVioletRed", "DarkOrange", "DeepSkyBlue"];
    let svg = paths.to_svg(values, colors, &DiagramConfig::default());
    compare_snapshot!("three_docs.svg", svg);
}
