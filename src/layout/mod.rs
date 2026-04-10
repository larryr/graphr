//! Layout engine module.
//!
//! Provides the `Layout` trait and implementations for positioning
//! graph nodes and edges in 2D space.

use crate::cgraph::Graph;

/// Trait for graph layout algorithms.
pub trait Layout {
    /// Compute positions for all nodes in the graph.
    fn layout(&self, g: &mut Graph);
}

/// Simple hierarchical layout (Sugiyama-style stub).
///
/// Assigns basic x,y coordinates to nodes by rank.
/// This is a foundation for the full dot-style hierarchical layout.
pub struct DotLayout;

impl DotLayout {
    pub fn new() -> DotLayout {
        DotLayout
    }
}

impl Layout for DotLayout {
    fn layout(&self, g: &mut Graph) {
        // Phase 1: Assign ranks (topological layers)
        // For now, assign simple sequential positions
        let node_count = g.node_count();
        if node_count == 0 {
            return;
        }

        // Simple rank assignment: nodes appear in order of discovery
        // Edges go from lower rank to higher rank
        let spacing_x = 72.0; // points
        let spacing_y = 72.0;

        for (i, node) in g.nodes.iter_mut().enumerate() {
            let x = spacing_x;
            let y = (i as f64) * spacing_y;
            node.set_attr("pos".to_string(), format!("{},{}", x, y));
            // Default dimensions if not set
            if node.get_attr("width").is_none() {
                node.set_attr("width".to_string(), "0.75".to_string());
            }
            if node.get_attr("height").is_none() {
                node.set_attr("height".to_string(), "0.5".to_string());
            }
        }
    }
}
