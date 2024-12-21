
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;

use crate::num::constant::Zero;

use super::build_path;

pub fn dijkstra<Node, Cost, FN, IN, FS>(
    starts: impl IntoIterator<Item = Node>,
    mut successors: FN,
    mut success: FS,
) -> Option<(Vec<Node>, Cost)>
where
    Node: Eq + Hash + Clone,
    Cost: Zero + Ord + Copy,
    FN: FnMut(&Node) -> IN,
    IN: IntoIterator<Item = (Node, Cost)>,
    FS: FnMut(&Node) -> bool,
{
    let (parents, reached) = run_dijkstra(starts, &mut successors, &mut success);
    reached.map(|target| {
        (
            build_path(&target.clone(), &parents, |(p, _)| p.as_ref()),
            parents[&target].1,
        )
    })
}

fn run_dijkstra<Node, Cost, FN, IN, FS>(
    starts: impl IntoIterator<Item = Node>,
    successors: &mut FN,
    stop: &mut FS,
) -> (HashMap<Node, (Option<Node>, Cost)>, Option<Node>)
where
    Node: Eq + Hash + Clone,
    Cost: Zero + Ord + Copy,
    FN: FnMut(&Node) -> IN,
    IN: IntoIterator<Item = (Node, Cost)>,
    FS: FnMut(&Node) -> bool,
{
    let mut to_visit = BinaryHeap::new();
    let mut parent: HashMap<Node, (Option<Node>, Cost)> = HashMap::default();
    
    for start in starts {
        to_visit.push(SmallestHolder{cost: Cost::ZERO, data: start.clone()});
        parent.insert(start.clone(), (None, Cost::ZERO));
    }

    let mut target_reached = None;

    while let Some(SmallestHolder { cost, data: node }) = to_visit.pop() {
        if stop(&node) {
            target_reached = Some(node);
            break;
        }

        for (neighbor, move_cost) in successors(&node) {
            let new_cost = cost + move_cost;

            let is_empty_or_less_costly = parent
                    .get(&neighbor)
                    .map_or(true, |&(_, current_cost)| new_cost < current_cost);

            // If we already have a cheaper path to `node`, do nothing.
            if is_empty_or_less_costly {
                parent.insert(neighbor.clone(), (Some(node.clone()), new_cost));
                to_visit.push(SmallestHolder{cost: new_cost, data: neighbor});
            }
        }
    }
    (parent, target_reached)
}

struct SmallestHolder<Cost, T> {
    cost: Cost,
    data: T,
}

impl<Cost: PartialEq, T> PartialEq for SmallestHolder<Cost, T> {
    fn eq(&self, other: &Self) -> bool { self.cost.eq(&other.cost) }
}

impl<Cost: PartialEq, T> Eq for SmallestHolder<Cost, T> {}

impl<Cost: Ord, T> PartialOrd for SmallestHolder<Cost, T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<Cost: Ord, T> Ord for SmallestHolder<Cost, T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}