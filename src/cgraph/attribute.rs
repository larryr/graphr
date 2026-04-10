use super::Attribute;
use std::fmt;

impl Attribute {
    pub fn new(key: String, value: String) -> Attribute {
        Attribute { key, value }
    }
}

impl fmt::Display for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}={}", self.key, self.value)
    }
}
