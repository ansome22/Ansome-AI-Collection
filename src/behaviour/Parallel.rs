pub struct Parallel extends Task{
children: Task[],
}

// Holds all the children currently running.
runningChildren: Task[] = [];
// The final result for our run method.
result: bool;

10 fn run() -> bool{
    result = null
}

// Internal fns, defined locally to this run method.
14
fn runChild(child){
     runningChildren += child;
     returned = child.run();
     runningChildren -= child;
    
     if returned == false{
    // Write to the outer result variable.
    result = false;

        // Stop all the running children.
        terminate()
     }else if runningChildren.length == 0{
     result = true;
     }

}

fn terminate(){
    for child in runningChildren{
        child.terminate();
    
        // Start all our children running.
        for child in children{
            let thread = new Thread();
            thread.start(runChild, child);
    
            // Wait until we have a result to return.
            while result == null{
                sleep() 
            } 
        }
    }

    return result
}
