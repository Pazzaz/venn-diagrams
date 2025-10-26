use venn_diagrams::{
    svg::{DiagramConfig, OffsetMethod},
    venn,
};

use crate::common::{COLORS, VALUES, normalize, test_venn};

mod common;

#[test]
fn five() {
    let colors = &COLORS[0..5];
    let values = normalize(&VALUES[0..5]);
    let mut config = DiagramConfig::default();
    config.offset_method = OffsetMethod::Optimizing;
    test_venn("five.svg", &venn::FIVE.into(), &values, &colors, &mut config);
}
