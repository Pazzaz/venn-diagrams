use venn_diagrams::{
    constants,
    svg::{CornerStyle, DiagramConfig},
};

use crate::common::test_venn;

mod common;

#[test]
fn eight() {
    test_venn("eight.svg", constants::EIGHT.into(), &DiagramConfig::default());
}

#[test]
fn six() {
    test_venn("six.svg", constants::SIX.into(), &DiagramConfig::default());
}

#[test]
fn five() {
    test_venn("five.svg", constants::FIVE.into(), &DiagramConfig::default());
}

#[test]
fn four() {
    test_venn("four.svg", constants::FOUR.into(), &DiagramConfig::default());
}

#[test]
fn three() {
    test_venn("three.svg", constants::THREE.into(), &DiagramConfig::default());
}

#[test]
fn two() {
    test_venn("two.svg", constants::TWO.into(), &DiagramConfig::default());
}

#[test]
fn four_wider() {
    let mut config = DiagramConfig::default();
    config.line_width = 0.1;
    test_venn("four_wide.svg", constants::FOUR.into(), &mut config);
}

#[test]
fn eight_straight() {
    let mut config = DiagramConfig::default();
    config.corner_style = CornerStyle::Straight;
    test_venn("eight_straight.svg", constants::EIGHT.into(), &mut config);
}
