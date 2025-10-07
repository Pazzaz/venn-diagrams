use venn_diagrams::{
    diagram::{CirclePlacement, Diagram},
    venn,
};

fn main() {
    let colors_s =
        ["#EE2020", "#DDDD00", "#1B49DD", "#AF0000", "#009933", "#231977", "#83CF39", "#6BB7EC"];
    let values = normalize([107.0, 73.0, 68.0, 24.0, 24.0, 19.0, 18.0, 16.0]);
    let colors = colors_s.map(|x| x.to_string());
    let diagram = Diagram {
        venns: venn::EIGHT,
        values,
        colors,
        radius: 3.5,
        opacity_below: 0.3,
        opacity_edge: 1.0,
        opacity_above: 0.3,
        circle_placement: CirclePlacement::SquareCenter,
    };

    let svg = diagram.to_svg();
    svg::save("image.svg", &svg).unwrap();
}

fn normalize<const N: usize>(values: [f64; N]) -> [f64; N] {
    let sum: f64 = values.iter().sum();
    values.map(|x| x / sum)
}
