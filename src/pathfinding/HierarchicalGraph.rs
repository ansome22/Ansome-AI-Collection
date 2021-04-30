mod HierarchicalGraph extends Graph{
// ... Inherits getConnections from graph ...
// Return the number of levels in the graph.
fn getLevels() -> int{}

// Set the graph so all future calls to getConnections are treated
// as requests at the given level.
fn setLevel(level: int){}

// Convert a node at the input level to a node at the output level.
13 fn getNodeAtLevel(inputLevel: int, node: Node,outputLevel: int) -> Node{}
}
