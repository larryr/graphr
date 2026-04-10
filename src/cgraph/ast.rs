// AST types produced by the LALRPOP parser.
// These are intermediate representations that get converted
// into the runtime Graph/Node/Edge types.

#[derive(Debug, Clone)]
pub struct AstGraph {
    pub strict: bool,
    pub directed: bool,
    pub id: Option<String>,
    pub stmts: Vec<AstStmt>,
}

#[derive(Debug, Clone)]
pub enum AstStmt {
    Node {
        id: AstNodeId,
        attrs: Vec<AstAttr>,
    },
    Edge {
        endpoints: Vec<AstEdgeEndpoint>,
        attrs: Vec<AstAttr>,
    },
    Attr {
        target: AstAttrTarget,
        attrs: Vec<AstAttr>,
    },
    Assign {
        lhs: String,
        rhs: String,
    },
    SubGraph(AstSubGraph),
}

#[derive(Debug, Clone)]
pub enum AstEdgeEndpoint {
    Node(AstNodeId),
    SubGraph(AstSubGraph),
}

#[derive(Debug, Clone)]
pub struct AstNodeId {
    pub name: String,
    pub port: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AstAttr {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub enum AstAttrTarget {
    Graph,
    Node,
    Edge,
}

#[derive(Debug, Clone)]
pub struct AstSubGraph {
    pub id: Option<String>,
    pub stmts: Vec<AstStmt>,
}
