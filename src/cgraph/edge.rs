use super::Edge;
use std::fmt;
impl Edge {
    fn new(name: String) -> Edge {
        println!("new Edge");
        Edge {
            name: name,
            attributes: Vec::new()
            id: 0,

        }
    }
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Edge")
    }
}
