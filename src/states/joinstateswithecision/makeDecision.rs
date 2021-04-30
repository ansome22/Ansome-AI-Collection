 fn makeDecision(node) -> DecisionTreeNode{
    // Check if we need to make a decision.
    if !node or node is_instance_of TargetState{
        // We’ve got the target (or a null target), return it.
        return node
    }
    else{
        // Make the decision and recurse based on the result.
        if node.test(){
            return makeDecision(node.trueNode)
        }

        else{
            return makeDecision(node.falseNode)
        }
    }

}

