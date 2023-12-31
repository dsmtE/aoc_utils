
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;
use std::cmp::Ordering;

use num_traits::Zero;

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
            build_path(&target.clone(), &parents, |(p, _)| p.clone()),
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
        to_visit.push(SmallestHolder {
            cost: Zero::zero(),
            node: start.clone(),
        });
        parent.insert(start.clone(), (None, Zero::zero()));
    }

    let mut target_reached = None;

    while let Some(SmallestHolder { cost, node }) = to_visit.pop() {
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
                to_visit.push(SmallestHolder {
                    cost: new_cost,
                    node: neighbor,
                });
            }
        }
    }
    (parent, target_reached)
}

fn build_path<Node, V, F>(target: &Node, parents: &HashMap<Node, V>, mapping: F) -> Vec<Node>
where
    Node: Eq + Hash + Clone,
    F: Fn(&V) -> Option<Node>,
{
    let mut reverse_path = vec![target.clone()];

    while let Some(node) = parents.get(reverse_path.last().unwrap()).and_then(&mapping) {
        reverse_path.push(node.clone());
    }

    reverse_path.reverse();
    reverse_path
}

struct SmallestHolder<Cost, Node> {
    cost: Cost,
    node: Node,
}

impl<Cost: PartialEq, Node> PartialEq for SmallestHolder<Cost, Node> {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl<Cost: PartialEq, Node> Eq for SmallestHolder<Cost, Node> {}

impl<Cost: Ord, Node> PartialOrd for SmallestHolder<Cost, Node> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<Cost: Ord, Node> Ord for SmallestHolder<Cost, Node> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}