use graphr::cgraph;


#[test]
fn test_structs() {
let g = cgraph::Graph::new();
let n = cgraph::Node::new();
let e = cgraph::Edge::new();
let a = cgraph::Attribute::new();


println!("g={:?}", g);
println!("n={}", n);
println!("e={}", e);
println!("a={}", a);

}