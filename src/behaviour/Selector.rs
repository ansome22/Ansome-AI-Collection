 pub struct Selector extends Task{
    children: Task[]
 }

fn run() -> bool{
    for c in children{
        if c.run(){
            return true
        }else{
            return false
        }

    }
}
