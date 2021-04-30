fn chooseAction(actions: Action[], goals: Goal[]) -> Action{
    // Find the action leading to the lowest discontentment.
     bestAction: Action = null;
     bestValue: f64 = infinity;
    
     for action in actions{
     thisValue = discontentment(action, goals)
     if thisValue < bestValue{
        bestValue = thisValue
        bestAction = action
     }
     
    }
    return bestAction
}

    fn discontentment(action: Action, goals: Goal[]) -> f64{
    // Keep a running total.
     discontentment: f64 = 0;
    
    // Loop through each goal.
     for goal in goals{
    // Calculate the new value after the action.
    newValue = goal.value + action.getGoalChange(goal);
    
    // Get the discontentment of this value.
     discontentment += goal.getDiscontentment(value);
     }

    return discontentment
    }


