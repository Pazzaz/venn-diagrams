use venn_diagrams::{
    diagram::{Diagram, DiagramConfig},
    venn::{self, VennDiagram},
};

const COLORS: [&str; 8] =
    ["#EE2020", "#DDDD00", "#1B49DD", "#AF0000", "#009933", "#231977", "#83CF39", "#6BB7EC"];
const VALUES: [f64; 8] = [107.0, 73.0, 68.0, 24.0, 24.0, 19.0, 18.0, 16.0];

#[test]
fn eight() {
    let colors = COLORS;
    let values = normalize(&VALUES);
    test_venn("eight.svg", &venn::EIGHT.into(), &values, &colors, &DiagramConfig::default());
}

#[test]
fn six() {
    let colors = &COLORS[0..6];
    let values = normalize(&VALUES[0..6]);
    test_venn("six.svg", &venn::SIX.into(), &values, &colors, &DiagramConfig::default());
}

#[test]
fn five() {
    let colors = &COLORS[0..5];
    let values = normalize(&VALUES[0..5]);
    test_venn("five.svg", &venn::FIVE.into(), &values, &colors, &DiagramConfig::default());
}

#[test]
fn four() {
    let colors = &COLORS[0..4];
    let values = normalize(&VALUES[0..4]);
    test_venn("four.svg", &venn::FOUR.into(), &values, &colors, &DiagramConfig::default());
}

#[test]
fn three() {
    let colors = &COLORS[0..3];
    let values = normalize(&VALUES[0..3]);
    test_venn("three.svg", &venn::THREE.into(), &values, &colors, &DiagramConfig::default());
}

#[test]
fn two() {
    let colors = &COLORS[0..2];
    let values = normalize(&VALUES[0..2]);
    test_venn("two.svg", &venn::TWO.into(), &values, &colors, &DiagramConfig::default());
}

#[test]
fn four_wider() {
    let colors = &COLORS[0..4];
    let values = normalize(&VALUES[0..4]);
    let mut config = DiagramConfig::default();
    config.line_width = 2.0;
    test_venn("four_wide.svg", &venn::FOUR.into(), &values, &colors, &mut config);
}

fn normalize(values: &[f64]) -> Vec<f64> {
    let sum: f64 = values.iter().sum();
    values.iter().map(|x| x / sum).collect()
}

fn test_venn(
    name: &str,
    venn: &VennDiagram,
    values: &[f64],
    colors: &[&str],
    config: &DiagramConfig,
) {
    assert!(colors.len() == values.len());
    assert!(colors.len() == venn.n());

    let svg = Diagram::to_svg(
        venn,
        &values,
        &colors.iter().map(ToString::to_string).collect::<Vec<String>>(),
        config,
    );
    insta::assert_binary_snapshot!(name, svg.to_string().as_bytes().into_iter().cloned().collect());
}
