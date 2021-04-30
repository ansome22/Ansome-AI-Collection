fn hierarchicalPathfind(graph: Graph, start: Node,end: Node, heuristic: Heuristic) -> Connection[]{
    // Check if we have no path to find.
    if (start == end){
        return null
    }

    // Set up our initial pair of nodes.
    startNode: Node = start;
    endNode: Node = end;
    levelOfNodes: int = 0;

    // Descend through levels of the graph.
    currentLevel: int = graph.getLevels() - 1;
    
    while currentLevel >= 0{
        // Find the start and end nodes at this level.
        startNode = graph.getNodeAtLevel(0, start, currentLevel);
        endNode = graph.getNodeAtLevel(levelOfNodes,endNode, currentLevel);
        levelOfNodes = currentLevel;
        
        // Are the start and end node the same?
        if startNode == endNode{
            // Skip this level.
            continue
        }

        // Otherwise we can perform the plan.
        graph.setLevel(currentLevel)
        path = pathfindAStar(graph, startNode, endNode, heuristic)

        // Take the first move of this plan and use it for the next.
        endNode = path[0].getToNode()
    }
    // The last path we considered would have been the one at level
    // zero: we return it.
    return path
}
