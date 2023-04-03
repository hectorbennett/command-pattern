## Implementing the Command Pattern and undo/redo functionality in both Python and Rust

NOTE: This is a WIP - I'll publish this outside of github once it's finished.

I'm currently developing a pixel editor using Rust and WebAssembly. One of the key functionalities that I need to implement is undo/redo. To achieve this, I'll be utilizing the Command Pattern.

The Command Pattern is a design pattern that allows instructions to be encapsulated as objects, each containing all the necessary data to execute a specific command. This approach differs from the traditional method of issuing instructions as simple function calls, as commands can now be queued and executed at a later time.

The Command Pattern is especially useful when developing applications that require undo/redo functionality, as it enables us to store a history of commands along with both execute and rollback methods that allow us to move forwards and backwards through the command history.

Although Python and Rust have different implementations of the Command Pattern, the underlying principles remain consistent. We will demonstrate these principles using a brief Python example before diving into Rust, where we need to take additional steps to handle memory allocation and satisfy the Rust borrow checker and lifetimes.

## Python example

In this tutorial, we will use a simple Graph object containing nodes and edges to illustrate the Command Pattern. Here is an example of the pattern in action:

```python
# python/src/example.py

from graph import Graph
from history import History
from commands import AddNode, AddEdge

graph = Graph()
history = History()

# Add a node to the graph at (0, 0)
history.append(AddNode(graph, (0, 0)))

# Add a node to the graph at (1, 1)
history.append(AddNode(graph, (1, 1)))

# Check that the graph is still unchanged
assert graph.nodes == set()

# Execute the commands and check that the changes have now been made
history.execute()
assert graph.nodes == {(0, 0), (1, 1)}

# Connect the two nodes into a vertex
history.append(AddEdge(graph, (0, 0), (1, 1)))
history.execute()
assert graph.edges == {((0, 0), (1, 1))}

# undo the last action
history.undo()
assert graph.edges == set()

# redo the last action
history.redo()
assert graph.edges == {((0, 0), (1, 1))}

# undo the last action and perform a new action, rewriting the history
history.undo()
history.append(AddNode(graph, (2, 2)))
history.execute()
assert graph.nodes == {(0, 0), (1, 1), (2, 2)}

```

The History instance keeps a log of each action and also instructions on how to revert them.

```python
# python/src/history.py

class History:
    def __init__(self):
        # A log of all the commands in their execution order
        self.history = []

        # Where we have executed up to so far
        self.cursor = 0

        # The position in the history we want to execute to
        self.revision = 0

    def append(self, command):
        # Destroy anything ahead of the current revision
        self.history = self.history[0 : self.revision]

        # Add a command to the history
        self.history.append(command)

        # move forward one step in the history
        self.revision += 1

    def execute(self):
        # execute all the methods that have not yet been executed
        for i in range(self.cursor, self.revision):
            self.history[i].execute()
        self.cursor = self.revision

    def undo(self):
        if not self.history:
            return

        # Move the cursor back 1
        self.revision = max(0, self.revision - 1)

        # undo the current command
        self.history[self.revision].rollback()

        self.cursor = self.revision

    def redo(self):
        if self.revision == len(self.history):
            return

        # redo the current command
        self.history[self.revision].execute()

        # Move forwards (again) to where we were in history
        self.revision += 1

        self.cursor = self.revision

```

Commands offer `execute()` and `rollback()` methods that instruct the history instance how to move forwards and backwards through the commands.

```python
# python/src/commands.py

class AddNode:
    def __init__(self, graph, node):
        self.graph = graph
        self.node = node

    def execute(self):
        self.graph.add_node(self.node)

    def rollback(self):
        self.graph.remove_node(self.node)


class AddEdge:
    def __init__(self, graph, node1, node2):
        self.graph = graph
        self.node1 = node1
        self.node2 = node2

    def execute(self):
        self.graph.add_edge(self.node1, self.node2)

    def rollback(self):
        self.graph.remove_edge(self.node1, self.node2)

```

And finally the graph itself is built very simply

```python
# python/src/graph.py

class Graph:
    def __init__(self):
        self.nodes = set()
        self.edges = set()

    def add_node(self, node):
        self.nodes.add(node)

    def remove_node(self, node):
        self.nodes.remove(node)

    def add_edge(self, node1, node2):
        self.edges.add((node1, node2))

    def remove_edge(self, node1, node2):
        self.edges.remove((node1, node2))

```

## Implementing in Rust

Some parts of the above code can be translated from python very straightforwardly, for example the Graph class is adapted without any unexpected changes.

```rust
// rust/src/graph.rs

pub type Node = [u8; 2];
pub type Edge = [Node; 2];

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
```

For command objects, we create a trait 'Command' with execute and rollback
methods and we implement this trait on each of the commands we wish to write.

```rust
// rust/src/commands.rs

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

```

Once we start writing the history class, we need to start making some changes
to accomodate for the way that Rust handles traits. Below is a (faulty) version of the history class translated from python.

```rust
// rust/src/history.rs

use std::cmp;

use crate::commands::Command;

pub struct History {
    // A log of all the commands in their execution order
    pub history: Vec<Command>,

    // Where we have executed up to so far
    pub cursor: usize,

    // The position in the history we want to execute to
    pub revision: usize,
}

impl History {
    pub fn new() -> History {
        History {
            history: vec![],
            cursor: 0,
            revision: 0,
        }
    }

    pub fn append(&mut self, command: Command) {
        // Destroy anything ahead of the current revision
        self.history.truncate(self.revision);

        // Add a command to the history
        self.history.push(command);

        // Move forward one step in the history
        self.revision += 1;
    }
}
```

If we attempt to compile the above, we will get the following error code:

https://github.com/rust-lang/rust/blob/master/compiler/rustc_error_codes/src/error_codes/E0782.md

which tells us

'Trait objects are a way to call methods on types that are not known until runtime but conform to some trait. Trait objects should be formed with `Box<dyn Foo>`.'

So we rewrite the above as

```rust
pub struct History {
    pub history: Vec<Box<dyn Command>>,
    ...
}

impl History {

    ...

    pub fn append(&mut self, command: Box<dyn Command>) {
        ...
    }

    ...
}

```

Now if we compile our code, we get a 'missing lifetime specififier' error. Our Command class is currently pointing to a reference of a graph object.

```
9 |     graph: &Graph,
  |            ^ expected named lifetime parameter
```

To avoid using lifetime annotations, we are going to transform our commands to point to a graph object within a RefCell within a Rc.

https://doc.rust-lang.org/std/cell/index.html#introducing-mutability-inside-of-something-immutable

So we rewrite our commands to point to a 

