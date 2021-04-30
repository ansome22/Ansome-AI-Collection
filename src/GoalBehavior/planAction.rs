fn planAction(worldModel: WorldModel,
2 goal: Goal,
3 heuristic: Heuristic,
4 maxDepth: int) -> Action{
    //Initial cutoff is the heuristic from the start model.
     cutoff: f64 = heuristic.estimate(worldModel);
    
    // Create a transposition table.
     transpositionTable = new TranspositionTable();
    
    // Iterate the depth first search until we have a valid plan, or
    // until we know there is none possible.
     while 0 < cutoff < infinity{
    // Get the new cutoff, or best action from the search.
     cutoff, action = doDepthFirst(
     worldModel, goal,
     transpositionTable, heuristic,
     maxDepth, cutoff)
    }
    // If we have an action, return it.
     return action
}

