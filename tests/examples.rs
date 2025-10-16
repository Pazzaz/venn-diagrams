use venn_diagrams::{
    diagram::{Diagram, DiagramConfig},
    venn,
};

#[test]
fn eight() {
    fn normalize<const N: usize>(values: [f64; N]) -> [f64; N] {
        let sum: f64 = values.iter().sum();
        values.map(|x| x / sum)
    }
    let colors_s =
        ["#EE2020", "#DDDD00", "#1B49DD", "#AF0000", "#009933", "#231977", "#83CF39", "#6BB7EC"];
    let values = normalize([107.0, 73.0, 68.0, 24.0, 24.0, 19.0, 18.0, 16.0]);
    let colors = colors_s.map(ToString::to_string);

    let svg = Diagram::to_svg(&venn::EIGHT.into(), &values, &colors, &DiagramConfig::default());
    insta::assert_binary_snapshot!(
        "eight.svg",
        svg.to_string().as_bytes().into_iter().cloned().collect()
    );
}
