use graphr::gvc::ctx;

fn main() {
    // get a context and parse command-line args
    let mut ctx = ctx::new();
    ctx.parse_args();

    // process all input graphs
    let mut graphs = ctx.next_input_graph();

    if graphs.is_empty() {
        eprintln!("No graphs found in input");
        std::process::exit(1);
    }

    for g in &mut graphs {
        ctx.layout_jobs(g);
        ctx.render_jobs(g);
    }
}
