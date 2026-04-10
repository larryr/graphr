
use std::io;
use graphr::cgraph;

fn main() {
    let mut stdin = io::stdin();
    match cgraph::Graph::parse(&mut stdin) {
        Ok(g) => println!("{}", g),
        Err(e) => eprintln!("Error: {}", e),
    }
}
