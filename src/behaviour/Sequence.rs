pub struct Sequence extends Task{
    children: Task[]
}

fn run() -> bool{
    for c in children{
        if !c.run(){
            return false
        }else{
            return true
        }
    }
}

