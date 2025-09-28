#[macro_use]
extern crate static_assertions;

mod direction;

pub mod diagram;
pub mod venn;

pub type Polyomino<const X: usize, const Y: usize> = [[bool; X]; Y];
