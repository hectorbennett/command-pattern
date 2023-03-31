use crate::graph::{Graph, Node};

pub trait Command {
    fn execute(&self);
    fn rollback(&self);
}

pub struct AddNode {
    graph: Graph,
    node: Node,
}

impl AddNode {
    pub fn new(graph: Graph, node: Node) -> AddNode {
        AddNode { graph, node }
    }
}

impl Command for AddNode {
    fn execute(&self) {
        self.graph.add_node(self.node);
    }

    fn rollback(&self) {
        self.graph.remove_node(self.node);
    }
}

pub struct AddEdge {
    graph: Graph,
    node1: Node,
    node2: Node,
}

impl AddEdge {
    pub fn new(graph: Graph, node1: Node, node2: Node) -> AddEdge {
        AddEdge {
            graph,
            node1,
            node2,
        }
    }
}

impl Command for AddEdge {
    fn execute(&self) {
        self.graph.add_edge(self.node1, self.node2);
    }

    fn rollback(&self) {
        self.graph.remove_edge(self.node1, self.node2);
    }
}
