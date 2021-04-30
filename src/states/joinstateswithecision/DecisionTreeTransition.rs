 mod DecisionTreeTransition extends Transition{
    // The target state at the end of the decision tree, when a
    // decision has been made.
    targetState: State = null

    // The root decision in the tree.
    decisionTreeRoot: DecisionTreeNode

    fn getActions() -> Action[]{
        if targetState:
        return targetState.getActions()
        else:
        return []
    }

    fn getTargetState() -> State{
        if targetState:
        return targetState.getTargetState()
        else:
        return null
    }


    fn isTriggered() -> bool{
    // Get the result of the decision tree and store it.
    targetState = makeDecision(decisionTreeRoot)

    // Return true if the target state points to a destination,
    // otherwise assume that we don’t trigger.
    return targetState != null
    }

}
