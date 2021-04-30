pub struct PlayAnimation extends Task{
 animationId: int,
speed: f64 = 1.0,
}

//Chapter 5 Decision Making
fn run() -> bool{
    if animationEngine.ready(){
    animationEngine.play(animationId, speed);
    return true
}else{
    // Task failure, the animation could not be played.
    return false
}
}
