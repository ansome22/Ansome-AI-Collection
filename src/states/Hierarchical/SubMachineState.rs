 mod SubMachineState extends State, HierarchicalStateMachine{
    // Route to the state.
    fn getActions() -> Action[]{
        return State.getActions()
    }
    // Route update to the state machine.
    fn update() -> Action[]{
        return HierarchicalStateMachine.update()
    }

    // We get states by adding ourself to our active children.
    fn getStates() -> State[]{
        if currentState{
            return [this] + currentState.getStates();
        }
        else{
            return [this];
        }
    }
 }
