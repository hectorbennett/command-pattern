use std::{cell::RefCell, rc::Rc};

use command_pattern::commands::{AddEdge, AddNode};
use command_pattern::graph::{Edge, Graph, Node};
use command_pattern::history::History;

#[test]
fn test() {
    let graph = Rc::new(RefCell::new(Graph::new()));
    let mut history: History = History::new();

    assert_eq!(history.revision, 0);

    // Add a node to the graph at (0, 0)
    history.append(Box::new(AddNode::new(graph.clone(), [0, 0])));

    // Add a node to the graph at (1, 1)
    history.append(Box::new(AddNode::new(graph.clone(), [1, 1])));

    // Check that the graph is still unchanged
    assert_eq!(history.cursor, 0);
    assert_eq!(history.revision, 2);
    let empty_node_vec: Vec<Node> = vec![];
    assert_eq!(graph.borrow().nodes, empty_node_vec);

    // Execute the commands and check that the changes have now been made
    history.execute();
    assert_eq!(history.cursor, 2);
    assert_eq!(history.revision, 2);
    assert_eq!(graph.borrow().nodes, [[0, 0], [1, 1]]);

    // Connect the two nodes into a vertex
    history.append(Box::new(AddEdge::new(graph.clone(), [0, 0], [1, 1])));
    history.execute();
    assert_eq!(history.revision, 3);
    assert_eq!(graph.borrow().edges, [[[0, 0], [1, 1]]]);

    // Undo the last action
    history.undo();
    assert_eq!(history.revision, 2);
    let empty_edge_vec: Vec<Edge> = vec![];
    assert_eq!(graph.borrow().edges, empty_edge_vec);
    assert_eq!(history.history.len(), 3);

    // Redo the last action
    history.redo();
    assert_eq!(history.revision, 3);
    assert_eq!(graph.borrow().edges, [[[0, 0], [1, 1]]]);

    // Undo the last action and perform a new action, rewriting the history
    history.undo();
    history.append(Box::new(AddNode::new(graph.clone(), [2, 2])));
    history.execute();
    assert_eq!(history.revision, 3);
    assert_eq!(graph.borrow().nodes, [[0, 0], [1, 1], [2, 2]]);
    assert_eq!(graph.borrow().edges, empty_edge_vec);
    assert_eq!(history.history.len(), 3);
}
