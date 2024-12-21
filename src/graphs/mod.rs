use std::{
    collections::HashMap,
    hash::Hash
};

pub mod dijkstra;
pub mod astar;

fn build_path<Node, V, F>(target: &Node, parents: &HashMap<Node, V>, mapping: F) -> Vec<Node>
where
    Node: Eq + Hash + Clone,
    F: Fn(&V) -> Option<&Node>,
{
    build_path_ref(target, parents, mapping).into_iter().cloned().collect()
}

fn build_path_ref<'a, Node, V, F>(target: &'a Node, parents: &'a HashMap<Node, V>, mapping: F) -> Vec<&'a Node>
where
    Node: Eq + Hash,
    F: Fn(&V) -> Option<&Node>,
{
    let mut current = target;
    let reverse_path = std::iter::from_fn(|| {
        parents.get(current).and_then(&mapping).map(|node| {
            current = node;
            node
        })
    })
    .collect::<Vec<&Node>>();

    reverse_path.into_iter().rev().collect()
}