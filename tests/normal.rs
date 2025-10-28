use venn_diagrams::{
    svg::{CornerStyle, DiagramConfig},
    venn,
};

use crate::common::{COLORS, VALUES, normalize, test_venn};

mod common;

#[test]
fn eight() {
    let colors = COLORS;
    let values = normalize(&VALUES);
    test_venn("eight.svg", venn::EIGHT.into(), &values, &colors, &DiagramConfig::default());
}

#[test]
fn six() {
    let colors = &COLORS[0..6];
    let values = normalize(&VALUES[0..6]);
    test_venn("six.svg", venn::SIX.into(), &values, &colors, &DiagramConfig::default());
}

#[test]
fn five() {
    let colors = &COLORS[0..5];
    let values = normalize(&VALUES[0..5]);
    test_venn("five.svg", venn::FIVE.into(), &values, &colors, &DiagramConfig::default());
}

#[test]
fn four() {
    let colors = &COLORS[0..4];
    let values = normalize(&VALUES[0..4]);
    test_venn("four.svg", venn::FOUR.into(), &values, &colors, &DiagramConfig::default());
}

#[test]
fn three() {
    let colors = &COLORS[0..3];
    let values = normalize(&VALUES[0..3]);
    test_venn("three.svg", venn::THREE.into(), &values, &colors, &DiagramConfig::default());
}

#[test]
fn two() {
    let colors = &COLORS[0..2];
    let values = normalize(&VALUES[0..2]);
    test_venn("two.svg", venn::TWO.into(), &values, &colors, &DiagramConfig::default());
}

#[test]
fn four_wider() {
    let colors = &COLORS[0..4];
    let values = normalize(&VALUES[0..4]);
    let mut config = DiagramConfig::default();
    config.line_width = 0.1;
    test_venn("four_wide.svg", venn::FOUR.into(), &values, &colors, &mut config);
}

#[test]
fn eight_straight() {
    let colors = COLORS;
    let values = normalize(&VALUES);
    let mut config = DiagramConfig::default();
    config.corner_style = CornerStyle::Straight;
    test_venn("eight_straight.svg", venn::EIGHT.into(), &values, &colors, &mut config);
}
