#[cfg(feature = "regex")]
mod regex;

pub mod iterator;
pub mod parsing;
pub mod nom_parsing;
pub mod cartesian;
pub mod dijkstra;

pub use aoc_utils_proc_macro::*;

mod grid;
pub use grid::*;

#[macro_export]
macro_rules! get_input { () => (
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/", env!("CARGO_BIN_NAME"), ".txt"))
)}