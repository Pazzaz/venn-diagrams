#![cfg(feature = "optimize")]

use venn_diagrams::{
    constants,
    svg::{DiagramConfig, OffsetMethod},
};

use crate::common::test_venn;

mod common;

#[test]
fn two() {
    let mut config = DiagramConfig::default();
    config.offset_method = OffsetMethod::Optimizing;
    test_venn("two.svg", constants::TWO.into(), &mut config);
}

#[test]
fn three() {
    let mut config = DiagramConfig::default();
    config.offset_method = OffsetMethod::Optimizing;
    test_venn("three.svg", constants::THREE.into(), &mut config);
}

#[test]
fn four() {
    let mut config = DiagramConfig::default();
    config.offset_method = OffsetMethod::Optimizing;
    test_venn("four.svg", constants::FOUR.into(), &mut config);
}

#[test]
fn five() {
    let mut config = DiagramConfig::default();
    config.offset_method = OffsetMethod::Optimizing;
    test_venn("five.svg", constants::FIVE.into(), &mut config);
}

#[test]
#[ignore = "expensive"]
fn eight() {
    let mut config = DiagramConfig::default();
    config.offset_method = OffsetMethod::Optimizing;
    test_venn("eight.svg", constants::EIGHT.into(), &mut config);
}
