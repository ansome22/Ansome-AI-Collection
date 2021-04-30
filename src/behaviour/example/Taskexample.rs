pub struct EnemyNear extends Task{

 }
2 fn run() -> bool{
    //Task fails if there is no enemy nearby.
    return distanceToEnemy < 10
}

