use crate::Action::Action;

pub struct ActionCombination{
    // The sub-actions.
    actions: Vec<Action>,
}

impl ActionCombination {
    pub fn new(actions: Vec<Action>) -> Self { Self { actions } }

    fn interrupt(&self) -> bool{
        // We can interrupt if any of our sub-actions can.
        &for action in self.actions.iter() {
            if action.interrupt(){
                return true;
            }else{
                return false;
            }
        };
        
    }
    
    fn canDoBoth(&self, other: Action) -> bool{
    // We can do both if all of our sub-actions can.
    &for action in self.actions.iter() {
            if !action.canDoBoth(other){
                return false;
            }else{
                return true;
            }
        };
    }
    
    fn isComplete(self) -> bool{
        // We are complete if all of our sub-actions are.
        &for action in self.actions.iter() {
            if !action.isComplete(){
                return false;
            }else{
                return true;
            }
        }
    }
    
    fn execute(self){
        // Execute all of our sub-actions.
        &for action in self.actions.iter() {
            action.execute();
        }
    }
    

    /// Get a reference to the action combination's actions.
    pub fn actions(&self) -> &Vec<Action> {
        &self.actions
    }

    /// Set the action combination's actions.
    pub fn set_actions(&mut self, actions: Vec<Action>) {
        self.actions = actions;
    }
}


