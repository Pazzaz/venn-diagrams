#![cfg(feature = "optimize")]

use venn_diagrams::{
    svg::{DiagramConfig, OffsetMethod},
    constants,
};

use crate::common::{COLORS, VALUES, normalize, test_venn};

mod common;

#[test]
fn five() {
    let colors = &COLORS[0..5];
    let values = normalize(&VALUES[0..5]);
    let mut config = DiagramConfig::default();
    config.offset_method = OffsetMethod::Optimizing;
    test_venn("five.svg", constants::FIVE.into(), &values, &colors, &mut config);
}

#[test]
#[ignore = "expensive"]
fn eight() {
    let colors = COLORS;
    let values = normalize(&VALUES);
    let mut config = DiagramConfig::default();
    config.offset_method = OffsetMethod::Optimizing;
    test_venn("eight.svg", constants::EIGHT.into(), &values, &colors, &mut config);
}
