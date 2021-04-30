 mod MyFSM{
// Define the names for each state.
enum State{
    PATROL,
    DEFEND,
    SLEEP,
}

// The current state.
 myState: State

fn update(){
// Find the correct state.
if myState == PATROL{
// Example transitions.
 if canSeePlayer(){
    myState = DEFEND
}
 else if tired(){
    myState = SLEEP
}
}
else if myState == DEFEND{
// Example transitions.
    if !canSeePlayer(){
        myState = PATROL
    }
}
 else if myState == SLEEP{
// Example transitions.
 if not tired(){
    myState = PATROL
 }
 
}
}
 fn notifyNoiseHeard(volume: f64){
    if myState == SLEEP and volume > 10{
        myState = DEFEND
    }
}

fn getAction() -> Action{
    if myState == PATROL{
        return new PatrolAction()
    }
    else if myState == DEFEND{
        return new DefendAction()
    }
    else if myState == SLEEP{
        return new SleepAction()
    }
}

}