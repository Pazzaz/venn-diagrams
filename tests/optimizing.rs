#![cfg(feature = "optimize")]

use venn_diagrams::{constants, diagram::Diagram};

mod common;

#[test]
fn two() {
    let diagram: Diagram = constants::d2::TWO.into();
    assert!(diagram.layout_optimize() == constants::d2::PATHLAYOUT_TWO_OPTIMIZING.into());
}

#[test]
fn three() {
    let diagram: Diagram = constants::d3::THREE.into();
    assert!(diagram.layout_optimize() == constants::d3::PATHLAYOUT_THREE_OPTIMIZING.into());
}

#[test]
fn four() {
    let diagram: Diagram = constants::d4::FOUR.into();
    assert!(diagram.layout_optimize() == constants::d4::PATHLAYOUT_FOUR_OPTIMIZING.into());
}

#[test]
fn five() {
    let diagram: Diagram = constants::d5::FIVE.into();
    assert!(diagram.layout_optimize() == constants::d5::PATHLAYOUT_FIVE_OPTIMIZING.into());
}

#[test]
#[ignore = "expensive"]
fn six() {
    let diagram: Diagram = constants::d6::SIX.into();
    assert!(diagram.layout_optimize() == constants::d6::PATHLAYOUT_SIX_OPTIMIZING.into());
}

#[test]
#[ignore = "expensive"]
fn seven() {
    let diagram: Diagram = constants::d7::SEVEN.into();
    assert!(diagram.layout_optimize() == constants::d7::PATHLAYOUT_SEVEN_OPTIMIZING.into());
}

#[test]
#[ignore = "expensive"]
fn eight() {
    let diagram: Diagram = constants::d8::EIGHT.into();
    assert!(diagram.layout_optimize() == constants::d8::PATHLAYOUT_EIGHT_OPTIMIZING.into());
}
