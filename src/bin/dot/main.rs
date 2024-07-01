
use graphr::gvc::ctx;
use graphr::cgraph;

fn main() {
    println!("Hello, world!");
    graphr::common::print();
    graphr::gvc::print();

    // load plugins if we decide that

    // get a global context
    let ctx = ctx::new();

    // parse command line args
    ctx.parse_args();

    // process all input graphs

    // for each graph
    //let graphs = ctx.next_input_graph();
    for g in ctx.next_input_graph() {
        // process graph
        println!("graph: {}", g);
        ctx.layout_jobs(g);
        ctx.render_jobs(g);
    }

    // finalize context

    // exit - set error code
    ctx.finalize();

    //testing

    let g = cgraph::Graph::new("testgraph".to_string(), cgraph::GraphKind::Directed);
    let n = cgraph::Node::new();
    let e = cgraph::Edge::new();
    let a = cgraph::Attribute::new();


    println!("g={}", g);
    println!("n={}", n);
    println!("e={}", e);
    println!("a={}", a);

}