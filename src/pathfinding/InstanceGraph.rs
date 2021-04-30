class InstanceGraph extends Graph{
// The building graph to delegate to.
 building: Graph

// Holds data for exit nodes.
 class ExitNodeAssignment:
 fromNode: Node
 toWorldNode: Node

// A hash of assignments for connections to the outside world.
 exitNodes: HashMap[Node -> ExitNodeAssignment[]]

// The offset for the nodes values used in this instance.
 nodeOffset: Node

 fn getConnections(fromNode: Node) -> Connection[]{
// Translate the node into building graph values.
buildingFromNode: Node = fromNode - nodeOffset


// Delegate to the building graph.
connections = building.getConnections(buildingFromNode)

// Translate the returned connections into our node values.
 for connection in connections{
 connection.toNode += nodeOffset
 }
// Add connections for each exit from this node.
 for exitAssignment in exitNodes[fromNode]{
 connection = new Connection()
 connection.fromNode = fromNode
 connection.toNode = exitAssignment.toWorldNode
 connection.cost = 0
 connections += connection
}

 return connections
}

}
