 mod MarkovStateMachine{

    pub struct Markov {
    // The state vector.
     state: f64[N],
    // The frames to wait before using the default transition.
     resetTime: int,
    // The default transition matrix.
     defaultTransitionMatrix: f64[N,N],
    // The current countdown.
     currentTime: int = resetTime,
    // A list of transitions.
     transitions: MarkovTransition[],
    }
    
    fn update() -> Action[]{
        // Check each transition for a trigger.
        triggeredTransition = null

        for transition in transitions{
            if transition.isTriggered(){
                triggeredTransition = transition;
                break
            }
        }
        // Check if we have a transition to fire.
        if triggeredTransition{
            // Reset the timer.
            currentTime = resetTime;
            
            // Multiply the matrix and the state vector.
            matrix = triggeredTransition.getMatrix();
            state = matrix * state;
            
            // Return the triggered transition’s action list.
            return triggeredTransition.getActions();
        }
        else{
            // Otherwise check the timer.
            currentTime -= 1;
            
            if currentTime <= 0{
                // Do the default transition.
                state = defaultTransitionMatrix * state;
                currentTime = resetTime;
            }
            // Return no actions, since no transition triggered.
            return []
        }
    }
}