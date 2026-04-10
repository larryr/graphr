use std::fmt;
use std::io;
use super::{Graph, GraphKind, Node, Edge, Attribute};
use super::ast::*;
use super::grammar;

/// A Graph is the top level abstraction to hold
/// the graph definition and attributes.
///
/// Each graph has a collection of:
/// - Nodes
/// - Edges
/// - Sub-Graphs
/// - Attributes
///
/// A Graph can be constructed by reading a graph descriptor
/// defined in the **DOT** language, or constructed by a series
/// of API calls.
///
/// A Graph can be rendered to a byte-stream in a variety of formats.
///
impl Graph {

    pub fn new(name: String, kind: GraphKind) -> Graph {
        Graph {
            name,
            kind,
            nodes: Vec::new(),
            edges: Vec::new(),
            sub_graphs: Vec::new(),
            attributes: Vec::new(),
        }
    }

    /// Parse a graph descriptor in the **DOT** language.
    /// Returns a newly created Graph object.
    pub fn parse(reader: &mut dyn io::Read) -> Result<Graph, String> {
        let mut input = String::new();
        reader.read_to_string(&mut input)
            .map_err(|e| format!("Failed to read input: {}", e))?;

        let ast = grammar::GraphParser::new()
            .parse(&input)
            .map_err(|e| format!("Parse error: {}", e))?;

        Ok(Graph::from_ast(&ast))
    }

    /// Parse a DOT string directly.
    pub fn parse_str(input: &str) -> Result<Graph, String> {
        let ast = grammar::GraphParser::new()
            .parse(input)
            .map_err(|e| format!("Parse error: {}", e))?;

        Ok(Graph::from_ast(&ast))
    }

    /// Build a Graph from an AST.
    fn from_ast(ast: &AstGraph) -> Graph {
        let kind = match (ast.strict, ast.directed) {
            (false, false) => GraphKind::Undirected,
            (false, true) => GraphKind::Directed,
            (true, false) => GraphKind::StrictUndirected,
            (true, true) => GraphKind::StrictDirected,
        };

        let name = ast.id.clone().unwrap_or_default();
        let mut graph = Graph::new(name, kind);

        graph.process_stmts(&ast.stmts);
        graph
    }

    /// Process a list of AST statements, populating the graph.
    fn process_stmts(&mut self, stmts: &[AstStmt]) {
        for stmt in stmts {
            match stmt {
                AstStmt::Node { id, attrs } => {
                    let idx = self.add_node(id.name.clone());
                    for attr in attrs {
                        self.nodes[idx].attributes.push(
                            Attribute::new(attr.key.clone(), attr.value.clone())
                        );
                    }
                }
                AstStmt::Edge { endpoints, attrs } => {
                    self.process_edge_stmt(endpoints, attrs);
                }
                AstStmt::Attr { target, attrs } => {
                    match target {
                        AstAttrTarget::Graph => {
                            for attr in attrs {
                                self.attributes.push(
                                    Attribute::new(attr.key.clone(), attr.value.clone())
                                );
                            }
                        }
                        AstAttrTarget::Node | AstAttrTarget::Edge => {
                            // Default attributes for nodes/edges — stored as graph attrs for now
                            // TODO: apply as defaults to subsequently created nodes/edges
                        }
                    }
                }
                AstStmt::Assign { lhs, rhs } => {
                    self.attributes.push(
                        Attribute::new(lhs.clone(), rhs.clone())
                    );
                }
                AstStmt::SubGraph(sg) => {
                    let sub = Graph::from_ast_subgraph(sg);
                    self.sub_graphs.push(sub);
                }
            }
        }
    }

