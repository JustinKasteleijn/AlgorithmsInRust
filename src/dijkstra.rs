use crate::graph::Graph;

use std::cmp::Ordering;
use std::collections::BinaryHeap;

type Vertex = u32;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    position: u32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

pub fn dijkstra(graph: &Graph, source: Vertex) -> Vec<u32> {
    let mut heap = BinaryHeap::new();
    let mut distances = (0..graph.adj_list.len())
        .map(|_| u32::MAX)
        .collect::<Vec<u32>>();

    distances[source as usize] = 0;
    heap.push(State {
        position: source,
        cost: 0,
    });

    while let Some(State { position, cost }) = heap.pop() {
        if cost > distances[position as usize] {
            continue;
        }

        for edge in graph.find_edges(position) {
            let next = State {
                cost: cost + edge.cost,
                position: edge.to,
            };

            if next.cost < distances[next.position as usize] {
                heap.push(next);
                distances[next.position as usize] = next.cost
            }
        }
    }
    return distances;
}
