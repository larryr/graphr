use graphr::cgraph;

fn main() {
    println!("xgr - graphr example");

    // create a digraph programmatically
    let mut g = cgraph::Graph::new("Sport".to_string(), cgraph::GraphKind::Directed);

    g.add_node("run".to_string());
    g.add_node("bike".to_string());
    g.add_node("tri".to_string());

    // connect nodes with edges
    g.add_edge("tri".to_string(), "run".to_string());
    g.add_edge("tri".to_string(), "bike".to_string());

    println!("dump graph description:\n{}", g);
}
