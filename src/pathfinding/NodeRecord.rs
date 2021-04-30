// The structure used to track the information we need for each node.
pub struct NodeRecord{
    node: Node,
    connection: Connection,
    costSoFar: f64,
    estimatedTotalCost: f64,
    category: Category,
    nextRecordInList: NodeRecord,
}


enum Category {CLOSED, OPEN, UNVISITED}