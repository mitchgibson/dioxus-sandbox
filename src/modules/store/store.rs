use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::dot::{Dot, Config};

pub struct Store {
    pub store: DiGraph<String, String>
}