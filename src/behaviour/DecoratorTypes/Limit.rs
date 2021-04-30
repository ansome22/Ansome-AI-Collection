pub struct Limit extends Decorator{
    runLimit: i32,
    runSoFar: i32 = 0,
}


fn run() -> bool{
    if runSoFar >= runLimit{
        return false;
        runSoFar++;
    }
    
    return child.run();
}

