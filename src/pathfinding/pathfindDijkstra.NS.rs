 fn pathfindDijkstra(graph: Graph,start: Node, end: Node) -> Connection[]{
// This structure is used to keep track of the information we need
// for each node.
 pub struct NodeRecord{
    node: Node,
     connection: Connection,
     costSoFar: f64,
}


// Initialize the record for the start node.
 startRecord = new NodeRecord();
 startRecord.node = start;
 startRecord.connection = null;
 startRecord.costSoFar = 0;

// Initialize the open and closed lists.
open = new PathfindingList();
open += startRecord;
closed = new PathfindingList();

// Iterate through processing each node.
while length(open) > 0 {
// Find the smallest element in the open list.
 current: NodeRecord = open.smallestElement();

// If it is the goal node, then terminate.
 if current.node == goal{
 break
}
// Otherwise get its outgoing connections.
 connections = graph.getConnections(current);

// Loop through each connection in turn.
 for connection in connections{
// Get the cost estimate for the end node.
 endNode = connection.getToNode();
 endNodeCost = current.costSoFar + connection.getCost();
}
// Skip if the node is closed.
 if closed.contains(endNode){
continue
}
// .. or if it is open and we’ve found a worse route.
 else if open.contains(endNode){
// Here we find the record in the open list
// corresponding to the endNode.
endNodeRecord = open.find(endNode)
 if endNodeRecord.cost <= endNodeCost{
 continue
}
}
// Otherwise we know we’ve got an unvisited node, so make a
// record for it.
 else{
 endNodeRecord = new NodeRecord()
 endNodeRecord.node = endNode
}
// We’re here if we need to update the node. Update the
// cost and connection.
 endNodeRecord.cost = endNodeCost
 endNodeRecord.connection = connection

// And add it to the open list.
if !open.contains(endNode){
 open += endNodeRecord
}
// We’ve finished looking at the connections for the current
// node, so add it to the closed list and remove it from the
// open list.
open -= current
closed += current
}
// We’re here if we’ve either found the goal, or if we’ve no more
// nodes to search, find which.
if current.node != goal{
    // We’ve run out of nodes without finding the goal, so there’s
    // no solution.
    return null
}
else{
    // Compile the list of connections in the path.
    path = []
}
// Work back along the path, accumulating connections.
while current.node != start{
 path += current.connection
 current = current.connection.getFromNode()
}
// Reverse the path, and return it.
return reverse(path)
}