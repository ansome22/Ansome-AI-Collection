pub struct Transition{
actions: Action[]
}

fn getActions() -> Action[]{
return actions
}

 fn getTargetState() -> State{
return targetState
 }
 

condition: Condition
fn isTriggered() -> bool {

return condition.test()
}

