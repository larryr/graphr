use std::fmt;
use std::io::{self, Read};
use super::{Graph, GraphKind, Node};

/// A Graph is the top level abstraction to hold
/// the graph definition and attributes.
///
/// Each graph has a collection of:
/// - Nodes
/// - Edges
/// - Sub-Graphs
/// - Attributes
///
/// A Graph can be constructed by reading a graph descriptor
/// defined in the **DOT** language, or constructed by a series
/// of API calls.
///
/// A Graph can be rendered to a byte-stream in a variety of formats.
///
impl Graph {

    pub fn new(name: String, kind: GraphKind) -> Graph {
        println!("Graph::new()");
        Graph {
            name,
            kind,
            nodes: Vec::new(),
            edges: Vec::new(),
            attributes: Vec::new()
        }
    }

    /// Parse a graph descriptor in the **DOT** language.
    /// Returns a newly created Graph object.
    ///
    pub fn parse(reader : &mut dyn io::Read) -> Graph
       {
        println!("Graph::parse(): parse a byte stream in the DOT language");

        // copy dot to stdout
        let mut stdout = io::stdout();
        io::copy(reader, &mut stdout).unwrap();

        Graph {
            name: "no-dot".to_string(),
            kind: GraphKind::Directed,
            nodes: Vec::new(),
            edges: Vec::new(),
            attributes: Vec::new()
        }
    }

    //
    // Graph API for adding nodes, edges, sub-graphs, and attributes
    //

    // add node to graph.
    pub fn add_node(&mut self, name : String) -> Node {
        println!("Graph::add_node");
        // check if node-name exists; add if not
        if ! self.nodes.iter().any(|n| n.name == name) {
            let node = Node::new(name);
            self.nodes.push(node);
            return node;
        }
        // return an error

    }





    pub fn nodes(&self) -> u32 {
        println!("Graph::nodes");
        0
    }

    pub fn edges(&self) -> u32 {
        println!("Graph::edges");
        0
    }

    pub fn sub_graphs(&self) -> u32 {
        println!("Graph::sub_graphs");
        0
    }

    pub fn is_directed(&self) -> bool {
        println!("Graph::is_directed");
        false
    }

    pub fn is_strict(&self) -> bool {
        println!("Graph::is_strict");
        false
    }

    pub fn is_simple(&self) -> bool {
        println!("Graph::is_simple");
        false
    }

}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Graph: {} kind: {:?}", self.name, self.kind)
    }
}