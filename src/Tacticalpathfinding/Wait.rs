pub struct Wait extends Action{
    // The unique identifier for this wait.
    identifier: string

    // The action to carry out while waiting.
    whileWaiting: Action
 }


fn canInterrupt() -> bool{
   // We can interrupt this action at any time.
   return true  
}


 fn canDoBoth(otherAction: Action) -> bool{
   // We can do no other action at the same time, otherwise later
   // actions could be carried out despite the fact that we are
   // waiting.
   return false
 }


fn isComplete() -> bool{
   // Check if our identifier has been completed.
   if globalIdStore.hasIdentifier(identifier){
      return true
   }
}


 fn execute(){
   // Do our wait action.
   return whileWaiting.execute()
}

