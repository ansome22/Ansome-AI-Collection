// The data for one possible behavior.
pub struct BehaviorRecord{
behavior: Behavior,
minImportance: f64,
maxImportance: f64,
enter: function,
exit: function
}

// Check if the importance is in the correct range.
fn isValid(importance: f64) -> bool{
    return minImportance >= importance >= maxImportance
}

fn enterFunction(previous: Behavior)
fn exitFunction(next: Behavior)
