use crate::style::{EdgeStyle, NodeStyle};

pub struct Node {
    id: usize,
    pub label: String,
    pub style: NodeStyle,
    pub position: (f64, f64),
}

pub struct Edge {
    pub from: usize,
    pub to: usize,
    pub style: EdgeStyle,
}

pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, label: String) -> usize {
        let idx = self.nodes.len();
        self.nodes.push(Node {
            id: idx,
            label,
            style: NodeStyle::default(),
            position: (0.0, 0.0),
        });
        idx
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.edges.push(Edge {
            from,
            to,
            style: EdgeStyle::default(),
        })
    }
}
