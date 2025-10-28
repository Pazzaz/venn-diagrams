use venn_diagrams::{
    svg::{DiagramConfig, PathLayout},
    venn::VennDiagram,
};

pub const COLORS: [&str; 8] =
    ["#EE2020", "#DDDD00", "#1B49DD", "#AF0000", "#009933", "#231977", "#83CF39", "#6BB7EC"];
pub const VALUES: [f64; 8] = [107.0, 73.0, 68.0, 24.0, 24.0, 19.0, 18.0, 16.0];

pub fn normalize(values: &[f64]) -> Vec<f64> {
    let sum: f64 = values.iter().sum();
    values.iter().map(|x| x / sum).collect()
}

pub fn test_venn(
    name: &str,
    venn: VennDiagram,
    values: &[f64],
    colors: &[&str],
    config: &DiagramConfig,
) {
    assert!(colors.len() == values.len());
    assert!(colors.len() == venn.n());

    let paths = PathLayout::from_diagram(venn, config.offset_method);

    let svg = paths.to_svg(
        &values,
        &colors.iter().map(ToString::to_string).collect::<Vec<String>>(),
        config,
    );
    insta::assert_binary_snapshot!(name, svg.to_string().as_bytes().into_iter().cloned().collect());
}
