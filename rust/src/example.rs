use crate::{commands::AddNode, graph::Graph, history::History};

pub fn example() {
    let mut graph: Graph = Graph::new();
    let mut history: History = History::new();

    history.append(AddNode::new(&graph, [0, 0]));
}
