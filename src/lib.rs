#[cfg(feature = "regex")]
mod regex;

pub mod iterator;
pub mod parsing;
pub mod nom_parsing;
pub mod cartesian;
pub mod graphs;
pub mod num;
mod grid;

use itertools::Itertools;

pub use aoc_utils_proc_macro::*;

use num::integer::Integer;

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
T:  Integer
{
    inner_area + boundary_points_count / T::TWO + T::ONE
}

pub fn hash_cycles<T, K: Eq + std::hash::Hash>(
    mut state: T,
    mut next: impl FnMut(T) -> T,
    mut key_fn: impl FnMut(&T) -> K,
    count: usize,
) -> T {
    let mut cache: std::collections::HashMap<K, usize> = std::collections::HashMap::new();
    for i in 0..count {
        let key = key_fn(&state);
        if let Some(&v) = cache.get(&key) {
            let cycle_length = i - v;
            let iters_left = count - i;
            let iters_after_cycles = iters_left % cycle_length;
            for _ in 0..iters_after_cycles {
                state = next(state);
            }
            return state;
        }
        cache.insert(key, i);
        state = next(state);
    }
    state
}
