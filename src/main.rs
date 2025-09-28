use venn_diagrams::{diagram::Diagram, venn};

fn main() {
    let colors_s =
        ["#EE2020", "#DDDD00", "#1B49DD", "#AF0000", "#009933", "#231977", "#83CF39", "#6BB7EC"];
    let values = normalize([0.303, 0.205, 0.191, 0.068, 0.067, 0.053, 0.051, 0.046]);
    let colors = colors_s.map(|x| x.to_string());
    let diagram = Diagram { venns: venn::EIGHT, values, colors };

    let svg = diagram.to_svg();
    svg::save("image.svg", &svg).unwrap();
}

fn normalize<const N: usize>(values: [f64; N]) -> [f64; N] {
    let sum: f64 = values.iter().sum();
    values.map(|x| x / sum)
}
