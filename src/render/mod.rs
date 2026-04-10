//! Render module.
//!
//! Provides the `Render` trait and implementations for outputting
//! graph descriptions in various formats.

use std::io::Write;
use crate::cgraph::Graph;

/// Trait for graph renderers.
pub trait Render {
    /// Render a graph to the given writer.
    fn render(&self, g: &Graph, out: &mut dyn Write) -> std::io::Result<()>;
}

/// Renders graph in DOT format (re-emit).
pub struct DotRenderer;

impl DotRenderer {
    pub fn new() -> DotRenderer {
        DotRenderer
    }
}

impl Render for DotRenderer {
    fn render(&self, g: &Graph, out: &mut dyn Write) -> std::io::Result<()> {
        write!(out, "{}", g)
    }
}

/// Renders graph in Graphviz "plain" text format.
///
/// Format:
///   graph scale width height
///   node name x y width height label style shape color fillcolor
///   edge tail head n x1 y1 ... xn yn [label xl yl] style color
///   stop
pub struct PlainRenderer;

impl PlainRenderer {
    pub fn new() -> PlainRenderer {
        PlainRenderer
    }
}

impl Render for PlainRenderer {
    fn render(&self, g: &Graph, out: &mut dyn Write) -> std::io::Result<()> {
        // Compute bounding box
        let mut max_x: f64 = 0.0;
        let mut max_y: f64 = 0.0;

        for node in &g.nodes {
            if let Some(pos) = node.get_attr("pos") {
                let parts: Vec<&str> = pos.split(',').collect();
                if parts.len() == 2 {
                    if let (Ok(x), Ok(y)) = (parts[0].parse::<f64>(), parts[1].parse::<f64>()) {
                        let w: f64 = node.get_attr("width")
                            .and_then(|v| v.parse().ok())
                            .unwrap_or(0.75);
                        let h: f64 = node.get_attr("height")
                            .and_then(|v| v.parse().ok())
                            .unwrap_or(0.5);
                        if x + w > max_x { max_x = x + w; }
                        if y + h > max_y { max_y = y + h; }
                    }
                }
            }
        }

        writeln!(out, "graph 1.0 {:.3} {:.3}", max_x / 72.0, max_y / 72.0)?;

        // Nodes
        for node in &g.nodes {
            let label = node.get_attr("label").unwrap_or(&node.name);
            let x: f64 = node.get_attr("pos")
                .and_then(|p| p.split(',').next()?.parse().ok())
                .unwrap_or(0.0) / 72.0;
            let y: f64 = node.get_attr("pos")
                .and_then(|p| p.split(',').nth(1)?.parse().ok())
                .unwrap_or(0.0) / 72.0;
            let w: f64 = node.get_attr("width")
                .and_then(|v| v.parse().ok())
                .unwrap_or(0.75);
            let h: f64 = node.get_attr("height")
                .and_then(|v| v.parse().ok())
                .unwrap_or(0.5);
            let shape = node.get_attr("shape").unwrap_or("ellipse");

            writeln!(out, "node {} {:.4} {:.4} {:.4} {:.4} {} solid {} black lightgrey",
                node.name, x, y, w, h, label, shape)?;
        }

        // Edges
        for edge in &g.edges {
            let label = edge.get_attr("label").unwrap_or("");
            if label.is_empty() {
                writeln!(out, "edge {} {} 0 solid black", edge.from, edge.to)?;
            } else {
                writeln!(out, "edge {} {} 0 {} solid black", edge.from, edge.to, label)?;
            }
        }

        writeln!(out, "stop")?;
        Ok(())
    }
}
