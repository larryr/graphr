use super::{Node, Attribute};
use std::fmt;

impl Node {
    pub fn new(name: String) -> Node {
        Node {
            name,
            id: 0,
            attributes: Vec::new(),
        }
    }

    /// Set an attribute on this node.
    pub fn set_attr(&mut self, key: String, value: String) {
        if let Some(attr) = self.attributes.iter_mut().find(|a| a.key == key) {
            attr.value = value;
        } else {
            self.attributes.push(Attribute::new(key, value));
        }
    }

    /// Get an attribute value by key.
    pub fn get_attr(&self, key: &str) -> Option<&str> {
        self.attributes.iter()
            .find(|a| a.key == key)
            .map(|a| a.value.as_str())
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Node({})", self.name)
    }
}
