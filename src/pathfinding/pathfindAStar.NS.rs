// Also possible 
// SMA*—Simplified Memory-Bounded A*
// IDA*—Iterative Deepening A*
 
fn pathfindAStar(graph: Graph, start: Node, end: Node, heuristic: Heuristic) -> Connection[]{
// This structure is used to keep track of the
// information we need for each node.
pub struct NodeRecord{
 node: Node
 connection: Connection
 costSoFar: f64
 estimatedTotalCost: f64
}

// Initialize the record for the start node.
 startRecord = new NodeRecord();
 startRecord.node = start;
 startRecord.connection = null;
 startRecord.costSoFar = 0;
 startRecord.estimatedTotalCost = heuristic.estimate(start);

// Initialize the open and closed lists.
 open = new PathfindingList();
 open += startRecord;
 closed = new PathfindingList();

// Iterate through processing each node.
 while length(open) > 0{
// Find the smallest element in the open list (using the
// estimatedTotalCost).
 current = open.smallestElement();

// If it is the goal node, then terminate.
 if current.node == goal{
 break
}
// Otherwise get its outgoing connections.
 connections = graph.getConnections(current);

// Loop through each connection in turn.
 for connection in connections:
// Get the cost estimate for the end node.
 endNode = connection.getToNode()
 endNodeCost = current.costSoFar + connection.getCost()

// If the node is closed we may have to skip, or remove it
// from the closed list.
 if closed.contains(endNode):
// Here we find the record in the closed list
// corresponding to the endNode.
 endNodeRecord = closed.find(endNode)

// If we didn’t find a shorter route, skip.
 if endNodeRecord.costSoFar <= endNodeCost:
 continue

// Otherwise remove it from the closed list.
 closed -= endNodeRecord

// We can use the node’s old cost values to calculate
// its heuristic without calling the possibly expensive
// heuristic fn.
 endNodeHeuristic = endNodeRecord.estimatedTotalCost -
 endNodeRecord.costSoFar

// Skip if the node is open and we’ve not found a better
// route.
 else if open.contains(endNode):
// Here we find the record in the open list
// corresponding to the endNode.4.3 A* 221
 endNodeRecord = open.find(endNode)

// If our route is no better, then skip.
 if endNodeRecord.costSoFar <= endNodeCost:
 continue

// Again, we can calculate its heuristic.
 endNodeHeuristic = endNodeRecord.cost -
 endNodeRecord.costSoFar

// Otherwise we know we’ve got an unvisited node, so make a
// record for it.
 else:
 endNodeRecord = new NodeRecord()
 endNodeRecord.node = endNode

// We’ll need to calculate the heuristic value using
// the fn, since we don’t have an existing record
// to use.
 endNodeHeuristic = heuristic.estimate(endNode)

// We’re here if we need to update the node. Update the
// cost, estimate and connection.
 endNodeRecord.cost = endNodeCost
 endNodeRecord.connection = connection
 endNodeRecord.estimatedTotalCost = endNodeCost +
endNodeHeuristic

// And add it to the open list.
 if !open.contains(endNode):
 open += endNodeRecord

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

}else{
// Compile the list of connections in the path.222 Chapter 4 Pathfinding
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

