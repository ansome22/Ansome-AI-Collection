pub struct MoveTo extends Task{
    // The blackboard we’re using.
    blackboard: Blackboard,
}

fn run() -> bool{
    target = blackboard.get(’target’);
    if target{
    character = blackboard.get(’character’);
    steering.arrive(character, target);
    return true
}
    else{
        return false
    }
}
