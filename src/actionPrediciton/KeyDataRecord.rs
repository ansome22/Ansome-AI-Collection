use std::collections::HashMap;
 pub struct KeyDataRecord{
    // The counts for each successor action.
    counts: HashMap<i32>,
    // The number of times the window has been seen.
    total: i32,
 }
