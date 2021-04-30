 mod BehavioralLOD extends Behavior {

    pub struct LOD {
    // The list of behavior records.
    records: BehaviorRecord[],
    // The current behavior.
    current: Behavior = null,
    // The current importance.
    importance: f64,
}
    // Finds the right record to run, and runs it.
    fn run(time: int){
        // Check if we need to find a new behavior.
        if !(current and current.isValid(importance)){

        // Find a new behavior, by checking each in turn.
        next: BehaviorRecord = null;
        for record in records{
            // Check if the record is valid.
            if record.isValid(importance){
                // If so, use it.
                next = record
                break
            }
        }
        // We’re leaving the current behavior, so notify
        // it where we’re going.
         if current and current.exit{
         current.exit(next.behavior);
        // Likewise, notify our new behavior where we’re
        // coming from.
            if next and next.enter{
                next.enter(current.behavior);
                
                // Set our current behavior to be that found.
                current = next;
            }
        }
        
        // We should have either decided to use the previous
        // behavior, or else we have found a new one, either
        // way it is stored in the current variable, so run it.
         current.behavior.run(time)
    }
}