    /// Process edge statement: create edges between consecutive endpoints.
    fn process_edge_stmt(&mut self, endpoints: &[AstEdgeEndpoint], attrs: &[AstAttr]) {
        // Collect node names from endpoints (flatten subgraphs into their nodes)
        let mut node_names: Vec<Vec<String>> = Vec::new();
        for ep in endpoints {
            match ep {
                AstEdgeEndpoint::Node(nid) => {
                    self.add_node(nid.name.clone());
                    node_names.push(vec![nid.name.clone()]);
                }
                AstEdgeEndpoint::SubGraph(sg) => {
                    let sub = Graph::from_ast_subgraph(sg);
                    let names: Vec<String> = sub.nodes.iter().map(|n| n.name.clone()).collect();
                    // Add subgraph nodes to this graph too
                    for name in &names {
                        self.add_node(name.clone());
                    }
                    self.sub_graphs.push(sub);
                    node_names.push(names);
                }
            }
        }

        // Create edges between consecutive endpoint groups
        for i in 0..node_names.len().saturating_sub(1) {
            for from in &node_names[i] {
                for to in &node_names[i + 1] {
                    let edge_name = format!("{}->{}", from, to);
                    let mut edge = Edge::new(edge_name, from.clone(), to.clone());
                    for attr in attrs {
                        edge.attributes.push(
                            Attribute::new(attr.key.clone(), attr.value.clone())
                        );
                    }
                    self.edges.push(edge);
                }
            }
        }
    }

    /// Build a Graph from an AST subgraph.
    fn from_ast_subgraph(sg: &AstSubGraph) -> Graph {
        let name = sg.id.clone().unwrap_or_default();
        let mut graph = Graph::new(name, GraphKind::Directed);
        graph.process_stmts(&sg.stmts);
        graph
    }

    //
    // Graph API for adding nodes, edges, sub-graphs, and attributes
    //

    /// Add a node to the graph. Returns the index of the node.
    /// If a node with the same name already exists, returns its index.
    pub fn add_node(&mut self, name: String) -> usize {
        if let Some(idx) = self.nodes.iter().position(|n| n.name == name) {
            return idx;
        }
        let node = Node::new(name);
        self.nodes.push(node);
        self.nodes.len() - 1
    }

    /// Add an edge between two nodes (by name). Creates nodes if needed.
    pub fn add_edge(&mut self, from: String, to: String) -> usize {
        self.add_node(from.clone());
        self.add_node(to.clone());
        let edge_name = format!("{}->{}", from, to);
        let edge = Edge::new(edge_name, from, to);
        self.edges.push(edge);
        self.edges.len() - 1
    }

