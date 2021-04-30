 mod StateMachine{
    // We’re in one state at a time.
     initialState: State;
     currentState: State = initialState;
    
    // Checks and applies transitions, returning a list of actions.
     fn update() -> Action[]{
    // Assume no transition is triggered.
    triggered: Transition = null;
    
    // Check through each transition and store the first
    // one that triggers.
     for transition in currentState.getTransitions(){
        if transition.isTriggered(){
            triggered = transition;
            break
        }
        
        // Check if we have a transition to fire.
        if triggered{
            // Find the target state.
            targetState = triggered.getTargetState()
            
            // Add the exit action of the old state, the
            // transition action and the entry for the new state.
            actions = currentState.getExitActions()
            actions += triggered.getActions()
            actions += targetState.getEntryActions()
            
            // Complete the transition and return the action list.
            currentState = targetState
            return actions
            
            // Otherwise just return the current state’s actions.
        }
        else{
            return currentState.getActions();
        }
        
     }
    }
 }
