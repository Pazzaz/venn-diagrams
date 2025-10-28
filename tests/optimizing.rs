#![cfg(feature = "optimize")]

use venn_diagrams::{
    constants,
    svg::{OffsetMethod, PathLayout},
};

mod common;

#[test]
fn two() {
    let paths = PathLayout::from_diagram(constants::d2::TWO.into(), OffsetMethod::Optimizing);
    assert!(paths == constants::d2::PATHLAYOUT_TWO_OPTIMIZING.into());
}

#[test]
fn three() {
    let paths = PathLayout::from_diagram(constants::d3::THREE.into(), OffsetMethod::Optimizing);
    assert!(paths == constants::d3::PATHLAYOUT_THREE_OPTIMIZING.into());
}

#[test]
fn four() {
    let paths = PathLayout::from_diagram(constants::d4::FOUR.into(), OffsetMethod::Optimizing);
    assert!(paths == constants::d4::PATHLAYOUT_FOUR_OPTIMIZING.into());
}

#[test]
fn five() {
    let paths = PathLayout::from_diagram(constants::d5::FIVE.into(), OffsetMethod::Optimizing);
    assert!(paths == constants::d5::PATHLAYOUT_FIVE_OPTIMIZING.into());
}

#[test]
#[ignore = "expensive"]
fn eight() {
    let paths = PathLayout::from_diagram(constants::d8::EIGHT.into(), OffsetMethod::Optimizing);
    assert!(paths == constants::d8::PATHLAYOUT_EIGHT_OPTIMIZING.into());
}