    pub fn get_node(&self, name: &str) -> Option<&Node> {
        self.nodes.iter().find(|n| n.name == name)
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    pub fn sub_graph_count(&self) -> usize {
        self.sub_graphs.len()
    }

    pub fn nodes_iter(&self) -> impl Iterator<Item = &Node> {
        self.nodes.iter()
    }

    pub fn edges_iter(&self) -> impl Iterator<Item = &Edge> {
        self.edges.iter()
    }

    pub fn is_directed(&self) -> bool {
        matches!(self.kind, GraphKind::Directed | GraphKind::StrictDirected)
    }

    pub fn is_strict(&self) -> bool {
        matches!(self.kind, GraphKind::StrictDirected | GraphKind::StrictUndirected)
    }

    /// Get a mutable reference to a node by name.
    pub fn get_node_mut(&mut self, name: &str) -> Option<&mut Node> {
        self.nodes.iter_mut().find(|n| n.name == name)
    }

    /// Remove a node by name, along with all its connected edges.
    /// Returns true if the node was found and removed.
    pub fn del_node(&mut self, name: &str) -> bool {
        if let Some(idx) = self.nodes.iter().position(|n| n.name == name) {
            self.nodes.remove(idx);
            self.edges.retain(|e| e.from != name && e.to != name);
            true
        } else {
            false
        }
    }

    /// Remove an edge by index. Returns true if removed.
    pub fn del_edge(&mut self, idx: usize) -> bool {
        if idx < self.edges.len() {
            self.edges.remove(idx);
            true
        } else {
            false
        }
    }

    /// Create a new subgraph within this graph.
    pub fn new_subgraph(&mut self, name: String) -> &mut Graph {
        let kind = self.kind_clone();
        let sub = Graph::new(name, kind);
        self.sub_graphs.push(sub);
        self.sub_graphs.last_mut().unwrap()
    }

    /// Get a subgraph by name.
    pub fn get_subgraph(&self, name: &str) -> Option<&Graph> {
        self.sub_graphs.iter().find(|sg| sg.name == name)
    }

    /// Get a mutable subgraph by name.
    pub fn get_subgraph_mut(&mut self, name: &str) -> Option<&mut Graph> {
        self.sub_graphs.iter_mut().find(|sg| sg.name == name)
    }

    /// Remove a subgraph by name.
    pub fn del_subgraph(&mut self, name: &str) -> bool {
        if let Some(idx) = self.sub_graphs.iter().position(|sg| sg.name == name) {
            self.sub_graphs.remove(idx);
            true
        } else {
            false
        }
    }

    /// Set a graph-level attribute.
    pub fn set_attr(&mut self, key: String, value: String) {
        if let Some(attr) = self.attributes.iter_mut().find(|a| a.key == key) {
            attr.value = value;
        } else {
            self.attributes.push(Attribute::new(key, value));
        }
    }

    /// Get a graph-level attribute value.
    pub fn get_attr(&self, key: &str) -> Option<&str> {
        self.attributes.iter()
            .find(|a| a.key == key)
            .map(|a| a.value.as_str())
    }

    /// Iterator over subgraphs.
    pub fn sub_graphs_iter(&self) -> impl Iterator<Item = &Graph> {
        self.sub_graphs.iter()
    }

    /// Clone the GraphKind (used for subgraph creation).
    fn kind_clone(&self) -> GraphKind {
        match self.kind {
            GraphKind::Directed => GraphKind::Directed,
            GraphKind::Undirected => GraphKind::Undirected,
            GraphKind::StrictDirected => GraphKind::StrictDirected,
            GraphKind::StrictUndirected => GraphKind::StrictUndirected,
        }
    }

    pub fn is_simple(&self) -> bool {
        // Check no self-loops
        for edge in &self.edges {
            if edge.from == edge.to {
                return false;
            }
        }
        // Check no parallel edges
        for i in 0..self.edges.len() {
            for j in (i + 1)..self.edges.len() {
                if self.edges[i].from == self.edges[j].from
                    && self.edges[i].to == self.edges[j].to
                {
                    return false;
                }
            }
        }
        true
    }
}

/// Write the graph in DOT format.
impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let strict = if self.is_strict() { "strict " } else { "" };
        let kind = if self.is_directed() { "digraph" } else { "graph" };
        let edge_op = if self.is_directed() { " -> " } else { " -- " };

        if self.name.is_empty() {
            writeln!(f, "{}{} {{", strict, kind)?;
        } else {
            writeln!(f, "{}{} {} {{", strict, kind, self.name)?;
        }

        // Graph attributes
        for attr in &self.attributes {
            writeln!(f, "    {} = \"{}\";", attr.key, attr.value)?;
        }

        // Nodes with attributes
        for node in &self.nodes {
            if node.attributes.is_empty() {
                writeln!(f, "    {};", node.name)?;
            } else {
                let attrs: Vec<String> = node.attributes.iter()
                    .map(|a| format!("{}=\"{}\"", a.key, a.value))
                    .collect();
                writeln!(f, "    {} [{}];", node.name, attrs.join(", "))?;
            }
        }

        // Edges
        for edge in &self.edges {
            if edge.attributes.is_empty() {
                writeln!(f, "    {}{}{};", edge.from, edge_op, edge.to)?;
            } else {
                let attrs: Vec<String> = edge.attributes.iter()
                    .map(|a| format!("{}=\"{}\"", a.key, a.value))
                    .collect();
                writeln!(f, "    {}{}{} [{}];", edge.from, edge_op, edge.to, attrs.join(", "))?;
            }
        }

        // Subgraphs
        for sg in &self.sub_graphs {
            if sg.name.is_empty() {
                writeln!(f, "    subgraph {{")?;
            } else {
                writeln!(f, "    subgraph {} {{", sg.name)?;
            }
            for node in &sg.nodes {
                writeln!(f, "        {};", node.name)?;
            }
            for edge in &sg.edges {
                writeln!(f, "        {}{}{};", edge.from, edge_op, edge.to)?;
            }
            writeln!(f, "    }}")?;
        }

        write!(f, "}}")
    }
}
