
// Holds a state along with its current degree of membership.
pub struct StateAndDOM{
    state: int,
    dom: f64,
}

// The initial states, along with DOM values.
initialStates: StateAndDOM[];

// The current states, with DOM values.
 currentStates = initialStates;

// Checks and applies transitions.
fn update(){
// Sorts the current states into DOM order.
statesInOrder = currentStates.sortByDecreasingDOM()

// Go through each state in turn.
for state in statesInOrder{}

// Go through each transition in the state.
 for transition in currentState.getTransitions():

// Check for triggering.
 if transition.isTriggered():
// Get the transition’s degree of transition.
 dot = transition.getDot()

// Move into all target states.
 for endState in transition.getTargetStates():
// Update the degree of membership.
 end = currentStates.get(endState)
 end.dom = max(end.dom, min(state.dom, dot))

// Check if the state is new.
 if end.dom > 0 && !end in currentStates{
    currentStates.append(end)
 }

// Remove some membership from the start.
 state.dom = min(state.dom, 1 - dot)

// Check if we need to remove the start state.
 if state.dom <= 0.0{
    currentStates.remove(state)
 }
// We don’t look at any more transitions for this
// active state.
break
}