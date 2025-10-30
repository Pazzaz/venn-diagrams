use svg::{
    Document,
    node::element::{Definitions, Mask, Rectangle, SVG},
};

use crate::{
    constants::{ConstVennDiagram, VennDiagram},
    direction::DirectedEdge,
    svg::{
        DiagramConfig, OffsetMethod, draw_circle, get_combined_paths, get_paths, get_points,
        get_polys, get_rounded_paths, inner_offset,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathLayout {
    pub(crate) x: usize,
    pub(crate) y: usize,
    pub(crate) combined_paths: Vec<Vec<DirectedEdge>>,
    pub(crate) offsets: Vec<Vec<i32>>,
    pub(crate) diagram: VennDiagram,
}

#[derive(Debug, Clone)]
pub struct PathLayoutConst<const L: usize, const K: usize, const X: usize, const Y: usize> {
    pub combined_paths: [DirectedEdge; L],
    pub offsets: [i32; L],
    pub parts_len: [usize; K],
    pub diagram: ConstVennDiagram<K, X, Y>,
}

struct PartsIterator<'a, T> {
    part: usize,
    start_i: usize,
    values: &'a [T],
    parts_len: &'a [usize],
}

fn iterate<'a, T>(values: &'a [T], parts_len: &'a [usize]) -> PartsIterator<'a, T> {
    PartsIterator { part: 0, start_i: 0, values, parts_len }
}

impl<'a, T> Iterator for PartsIterator<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.part >= self.parts_len.len() {
            return None;
        }
        let part_len = self.parts_len[self.part];

        let next = &self.values[self.start_i..(self.start_i + part_len)];
        self.start_i += part_len;
        self.part += 1;

        Some(next)
    }
}

impl<const L: usize, const K: usize, const X: usize, const Y: usize>
    From<PathLayoutConst<L, K, X, Y>> for PathLayout
{
    fn from(value: PathLayoutConst<L, K, X, Y>) -> Self {
        let combined_paths =
            iterate(&value.combined_paths, &value.parts_len).map(|x| x.to_vec()).collect();
        let offsets = iterate(&value.offsets, &value.parts_len).map(|x| x.to_vec()).collect();
        let polyominoes = value.diagram.into();

        PathLayout { x: X, y: Y, combined_paths, offsets, diagram: polyominoes }
    }
}

impl PathLayout {
    pub const fn n(&self) -> usize {
        self.combined_paths.len()
    }

    pub fn from_diagram(diagram: VennDiagram, offset_method: OffsetMethod) -> Self {
        let polys = get_polys(diagram.x(), diagram.y(), &diagram.polyominos);
        let paths = get_paths(&polys);
        let combined_paths = get_combined_paths(paths);
        let offsets = offset_method.get_offsets(diagram.x(), diagram.y(), &combined_paths);

        Self { x: diagram.x(), y: diagram.y(), combined_paths, offsets, diagram }
    }

    #[must_use]
    pub fn to_svg(&self, values: &[f64], colors: &[String], config: &DiagramConfig) -> SVG {
        let PathLayout { x, y, combined_paths, offsets, diagram: polyominoes } = self;
        let internal_offsets = inner_offset(*x, *y, offsets, combined_paths, config.line_width);

        let points =
            get_points(*x, *y, combined_paths, offsets, config.line_width, config.corner_offset);

        let paths = get_rounded_paths(&points, config.corner_style).unwrap();

        // Then we create the svg
        let min_x = -0.5;
        let width = (x + 1) as f64;

        let min_y = -0.5;
        let height = (y + 1) as f64;

        let mut out = Document::new().set("viewBox", (min_x, min_y, width, height));

        if let Some(width_mul) = config.width_mul {
            out = out.set("width", format!("{}px", width_mul * width));
        }

        if let Some(height_mul) = config.height_mul {
            out = out.set("height", format!("{}px", height_mul * width));
        }

        let mut mask = Mask::new().set("id", "background_mask");
        for path in &paths {
            let part = path.clone().set("fill", "white").set("stroke", "none");
            mask = mask.add(part);
        }

        let defs = Definitions::new().add(mask);

        out = out.add(defs);

        let rect = Rectangle::new()
            .set("width", width)
            .set("height", height)
            .set("x", min_x)
            .set("y", min_y)
            .set("mask", "url(#background_mask)");

        out = out.add(rect);

        for (path, color) in paths.iter().zip(colors) {
            let path = path
                .clone()
                .set("fill", color.clone())
                .set("fill-opacity", 0.2)
                .set("stroke", "none")
                .set("stroke-width", 0.05);
            out = out.add(path);
        }

        for (path, color) in paths.iter().zip(colors) {
            let path = path
                .clone()
                .set("fill", "none")
                .set("stroke", color.clone())
                .set("stroke-width", config.line_width);
            out = out.add(path);
        }

        let n = self.n();

        let mut pairs = vec![false; n];
        for x in 0..*x {
            for y in 0..*y {
                let mut any_true = false;
                for i in 0..n {
                    let v = polyominoes.polyominos[i][(x, y)];
                    any_true |= v;
                    pairs[i] = v;
                }
                if any_true {
                    let (x_pos, y_pos) =
                        config.circle_placement.get_circle_pos(x, y, internal_offsets[(x, y)]);
                    out = draw_circle(x_pos, y_pos, &pairs, out, config, values, colors);
                }
            }
        }

        out
    }
}
