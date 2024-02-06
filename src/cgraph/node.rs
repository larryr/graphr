use super::Node;
use std::fmt;

impl Node {
    pub fn new() -> Node {
        println!("new Node");
        Node {}
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Node")
    }
}