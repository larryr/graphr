use super::Attribute;
use std::fmt;

impl Attribute {
    pub fn new() -> Attribute {
        println!("new Attribute");
        Attribute {}
    }
}

impl fmt::Display for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Attribute")
    }
}