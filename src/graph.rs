type Vertex = u32;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Edge {
    pub to: Vertex,
    pub cost: u32,
}

pub struct Graph {
    pub adj_list: Vec<Vec<Edge>>,
}

impl Graph {
    pub fn with_capacity(n: usize) -> Graph {
        return Graph {
            adj_list: vec![Vec::new(); n],
        };
    }

    pub fn add_edge(&mut self, from: Vertex, to: Vertex, cost: u32) {
        self.adj_list[from as usize].push(Edge { to, cost })
    }

    pub fn find_edges(&self, from: Vertex) -> &[Edge] {
        &self.adj_list[from as usize]
    }

    pub fn flip(&self) -> Graph {
        let mut flipped = vec![Vec::new(); self.adj_list.len()];

        self.adj_list
            .iter()
            .enumerate()
            .flat_map(|(i, edges)| {
                edges.iter().map(move |edge| {
                    (
                        edge.to as usize,
                        Edge {
                            to: i as u32,
                            cost: edge.cost,
                        },
                    )
                })
            })
            .for_each(|(to, edge)| flipped[to].push(edge));

        return Graph { adj_list: flipped };
    }
}
