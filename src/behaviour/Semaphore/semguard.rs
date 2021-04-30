pub struct SemaphoreGuard extends Decorator{
    // The semaphore that we’re using to guard a resource.
    semaphore: Semaphore
}


fn run() -> bool{
    if semaphore.acquire(){
    result = child.run()
    semaphore.release()
    return result
}else{
    return false
}
}
