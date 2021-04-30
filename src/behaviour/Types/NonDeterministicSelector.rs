/*  This gives us randomness but has two problems: it may try the same child more than
# once, even several times in a row, and it will never give up, even if all its children repeatedly
# fail. For these reasons, this simple implementation isn’t widely useful, but it can still be used,
# especially in combination with the parallel task we’ll meet later in this section.
# A better approach would be to walk through all the children in some random order. We
# can do this for either Selectors or Sequences. Using a suitable random shuffling procedure,
# we can implement this as:
 */

pub struct NonDeterministicSelector extends Task{
    children: Task[]
}

fn run() -> bool{
    shuffled = randomShuffle(children)
    for child in shuffled{
    }
    if child.run(){
        return true
    }else{
        return false
    }
}
