use std::io::{self, Read};
use super::Graph;

/// A Graph is the top level abastraction to hold a
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
    pub fn new() -> Graph {
        println!("Graph::new()");
        Graph {}
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

        Graph {}
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

