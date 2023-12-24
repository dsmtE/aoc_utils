#[cfg(feature = "regex")]
mod regex;

pub mod iterator;
pub mod parsing;
pub mod nom_parsing;
pub mod cartesian;
pub mod dijkstra;

use itertools::Itertools;

pub use aoc_utils_proc_macro::*;

mod grid;
pub use grid::*;

#[macro_export]
macro_rules! get_input { () => (
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/", env!("CARGO_BIN_NAME"), ".txt"))
)}

pub fn shoelace_area(points: &[(i64, i64)]) -> i64 {
    points.into_iter().circular_tuple_windows().map(|((x1, y1), (x2, y2))| {
        (x2 * y1) - (x1 * y2)
    }).sum::<i64>().abs() / 2i64
}

// Pick's theorem using shoelace formula
pub fn pick_area<T>(inner_area: T, boundary_points_count: T) -> T 
where
T: num::Integer
{
    inner_area + boundary_points_count / (T::one() + T::one()) + T::one()
}