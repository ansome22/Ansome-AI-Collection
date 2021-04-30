pub struct SelectTarget extends Task{
    blackboard: Blackboard
} 

 fn run() -> bool{
    character = blackboard.get(’character’);
    candidates = enemiesVisibleTo(character);
     if candidates.length > 0{
        target = biggestThreat(candidates, character);
        blackboard.set(’target’, target);
        return true
     }

     else{
        return false
     }

 }

