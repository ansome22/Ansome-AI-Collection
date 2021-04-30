 pub struct Heuristic{
     // Stores the goal node that this heuristic is estimating for.
    goalNode: Node
 }

// Estimated cost to reach the stored goal from the given node.
fn estimate(fromNode: Node) -> f64{
    return estimate(fromNode, goalNode)
}
// Estimated cost to move between any two nodes.
fn estimate(fromNode: Node, toNode: Node) -> f64{}