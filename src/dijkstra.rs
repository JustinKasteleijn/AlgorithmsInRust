use crate::graph::{Edge, Graph};

use std::cmp::Reverse;
use std::collections::BinaryHeap;

type Vertex = u32;

pub fn dijkstra(graph: &Graph, source: Vertex) -> Vec<u32> {
    let mut heap: BinaryHeap<Reverse<Edge>> = BinaryHeap::new();
    let mut distances: Vec<u32> = (0..graph.adj_list.len())
        .map(|_| u32::MAX)
        .collect::<Vec<u32>>();

    distances[source as usize] = 0;
    heap.push(Reverse(Edge {
        to: source,
        cost: 0,
    }));

    while let Some(Reverse(Edge { to, cost })) = heap.pop() {
        if cost > distances[to as usize] {
            continue;
        }

        for edge in graph.find_edges(to) {
            let next = Edge {
                cost: cost + edge.cost,
                to: edge.to,
            };

            if next.cost < distances[next.to as usize] {
                heap.push(Reverse(next));
                distances[next.to as usize] = next.cost
            }
        }
    }
    return distances;
}
