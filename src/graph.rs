type Vertex = u32;

#[derive(Clone, Debug)]
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
        let mut rev_adj = vec![Vec::new(); self.adj_list.len()];

        for (u, edges) in self.adj_list.iter().enumerate() {
            for edge in edges {
                rev_adj[edge.to as usize].push(Edge {
                    to: u as u32,
                    cost: edge.cost,
                });
            }
        }

        Graph { adj_list: rev_adj }
    }
}
