mod graph;

pub struct Graph {
    name: String,
    kind: GraphKind,

    nodes: Vec<Node>,
    edges: Vec<Edge>,
    attributes: Vec<Attribute>
}

pub struct X;

pub struct Node {
    name: String,
    id: u64,
    attributes: Vec<Attribute>
}

pub struct Edge {
    name: String
}

pub struct Attribute;


// bring in sub modules
//pub mod graph;
//pub mod node;
//pub mod edge;
//pub mod attribute;
//pub mod dotlang;
//pub mod grammar;
// common definitions

#[derive(Debug)]
pub enum GraphKind {
    Directed,
    Undirected,
    StrictDirected,
    StrictUndirected
}

