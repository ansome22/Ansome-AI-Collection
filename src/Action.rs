mod ActionCombination;
mod ActionManager;
mod ActionSequence;


pub struct Action{
    expiryTime: f64,
    priority: f64
 }

pub trait Action2 {
    fn interrupt() -> bool;
    fn can_do_both(other: Action) -> bool;
    fn is_complete() -> bool;
}