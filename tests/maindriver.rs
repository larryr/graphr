use graphr::cgraph;

#[test]
fn test_structs() {
    let mut g = cgraph::Graph::new("test".to_string(), cgraph::GraphKind::Directed);
    g.add_node("node1".to_string());
    g.add_edge("node1".to_string(), "node2".to_string());
    let a = cgraph::Attribute::new("color".to_string(), "red".to_string());

    println!("g={}", g);
    println!("a={}", a);
    assert_eq!(g.node_count(), 2);
    assert_eq!(g.edge_count(), 1);
}

#[test]
fn test_parse_str() {
    let dot = r#"digraph G { a -> b; b -> c [label="test"] }"#;
    let g = cgraph::Graph::parse_str(dot).unwrap();
    assert_eq!(g.name, "G");
    assert!(g.is_directed());
    assert_eq!(g.node_count(), 3);
    assert_eq!(g.edge_count(), 2);
}

#[test]
fn test_parse_undirected() {
    let dot = r#"graph G { a -- b }"#;
    let g = cgraph::Graph::parse_str(dot).unwrap();
    assert!(!g.is_directed());
    assert_eq!(g.node_count(), 2);
    assert_eq!(g.edge_count(), 1);
}

#[test]
fn test_parse_strict() {
    let dot = r#"strict digraph G { a -> b }"#;
    let g = cgraph::Graph::parse_str(dot).unwrap();
    assert!(g.is_strict());
    assert!(g.is_directed());
}

#[test]
fn test_parse_with_attributes() {
    let dot = r#"digraph G { a [label="Node A"]; a -> b }"#;
    let g = cgraph::Graph::parse_str(dot).unwrap();
    let node_a = g.get_node("a").unwrap();
    assert_eq!(node_a.attributes.len(), 1);
    assert_eq!(node_a.attributes[0].key, "label");
    assert_eq!(node_a.attributes[0].value, "Node A");
}

#[test]
fn test_parse_subgraph() {
    let dot = r#"digraph G { a -> {b c} }"#;
    let g = cgraph::Graph::parse_str(dot).unwrap();
    assert_eq!(g.node_count(), 3);  // a, b, c
    assert_eq!(g.edge_count(), 2);  // a->b, a->c
}

#[test]
fn test_display_roundtrip() {
    let dot = r#"digraph G { a -> b; b -> c }"#;
    let g = cgraph::Graph::parse_str(dot).unwrap();
    let output = format!("{}", g);
    // Parse the output again
    let g2 = cgraph::Graph::parse_str(&output).unwrap();
    assert_eq!(g2.node_count(), g.node_count());
    assert_eq!(g2.edge_count(), g.edge_count());
}

#[test]
fn test_del_node() {
    let mut g = cgraph::Graph::new("test".to_string(), cgraph::GraphKind::Directed);
    g.add_edge("a".to_string(), "b".to_string());
    g.add_edge("b".to_string(), "c".to_string());
    assert_eq!(g.node_count(), 3);
    assert_eq!(g.edge_count(), 2);

    g.del_node("b");
    assert_eq!(g.node_count(), 2);
    assert_eq!(g.edge_count(), 0); // both edges touched b
}

#[test]
fn test_subgraph_api() {
    let mut g = cgraph::Graph::new("main".to_string(), cgraph::GraphKind::Directed);
    {
        let sg = g.new_subgraph("cluster_0".to_string());
        sg.add_node("x".to_string());
        sg.add_node("y".to_string());
        sg.add_edge("x".to_string(), "y".to_string());
    }
    assert_eq!(g.sub_graph_count(), 1);
    let sg = g.get_subgraph("cluster_0").unwrap();
    assert_eq!(sg.node_count(), 2);
    assert_eq!(sg.edge_count(), 1);

    g.del_subgraph("cluster_0");
    assert_eq!(g.sub_graph_count(), 0);
}

#[test]
fn test_graph_attrs() {
    let mut g = cgraph::Graph::new("test".to_string(), cgraph::GraphKind::Directed);
    g.set_attr("rankdir".to_string(), "LR".to_string());
    assert_eq!(g.get_attr("rankdir"), Some("LR"));

    // Update existing
    g.set_attr("rankdir".to_string(), "TB".to_string());
    assert_eq!(g.get_attr("rankdir"), Some("TB"));
    assert_eq!(g.attributes.len(), 1); // no duplicate
}

#[test]
fn test_node_attrs() {
    let mut g = cgraph::Graph::new("test".to_string(), cgraph::GraphKind::Directed);
    g.add_node("a".to_string());
    let node = g.get_node_mut("a").unwrap();
    node.set_attr("color".to_string(), "red".to_string());
    assert_eq!(node.get_attr("color"), Some("red"));
}

#[test]
fn test_is_simple() {
    let mut g = cgraph::Graph::new("test".to_string(), cgraph::GraphKind::Directed);
    g.add_edge("a".to_string(), "b".to_string());
    g.add_edge("b".to_string(), "c".to_string());
    assert!(g.is_simple());

    // Add parallel edge
    g.add_edge("a".to_string(), "b".to_string());
    assert!(!g.is_simple());
}
