use super::Edge;
use std::fmt;
impl Edge {
    pub fn new() -> Edge {
        println!("new Edge");
        Edge {}
    }
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Edge")
    }
}
