use graphr::gvc::ctx;
use graphr::cgraph;

fn main() {
    println!("xgr - graphr example");
    graphr::common::print();
    graphr::gvc::print();

    // load plugins if we decide that

    // get a global context
    let ctx = ctx::new();

    // parse command line args
    ctx.parse_args();

    // create a digraph
    let g = cgraph::Graph::new("Sport".to_string(), cgraph::GraphKind::Directed);

    let node_run = cgraph::Node::new();
    let node_bike = cgraph::Node::new();
    let node_tri  = cgraph::Node::new();

    // connect nodes with edges
    let edge_tri_run = cgraph::Edge::new(&node_tri, &node_run);
    let edge_tri_bike = cgraph::Edge::new(&node_tri, &node_bike);

    // add nodes to graph
    g.add_node(node_tri);

    println!("dump graph description:\ng={}", g);
}