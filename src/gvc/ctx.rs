
use std::env;
use std::fs::File;
use std::io::{self, Write};
use crate::cgraph::Graph;
use crate::layout::{Layout, DotLayout};
use crate::render::{Render, DotRenderer, PlainRenderer};

/// Graphviz Context — manages the graph processing pipeline.
pub struct Ctx {
    input_files: Vec<String>,
    output_file: Option<String>,
    output_format: String,
}

pub fn new() -> Ctx {
    Ctx {
        input_files: Vec::new(),
        output_file: None,
        output_format: "dot".to_string(),
    }
}

impl Ctx {
    pub fn print(&self) {
        println!("Ctx: format={}, inputs={:?}, output={:?}",
            self.output_format, self.input_files, self.output_file);
    }

    /// Parse command-line arguments.
    /// Supports: -T<format>, -o <file>, and input files.
    pub fn parse_args(&mut self) {
        let args: Vec<String> = env::args().skip(1).collect();
        let mut i = 0;
        while i < args.len() {
            let arg = &args[i];
            if arg.starts_with("-T") {
                self.output_format = arg[2..].to_string();
            } else if arg == "-o" {
                i += 1;
                if i < args.len() {
                    self.output_file = Some(args[i].clone());
                }
            } else if !arg.starts_with('-') {
                self.input_files.push(arg.clone());
            }
            i += 1;
        }
    }

    /// Read and parse all input graphs.
    /// If no input files specified, reads from stdin.
    pub fn next_input_graph(&self) -> Vec<Graph> {
        let mut graphs = Vec::new();

        if self.input_files.is_empty() {
            // Read from stdin
            let mut stdin = io::stdin();
            match Graph::parse(&mut stdin) {
                Ok(g) => graphs.push(g),
                Err(e) => eprintln!("Error parsing stdin: {}", e),
            }
        } else {
            for path in &self.input_files {
                match File::open(path) {
                    Ok(mut f) => {
                        match Graph::parse(&mut f) {
                            Ok(g) => graphs.push(g),
                            Err(e) => eprintln!("Error parsing {}: {}", path, e),
                        }
                    }
                    Err(e) => eprintln!("Error opening {}: {}", path, e),
                }
            }
        }

        graphs
    }

    /// Run layout on a graph.
    pub fn layout_jobs(&self, g: &mut Graph) {
        let layout = DotLayout::new();
        layout.layout(g);
    }

    /// Render a graph to the configured output.
    pub fn render_jobs(&self, g: &Graph) {
        let mut output: Box<dyn Write> = match &self.output_file {
            Some(path) => {
                match File::create(path) {
                    Ok(f) => Box::new(f),
                    Err(e) => {
                        eprintln!("Error creating {}: {}", path, e);
                        return;
                    }
                }
            }
            None => Box::new(io::stdout()),
        };

        let result = match self.output_format.as_str() {
            "plain" => PlainRenderer::new().render(g, &mut output),
            "dot" | _ => DotRenderer::new().render(g, &mut output),
        };

        if let Err(e) = result {
            eprintln!("Error rendering: {}", e);
        }
    }

    pub fn finalize(&self) {
        // Cleanup — currently a no-op
    }
}
