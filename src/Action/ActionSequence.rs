use crate::Action::Action;

pub struct ActionSequence{
    actions: Vec<Action>,
    // The index of the currently executing sub-action.
    activeIndex: i32,
 }

impl ActionSequence {
    pub fn new(actions: Vec<Action>, activeIndex: i32) -> Self { Self { actions, activeIndex } }


    fn interrupt(&self) -> bool{
        // We can interrupt if our first sub-actions can.
        return &self.actions[0].interrupt()
    }
    
    fn canDoBoth(&self, other: Action) -> bool{
        // We can do both if all of our sub-actions can If we only
        // tested the first one, we’d be in danger of suddenly finding
        // ourselves incompatible mid-sequence.
        &self.actions.into_iter().for_each(|action| {
            if !action.canDoBoth(other){
                return false
            }
            else {
                return true
            }
        });
    }
    
    fn isComplete(&self) -> bool{
        return &self.activeIndex >= &self.actions.len();
    }
    // We are complete if all of our sub-actions are.
    // Execute our current action.478 Chapter 5 Decision Making
    fn execute(&self) {
        &self.actions[&self.activeIndex].execute();
    
        // If our current action is complete, go to the next.
        if &self.actions[&self.activeIndex].isComplete(){
            &self.activeIndex += 1;
        }
    }
}





