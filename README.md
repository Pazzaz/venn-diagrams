<div align="center">

# Venn Diagrams
[![Crates.io](https://img.shields.io/crates/v/venn-diagrams.svg)](https://crates.io/crates/venn-diagrams)
![Example diagram](tests/common/snapshots/render_optimize__common__eight.snap.svg)

</div>

Program to illustrate data with Venn diagrams. Originally created to show possible coalitions in parliamentary democracies.

# Optimization
To optimize the layout of venn diagrams (minimizing overlapping edges and gaps), enable the [feature flag](https://doc.rust-lang.org/cargo/reference/features.html) `optimize`. The crate uses the [z3](https://en.wikipedia.org/wiki/Z3_Theorem_Prover) SMT solver. For more information about how to install z3, see the [z3 crate](https://crates.io/crates/z3).

# Development
This library uses [Insta](https://docs.rs/insta/latest/insta/index.html) for snapshot tests.