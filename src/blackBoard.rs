use std::any::Any;

use crate::Action::Action;

mod blackboardIteration;
mod BlackboardManager;

pub struct BlackboardDatum {
    id: String,
    bd_type: Type,
    value: dyn Any,
}

pub struct Blackboard {
    entries: Vec<BlackboardDatum>,
    passedActions: Vec<Action>,
}
