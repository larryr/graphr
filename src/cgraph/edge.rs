use super::{Edge, Attribute};
use std::fmt;

impl Edge {
    pub fn new(name: String, from: String, to: String) -> Edge {
        Edge {
            name,
            from,
            to,
            id: 0,
            attributes: Vec::new(),
        }
    }

    /// Set an attribute on this edge.
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

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Edge({}: {} -> {})", self.name, self.from, self.to)
    }
}
