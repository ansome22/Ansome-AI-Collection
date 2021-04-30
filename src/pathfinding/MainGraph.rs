 pub struct MainGraph{
// The graph for the rest of the world.
 worldGraph: Graph
}
// Holds data for a building instance.
pub struct EntranceNodeAssignment{
    fromNode: Node
    toInstanceNode: Node
    buildingGraph: Graph
 }


 class Buildings extends HashMap[Node -> EntranceNodeAssignment[]]:
// Return the graph containing the given node, or null.
 fn getBuilding(node) -> Graph{
    // Entrance node assignments.
    buildingInstances: Buildings;
 }



 fn getConnections(fromNode: Node) -> Connection[]{
    // Check if fromNode is in the range of any building.
    building = buildingInstances.getBuilding(fromNode)

    // If we have a building, then delegate to it.
    if building{
        return building.getConnections(fromNode);
    }
    
    // Otherwise, delegate to the world graph.
    connections = worldGraph.getConnections(fromNode);

    // Add connections for each entrance from this node.272 Chapter 4 Pathfinding
    for building in buildingInstances[fromNode]{
    connection = new Connection;
    connection.fromNode = fromNode;
    connection.toNode = building.toInstanceNode;
    connection.cost = 0;
    connections += connection;
    }
    return connections
 }
