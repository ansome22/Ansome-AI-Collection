pub struct UntilFail extends Decorator{}

fn run() -> bool{
    while true {
        result: bool = child.run();
        if !result{
            break
        }
    }
    return true
}

