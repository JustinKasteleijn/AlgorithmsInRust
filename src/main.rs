mod dijkstra;
mod find_best_road;
mod graph;

use crate::dijkstra::dijkstra;
use crate::find_best_road::*;
use crate::graph::Graph;

fn main() {
    let res = find_best_route(&build_example_graph(), 0, 5, &build_road_example());
    println!("{:?}", res);
}

fn simple_test_dijkstra() -> bool {
    let mut graph = Graph::with_capacity(5);
    graph.add_edge(0, 1, 1);
    graph.add_edge(0, 2, 4);
    graph.add_edge(1, 2, 2);
    graph.add_edge(1, 3, 6);
    graph.add_edge(2, 3, 3);
    let res = dijkstra(&mut graph, 0);
    return res == vec![0, 1, 3, 6, u32::MAX];
}
