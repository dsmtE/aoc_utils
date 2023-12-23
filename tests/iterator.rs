use aoc_utils::iterator::IterUtils;
use std::collections::HashSet;

#[test]
fn iter() {
    let vec = vec![1, 2, 3, 4, 5];

    assert_eq!(vec.collect_string(", "), "1, 2, 3, 4, 5");
    assert_eq!(vec![1, 1, 2, 3, 3, 3, 4, 5, 5, 5, 5, 5].collect_hash_set(), HashSet::from_iter(vec![1, 2, 3, 4, 5]));
}