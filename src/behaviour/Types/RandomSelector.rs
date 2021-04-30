//The simplest would be a Selector that repeatedly tries a single child at random:
pub struct RandomSelector extends Task{
    children: Task[]
}

fn run() -> bool{
    while true{
        child = randomChoice(children)
        if child.run(){
            return true
        }
    }
}


