pub struct PerformInterruption extends Task{
//The interrupter we’ll be interrupting.
    interrupter: Interrupter,
    
// The result we want to insert.
    desiredResult: bool,
}

fn run() -> bool{
    interrupter.setResult(desiredResult)
    return true
}
