## Implementing the Command Pattern and undo/redo functionality in both Python and Rust

I'm currently developing a pixel editor using Rust and WebAssembly. One of the key functionalities that I need to implement is undo/redo. To achieve this, I'll be utilizing the Command Pattern.

The Command Pattern is a design pattern that allows instructions to be encapsulated as objects, each containing all the necessary data to execute a specific command. This approach differs from the traditional method of issuing instructions as simple function calls, as commands can now be queued and executed at a later time.

The Command Pattern is especially useful when developing applications that require undo/redo functionality, as it enables us to store a history of commands along with both execute and rollback methods that allow us to move forwards and backwards through the command history.

Although Python and Rust have different implementations of the Command Pattern, the underlying principles remain consistent. We will demonstrate these principles using a brief Python example before diving into Rust, where we need to take additional steps to handle memory allocation and satisfy the Rust borrow checker and lifetimes.

## Python example

In this tutorial, we will use a simple [Graph](https://en.wikipedia.org/wiki/Graph_(discrete_mathematics)) object containing nodes and edges to illustrate the Command Pattern. Here is an example of the pattern in action:

### Example

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

### History class


The History instance keeps a log of each action and also instructions on how to revert them:


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

### Command class

Commands offer `execute()` and `rollback()` methods that instruct the history instance how to move forwards and backwards through the commands:


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

### Graph class

And finally the graph itself is built very simply, with some methods for adding and removing nodes and edges:

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

Although some parts of the above python example can be translated very straighforwardly, there are some parts that will require some extra thought. 

### Graph struct

Let's start with the Graph struct, which is adapted without any major unexpected changes:

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

    // etc.
}
```

### Command struct

The Command struct is the first place we need to make changes and think about how we manage memory.

For example, if we implement an example Command like this:

```rust
// rust/src/commands.rs

pub struct AddNode {
    graph: &Graph,
    node: Node,
}

// etc.
```

We run into two problems.

The first problem is that the compiler throws a 'missing lifetime specifier' error. The compiler recommends in this scenario that we fix this by making lifetime annotations, however, we have another way to solve this (which we will get into in a bit).

The second issue is that when we come to use these command objects, we will struggle to satisfy the borrow checker. As we continue to append commands to the history, we will be storing new references to the same Graph object, which Rust will not like.

To get around both of these problems, we will store our graph inside a smart pointer and also enable some form of shared mutibility. There are a few ways to do this, but here we will make use of `Rc<T>` and `RefCell<T>`, taking inspiration from the [official Rust documentation](https://doc.rust-lang.org/std/cell/index.html#introducing-mutability-inside-of-something-immutable).

Note, as per above the above documentation, that if we wanted this to work in a multi-threaded situation then we could use an `Arc<T>` and a `Mutex<T>` or an `RwLock<T>` 

Here is an example of what our commands file looks like now:

```rust 
// rust/src/commands.rs

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

// etc.

```

### History struct

Now, to start building the History struct. If we were to translate it directly from the python example, we would start by writing something like this:

```rust
// rust/src/history.rs

pub struct History {
    pub history: Vec<Command>,
    ...
}

impl History {
    ...
    pub fn append(&mut self, command: Command) {
        ...
    }
    ...
}
```

If we attempt to compile something like the above, we will get [an error](https://github.com/rust-lang/rust/blob/master/compiler/rustc_error_codes/src/error_codes/E0782.md) about Trait objects and the dyn keyword, which informs us:

```rust
Trait objects are a way to call methods on types that are not known until runtime but conform to some trait. Trait objects should be formed with `Box<dyn Foo>`.
```

So in light of this helpful advice from the rust compiler, we will rewrite the above as

```rust
// rust/src/history.rs

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

### Rust command pattern in action

Now we can put it all together.

```rust
// rust/tests/test.rs

let graph = Rc::new(RefCell::new(Graph::new()));
let mut history: History = History::new();

// Add a node to the graph at (0, 0)
history.append(Box::new(AddNode::new(graph.clone(), [0, 0])));

// Add a node to the graph at (1, 1)
history.append(Box::new(AddNode::new(graph.clone(), [1, 1])));

// Check that the graph is still unchanged
assert_eq!(graph.borrow().nodes, vec![]);

// Execute the commands and check that the changes have now been made
history.execute();
assert_eq!(graph.borrow().nodes, [[0, 0], [1, 1]]);

// Connect the two nodes into a vertex
history.append(Box::new(AddEdge::new(graph.clone(), [0, 0], [1, 1])));
history.execute();
assert_eq!(graph.borrow().edges, [[[0, 0], [1, 1]]]);

// Undo the last action
history.undo();
assert_eq!(graph.borrow().edges, [[[0, 0], [1, 1]]]);

// Redo the last action
history.redo();
assert_eq!(graph.borrow().edges, [[[0, 0], [1, 1]]]);

```

Notice how we clone `graph` for every command - don't worry, this isn't creating a new graph each time. What it is doing is duplicating the `Rc<T>` to create a new 'owner' for the graph object. An `Rc` (or reference counter) keeps track of the number of owners and will free the memory as soon as the count drops to zero.

Hopefully this has been useful and can be adapted easily for your use case. I hope you've found it useful - Some more detailed source code can be found in this repo: <repo url here>.
