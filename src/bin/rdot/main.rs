
use std::io;
use graphr::cgraph;
fn main() {
    let mut stdin = io::stdin();
    let g = cgraph::Graph::parse(&mut stdin);
    println!("g={}", g);
}