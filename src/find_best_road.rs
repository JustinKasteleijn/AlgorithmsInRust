use crate::dijkstra::dijkstra;
use crate::graph::{Edge, Graph};

type Vertex = u32;

pub fn build_example_graph() -> Graph {
    let mut graph = Graph::with_capacity(6);
    graph.add_edge(0, 1, 3);
    graph.add_edge(0, 2, 6);
    graph.add_edge(1, 3, 6);
    graph.add_edge(2, 4, 6);
    graph.add_edge(3, 5, 5);
    graph.add_edge(4, 5, 3);
    return graph;
}

pub fn build_road_example() -> Vec<(Vertex, Edge)> {
    vec![(1, Edge { to: 4, cost: 2 }), (2, Edge { to: 3, cost: 2 })]
}

pub fn find_best_route(
    graph: &Graph,
    source: Vertex,
    target: Vertex,
    roads: &Vec<(Vertex, Edge)>,
) -> Option<(u32, Edge, u32)> {
    let from_source = dijkstra(graph, source);
    let from_dest = dijkstra(&graph.flip(), target);

    println!("{:?}", from_source);
    println!("{:?}", from_dest);

    roads
        .iter()
        .map(|(from, edge)| {
            (
                *from,
                edge.clone(),
                from_source[*from as usize] + from_dest[edge.to as usize] + edge.cost,
            )
        })
        .min_by_key(|(_, _, value)| value.clone())
}
