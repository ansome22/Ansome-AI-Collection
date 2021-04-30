pub struct Signal{
    // The unique identifier for this signal.
    identifier: string,

    // Checks if the signal has been delivered.
    delivered: bool = false,
}

fn canInterrupt() -> bool{
    // We can interrupt this action at any time.6.4 Coordinated Action 567
    return true
}


 fn canDoBoth(otherAction: Action) -> bool{
    // We can do any other action at the same time as this one. We
    // won’t be waiting on this action at all, and we shouldn’t
    // wait another frame to carry on with our actions.
    return true
}

 fn isComplete() -> bool{
    // This event is complete only after it has delivered its
    // signal.
    return delivered
}


 fn execute(){
    // Deliver the signal.
    globalIdStore.setIdentifier(identifier)

    // Record that we’ve delivered.
    delivered = true
}
