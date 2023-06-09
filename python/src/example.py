from graph import Graph
from history import History
from commands import AddNode, AddEdge

graph = Graph()

history = History()

assert history.revision == 0

# Add a node to the graph at (0, 0)
history.append(AddNode(graph, (0, 0)))

# Add a node to the graph at (1, 1)
history.append(AddNode(graph, (1, 1)))

# Check that the graph is still unchanged
assert history.cursor == 0
assert history.revision == 2
assert graph.nodes == set()

# Execute the commands and check that the changes have now been made
history.execute()
assert history.cursor == 2
assert history.revision == 2
assert graph.nodes == {(0, 0), (1, 1)}

# Connect the two nodes into a vertex
history.append(AddEdge(graph, (0, 0), (1, 1)))
history.execute()
assert history.revision == 3
assert graph.edges == {((0, 0), (1, 1))}

# Undo the last action
history.undo()
assert history.revision == 2
assert graph.edges == set()
assert len(history.history) == 3

# Redo the last action
history.redo()
assert history.revision == 3
assert graph.edges == {((0, 0), (1, 1))}
assert len(history.history) == 3

# Undo the last action and perform a new action, rewriting the history
history.undo()
history.append(AddNode(graph, (2, 2)))
history.execute()
assert history.revision == 3
assert graph.nodes == {(0, 0), (1, 1), (2, 2)}
assert graph.edges == set()
assert len(history.history) == 3
