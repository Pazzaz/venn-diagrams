#![cfg(feature = "optimize")]

use venn_diagrams::{constants, diagram::Diagram};

mod common;

#[test]
fn two() {
    let diagram: Diagram = constants::d2::TWO.into();
    assert!(diagram.layout_optimize() == constants::d2::LAYOUT_OPTIMIZED_TWO.into());
}

#[test]
fn three() {
    let diagram: Diagram = constants::d3::THREE.into();
    assert!(diagram.layout_optimize() == constants::d3::LAYOUT_OPTIMIZED_THREE.into());
}

#[test]
fn four() {
    let diagram: Diagram = constants::d4::FOUR.into();
    assert!(diagram.layout_optimize() == constants::d4::LAYOUT_OPTIMIZED_FOUR.into());
}

#[test]
fn five() {
    let diagram: Diagram = constants::d5::FIVE.into();
    assert!(diagram.layout_optimize() == constants::d5::LAYOUT_OPTIMIZED_FIVE.into());
}

#[test]
#[ignore = "expensive"]
fn six() {
    let diagram: Diagram = constants::d6::SIX.into();
    assert!(diagram.layout_optimize() == constants::d6::LAYOUT_OPTIMIZED_SIX.into());
}

#[test]
#[ignore = "expensive"]
fn seven() {
    let diagram: Diagram = constants::d7::SEVEN.into();
    assert!(diagram.layout_optimize() == constants::d7::LAYOUT_OPTIMIZED_SEVEN.into());
}

#[test]
#[ignore = "expensive"]
fn eight() {
    let diagram: Diagram = constants::d8::EIGHT.into();
    assert!(diagram.layout_optimize() == constants::d8::LAYOUT_OPTIMIZED_EIGHT.into());
}
