mod graph;
mod node;
mod edge;
mod attribute;
pub mod ast;

#[allow(clippy::all)]
#[allow(unused)]
pub mod grammar;

#[derive(Debug)]
pub enum GraphKind {
    Directed,
    Undirected,
    StrictDirected,
    StrictUndirected,
}

pub struct Graph {
    pub name: String,
    pub kind: GraphKind,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub sub_graphs: Vec<Graph>,
    pub attributes: Vec<Attribute>,
}

pub struct Node {
    pub name: String,
    pub id: u64,
    pub attributes: Vec<Attribute>,
}

pub struct Edge {
    pub name: String,
    pub from: String,
    pub to: String,
    pub id: u64,
    pub attributes: Vec<Attribute>,
}

pub struct Attribute {
    pub key: String,
    pub value: String,
}

