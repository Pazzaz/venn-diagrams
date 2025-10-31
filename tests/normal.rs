use venn_diagrams::{
    constants,
    svg::{CornerStyle, DiagramConfig},
};

use crate::common::test_venn_greedy;

mod common;

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
