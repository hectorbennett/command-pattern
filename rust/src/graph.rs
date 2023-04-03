pub type Node = [u8; 2];
pub type Edge = [Node; 2];

#[derive(Debug)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            nodes: vec![],
            edges: vec![],
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }

    pub fn remove_node(&mut self, node: Node) {
        self.nodes.retain(|&n| n != node);
    }

    pub fn add_edge(&mut self, node1: Node, node2: Node) {
        self.edges.push([node1, node2]);
    }

    pub fn remove_edge(&mut self, node1: Node, node2: Node) {
        self.edges.retain(|&n| n != [node1, node2]);
    }
}

#[test]
fn test_graph() {
    let mut graph = Graph::new();
    graph.add_node([0, 0]);
    graph.add_node([1, 1]);
    assert_eq!(graph.nodes, vec![[0, 0], [1, 1]]);

    graph.remove_node([1, 1]);
    assert_eq!(graph.nodes, vec![[0, 0]]);

    graph.add_edge([0, 0], [1, 1]);
    assert_eq!(graph.edges, vec![[[0, 0], [1, 1]]])
}
