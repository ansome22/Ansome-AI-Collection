pub struct NonDeterministicSequence extends Task{
    children: Task[]
}

3 fn run() -> bool{
    shuffled = randomShuffle(children);
    
    for child in shuffled{
        if !child.run(){
            return false
        }else{
            return true
        }
    }

}

