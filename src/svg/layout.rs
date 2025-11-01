use svg::{
    Document,
    node::element::{Definitions, Mask, Rectangle, SVG},
};

use crate::{
    diagram::{Diagram, DiagramConst},
    direction::DirectedEdge,
    svg::{DiagramConfig, draw_circle, get_points, get_rounded_paths, inner_offset},
};

/// A Venn diagram with a computed layout of each polyomino border. For the
/// static version, see [`LayoutConst`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Layout {
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) combined_paths: Vec<Vec<DirectedEdge>>,
    pub(crate) offsets: Vec<Vec<i32>>,
    pub(crate) diagram: Diagram,
}

/// A Venn diagram with a computed layout of each polyomino border. For the
/// allocated version, see [`Layout`].
#[derive(Debug, Clone)]
pub struct LayoutConst<const L: usize, const K: usize, const X: usize, const Y: usize> {
    /// Edges of polyomino borders.
    pub combined_paths: [DirectedEdge; L],

    /// Offset positions of each edge, in every polyomino border.
    pub offsets: [i32; L],

    /// The number of edges of each polyomino. Sum of all values in `parts_len`
    /// must sum to `L`.
    pub parts_len: [usize; K],

    /// The Venn diagram itself.
    pub diagram: DiagramConst<K, X, Y>,
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

impl<const L: usize, const K: usize, const X: usize, const Y: usize> From<LayoutConst<L, K, X, Y>>
    for Layout
{
    fn from(value: LayoutConst<L, K, X, Y>) -> Self {
        let combined_paths =
            iterate(&value.combined_paths, &value.parts_len).map(|x| x.to_vec()).collect();
        let offsets = iterate(&value.offsets, &value.parts_len).map(|x| x.to_vec()).collect();
        let polyominoes = value.diagram.into();

        Self { width: X, height: Y, combined_paths, offsets, diagram: polyominoes }
    }
}

impl Layout {
    /// Number of sets/polyominos.
    pub const fn n(&self) -> usize {
        self.combined_paths.len()
    }

    /// Render as an SVG.
    #[must_use]
    pub fn to_svg(&self, values: &[f64], colors: &[&str], config: &DiagramConfig) -> SVG {
        let Self { width, height, combined_paths, offsets, diagram: polyominoes } = self;
        let internal_offsets =
            inner_offset(*width, *height, offsets, combined_paths, config.line_width);

        let points = get_points(
            *width,
            *height,
            combined_paths,
            offsets,
            config.line_width,
            config.corner_offset,
        );

        let paths = get_rounded_paths(&points, config.corner_style).unwrap();

        // Then we create the svg
        let min_x = -0.5;
        let total_width = (width + 1) as f64;

        let min_y = -0.5;
        let total_height = (height + 1) as f64;

        let mut out = Document::new().set("viewBox", (min_x, min_y, total_width, total_height));

        if let Some(width_mul) = config.width_mul {
            out = out.set("width", format!("{}px", width_mul * total_width));
        }

        if let Some(height_mul) = config.height_mul {
            out = out.set("height", format!("{}px", height_mul * total_width));
        }

        let mask_id = match config.id {
            Some(id) => &format!("background_mask_{id}"),
            None => "background_mask",
        };

        let mut mask = Mask::new().set("id", mask_id);

        for path in &paths {
            let part = path.clone().set("fill", "white").set("stroke", "none");
            mask = mask.add(part);
        }

        out = out.add(Definitions::new().add(mask));

        let rect = Rectangle::new()
            .set("width", total_width)
            .set("height", total_height)
            .set("x", min_x)
            .set("y", min_y)
            .set("mask", format!("url(#{mask_id})"));

        out = out.add(rect);

        for (path, color) in paths.iter().zip(colors) {
            let path = path
                .clone()
                .set("fill", *color)
                .set("fill-opacity", 0.2)
                .set("stroke", "none")
                .set("stroke-width", 0.05);
            out = out.add(path);
        }

        for (path, color) in paths.iter().zip(colors) {
            let path = path
                .clone()
                .set("fill", "none")
                .set("stroke", *color)
                .set("stroke-width", config.line_width);
            out = out.add(path);
        }

        // Draw the pie charts
        let mut pairs = vec![false; self.n()];
        for x in 0..*width {
            for y in 0..*height {
                let mut any_true = false;
                for i in 0..self.n() {
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
