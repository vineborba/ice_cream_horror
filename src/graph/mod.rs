pub mod bfs;
pub mod dfs;
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Vertex(u32);

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Edge(u32, u32);

#[derive(Clone)]
pub struct Graph {
    #[allow(dead_code)]
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
}

impl Graph {
    pub fn new(vertices: Vec<Vertex>, edges: Vec<Edge>) -> Self {
        Graph { vertices, edges }
    }
}

impl From<u32> for Vertex {
    fn from(item: u32) -> Self {
        Vertex(item)
    }
}

impl Vertex {
    pub fn value(&self) -> u32 {
        self.0
    }

    pub fn neighbors(&self, graph: &Graph) -> Vec<Vertex> {
        graph
            .edges
            .iter()
            .filter(|edge| edge.0 == self.0)
            .map(|edge| edge.1.into())
            .collect()
    }
}

impl From<(u32, u32)> for Edge {
    fn from(item: (u32, u32)) -> Self {
        Edge(item.0, item.1)
    }
}
