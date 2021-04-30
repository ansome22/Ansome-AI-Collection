mod State extends HSMBase{
 fn getStates() -> State{
// If we’re just a state, then the stack is just us.
return [this]
 }

// As before...
 fn getActions() -> Action[]
 fn getEntryActions() -> Action[]
 fn getExitActions() -> Action[]
 fn getTransitions() -> Action[]
}