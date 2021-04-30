pub struct Interrupter extends Decorator{
    // Is our child running?
    isRunning: bool,
    // The final result for our run method.
    result: bool,
}

fn run() -> bool{
    let result = undefined;
    
    // Start our child.
    let thread = new Thread();
    let thread.start(runChild, child);
    
    // Wait until we have a result to return.
    while result == undefined{
        sleep();
        
        return result
    }

}

fn runChild(child){
    let isRunning = true;
    let result = child.run();
    let isRunning = false;
}


26 fn terminate(){
    if isRunning{
        child.terminate();
    }

}

fn setResult(desiredResult: bool){
    result = desiredResult;
}
