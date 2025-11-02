use std::path::PathBuf;

use svg::node::element::SVG;
use venn_diagrams::{
    diagram::Diagram,
    svg::{DiagramConfig, Layout},
};

pub const COLORS: [&str; 8] =
    ["#EE2020", "#DDDD00", "#1B49DD", "#AF0000", "#009933", "#231977", "#83CF39", "#6BB7EC"];
pub const VALUES: [f64; 8] = [107.0, 73.0, 68.0, 24.0, 24.0, 19.0, 18.0, 16.0];

pub fn normalize(values: &[f64]) -> Vec<f64> {
    let sum: f64 = values.iter().sum();
    values.iter().map(|x| x / sum).collect()
}

// See: https://github.com/rust-lang/rust/issues/46379
#[allow(unused)]
pub fn test_venn_greedy(name: &str, venn: Diagram, config: &DiagramConfig) {
    let n = venn.n();
    let colors = &COLORS[0..n];
    let values = normalize(&VALUES[0..n]);
    let paths = venn.layout_greedy();

    let svg = paths.to_svg(&values, &colors, config);
    compare_snapshot(name, svg);
}

#[allow(unused)]
pub fn test_render_paths(name: &str, path_layout: Layout, config: &DiagramConfig) {
    let n = path_layout.n();
    let colors = &COLORS[0..n];
    let values = normalize(&VALUES[0..n]);
    let svg = path_layout.to_svg(&values, &colors, config);
    compare_snapshot(name, svg);
}

pub fn compare_snapshot(name: &str, svg: SVG) {
    let mut settings = insta::Settings::clone_current();
    let root_dir = std::env::var("CARGO_MANIFEST_DIR")
        .expect("Failed to retrieve value of CARGO_MANOFEST_DIR.");

    let mut path = PathBuf::from(&root_dir);
    path.push("snapshots");

    settings.set_snapshot_path(path);

    settings.bind(|| {
        insta::assert_binary_snapshot!(
            name,
            svg.to_string().as_bytes().into_iter().cloned().collect()
        );
    });
}
