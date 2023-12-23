use aoc_utils::dijkstra::dijkstra;

#[test]
fn test_dijkstra() {
    let mut graph = std::collections::HashMap::new();

    graph.insert('A', vec![('B', 3), ('C', 2)]);
    graph.insert('B', vec![('C', 5), ('D', 2), ('G', 7)]);
    graph.insert('C', vec![('E', 2), ('F', 9)]);
    graph.insert('D', vec![('E', 8), ('F', 1)]);
    graph.insert('E', vec![('G', 3)]);
    graph.insert('F', vec![('G', 6), ('H', 7), ('K', 8)]);
    graph.insert('G', vec![('I', 6), ('J', 9)]);
    graph.insert('H', vec![('I', 7), ('J', 2)]);
    graph.insert('I', vec![('K', 4)]);
    graph.insert('J', vec![('K', 6), ('L', 4)]);
    graph.insert('K', vec![('L', 5)]);
    graph.insert('L', vec![]);

    let (path, cost) = dijkstra(['A'], |node| graph.get(node).unwrap().clone(), |node| *node == 'L').unwrap();
    assert_eq!(path, vec!['A', 'B', 'D', 'F', 'K', 'L']);
    assert_eq!(cost, 19);

    let (path, cost) = dijkstra(['C'], |node| graph.get(node).unwrap().clone(), |node| *node == 'J').unwrap();
    assert_eq!(path, vec!['C', 'E', 'G', 'J']);
    assert_eq!(cost, 14);
}
