use crate::ast::{super_visit_function, ASTVisitor, Expression, Function, Program, Statement};
use petgraph::graph::NodeIndex;

pub type Cfg = petgraph::graph::DiGraph<CFGNode, EdgeCondition>;

#[derive(Debug)]
// FIXME: This is pretty clumsy. It means that any number of edges can be added to any kind of cfg node.
// Ideally, we'd want to be able to let the Cfg nodes be responsible for their own edges -- that way, for
// example, we can assert that there's at most a single true and false edge coming out from an if statement
// CFG node.
pub enum EdgeCondition {
    Unconditional,
    IfTrue,
    IfFalse,
}

#[derive(Debug)]
pub enum CFGNode {
    Entry,
    Statement(Statement),
    CondBr(Expression),
    Exit,
}

/// Builds separate CFGs for each function.
pub struct IntraprocCFGBuilder {
    cfg: Vec<Cfg>,
    current_function_idx: usize,
    last_node_idx: Option<NodeIndex>,
}

impl IntraprocCFGBuilder {
    pub fn to_owned_cfg_vec(self) -> Vec<Cfg> {
        self.cfg
    }
    pub fn from_program(p: Program) -> IntraprocCFGBuilder {
        let mut builder = Self {
            cfg: Vec::with_capacity(p.functions.len()),
            current_function_idx: 0,
            last_node_idx: None,
        };
        builder.visit_program(p);
        builder
    }
    fn current_cfg_mut(&mut self) -> &mut Cfg {
        &mut self.cfg[self.current_function_idx]
    }

    /// Adds a node connected to the last added node.
    fn append_node(&mut self, n: CFGNode, tag: EdgeCondition) {
        let last_node = self.last_node_idx;
        debug_assert!(
            self.last_node_idx.is_some() || matches!(n, CFGNode::Entry),
            "last node should only ever be None if the Entry node hasn't been added yet"
        );
        let this_node = self.current_cfg_mut().add_node(n);
        if last_node.is_some() {
            self.current_cfg_mut().add_edge(
                last_node.expect("Missing last node -- should at least be the entry node"),
                this_node,
                tag,
            );
        }
        self.last_node_idx = Some(this_node);
    }
}

impl ASTVisitor for IntraprocCFGBuilder {
    fn visit_function(&mut self, f: Function) {
        // Insert a new CFG with the entry node in it.
        self.cfg.push({
            let mut new_cfg = Cfg::new();
            self.last_node_idx = Some(new_cfg.add_node(CFGNode::Entry));
            new_cfg
        });
        super_visit_function(self, f);
        // Cap it off with the exit node.
        self.append_node(CFGNode::Exit, EdgeCondition::Unconditional);
        // Then, update the current function index.
        self.current_function_idx += 1;
    }

    fn visit_statement(&mut self, s: Statement) {
        self.append_node(CFGNode::Statement(s), EdgeCondition::Unconditional)
    }
}
