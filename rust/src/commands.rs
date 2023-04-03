use crate::graph::{Graph, Node};
use std::cell::RefCell;
use std::rc::Rc;

pub trait Command {
    fn execute(&self);
    fn rollback(&self);
}

pub struct AddNode {
    graph: Rc<RefCell<Graph>>,
    node: Node,
}

impl AddNode {
    pub fn new(graph: Rc<RefCell<Graph>>, node: Node) -> AddNode {
        AddNode { graph, node }
    }
}

impl Command for AddNode {
    fn execute(&self) {
        self.graph.borrow_mut().add_node(self.node);
    }

    fn rollback(&self) {
        self.graph.borrow_mut().remove_node(self.node);
    }
}

pub struct AddEdge {
    graph: Rc<RefCell<Graph>>,
    node1: Node,
    node2: Node,
}

impl AddEdge {
    pub fn new(graph: Rc<RefCell<Graph>>, node1: Node, node2: Node) -> AddEdge {
        AddEdge {
            graph,
            node1,
            node2,
        }
    }
}

impl Command for AddEdge {
    fn execute(&self) {
        self.graph.borrow_mut().add_edge(self.node1, self.node2);
    }

    fn rollback(&self) {
        self.graph.borrow_mut().remove_edge(self.node1, self.node2);
    }
}
