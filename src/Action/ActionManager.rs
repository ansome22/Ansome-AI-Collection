use crate::Action::Action;

pub struct ActionManager{
// The queue of pending actions.
    queue: Vec<Action>,
//The currently executing actions.
    active: Vec<Action>,
}

impl ActionManager {
    pub fn new(queue: Vec<Action>, active: Vec<Action>) -> Self { Self { queue, active } }

        // Add an action to the queue.
    fn scheduleAction(&mut self, action: Action){
        // Add it to the queue.
        self.queue += action;
    }


    // Process the manager.
    fn execute(&self){
        let currentTime = getTime();
        let priorityCutoff = &self.active.getHighestPriority();
        let mut newvec = &self.queue.to_vec();

    
    // Remove expired actions from queue.
    for action in newvec{
        if action.expiryTime < currentTime{
            &self.queue -= action;
        }
    }

    // Go through the queue.
    //let mut newvec = &self.queue.to_vec();
    for action in newvec {
        // If we drop below active priority, give up.
        if action.priority <= priorityCutoff{
            break
        }
        // If we have an interrupter, do it instead.
        if action.interrupt(){
            &self.queue -= action

            // Interrupter is now the only active, previous
            // active are discarded.
            &self.active = [action];
            priorityCutoff = action.priority;
        }
        else{
        // Check if we can add this action.
        canAddToActive = true
        }

    }

    for activeAction in active{
        if !activeAction.canDoBoth(action){
            canAddToActive = false;
            break
        }

        // If we can do both, then do.
        if canAddToActive{
            &self.queue -= action;
            &self.active += action;
            priorityCutoff = action.priority;
        }
    }



    // Remove or run active actions.
    for activeAction in copy(&self.active){
        if activeAction.isComplete(){
            self.active -= activeAction;
        }
        else{
            activeAction.execute()
        }
    }

    }

    /// Get a reference to the action manager's queue.
    pub fn queue(&self) -> &Action {
        &self.queue
    }

    /// Set the action manager's queue.
    pub fn set_queue(&mut self, queue: Action) {
        self.queue = queue;
    }
}


