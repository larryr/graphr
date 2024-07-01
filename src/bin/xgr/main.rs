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
    let n = cgraph::Node::new();

    println!("g={}", g);
}