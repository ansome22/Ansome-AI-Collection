use std::{any, collections::HashMap};

pub struct NGramPredictor{
    // The frequency data.
     data: HashMap<KeyDataRecord>,
    // The size of the window + 1.
     nValue: i32,
}

// Register a set of actions with predictor, updating its data. We7.3 Action Prediction 595
// assume actions has exactly nValue elements in it.
fn registerSequence(actions: Vec<any>){
    // Split the sequence into a key and value.
    let key = actions[0..nValue];
    let value = actions[nValue];

    // Make sure we’ve got storage.
    if (!key in &self.data) {
        let keyData = data[key] = new KeyDataRecord()
    }
    else{
        let keyData = data[key];
        
        // Add to the total, and to the count for the value.
        keyData.counts[value] += 1;
        keyData.total += 1;
        // Get the next action most likely from the given one. We assume
        // actions has nValue - 1 elements in it (i.e. the size of the
        // window).
    }
}

fn getMostLikely(actions: Vec<any>) -> any{
    // Get the key data.
    keyData = data[actions];

    // Find the highest probability.
    highestValue = 0;
    bestAction = null;
    
    // Get the list of actions in the store.
    actions = keyData.counts.getKeys();

    // Go through each.
    for action in actions{
    // Check for the highest value.
    if keyData.counts[action] > highestValue{
        // Store the action.
        highestValue = keyData.counts[action];
        bestAction = action;
    }
    }
    // We’ve looked through all actions, if best action is still
    // null, then its because we have no data on the given window.
    // Otherwise we have the best action to take.
    return bestAction
}

