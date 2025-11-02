// See: https://github.com/rust-lang/rust/issues/46379
#![allow(unused)]

use venn_diagrams::{
    diagram::Diagram,
    svg::{DiagramConfig, Layout},
};

// Our tests use colors and proportions from the Swedish 2022 election
pub const COLORS: [&str; 8] =
    ["#EE2020", "#DDDD00", "#1B49DD", "#AF0000", "#009933", "#231977", "#83CF39", "#6BB7EC"];
pub const VALUES: [f64; 8] = [107.0, 73.0, 68.0, 24.0, 24.0, 19.0, 18.0, 16.0];

pub fn normalize(values: &[f64]) -> Vec<f64> {
    let sum: f64 = values.iter().sum();
    values.iter().map(|x| x / sum).collect()
}

// We use a macro so that when `assert_binary_snapshot` adds the current module
// to the file name, it doesn't include this module.
macro_rules! compare_snapshot {
    ($name:expr, $svg:expr) => {
        use std::path::PathBuf;
        let mut settings = insta::Settings::clone_current();
        let root_dir = std::env::var("CARGO_MANIFEST_DIR")
            .expect("Failed to retrieve value of CARGO_MANOFEST_DIR.");

        let mut path = PathBuf::from(&root_dir);
        path.push("snapshots");

        settings.set_snapshot_path(path);

        settings.bind(|| {
            insta::assert_binary_snapshot!(
                $name,
                $svg.to_string().as_bytes().into_iter().cloned().collect()
            );
        });
    };
}
