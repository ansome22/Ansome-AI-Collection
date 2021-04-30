 fn discontentment(action: Action, goals: Goal[]) -> f64{
// Keep a running total.
 discontentment: f64 = 0;

// Loop through each goal.
 for goal in action{
// Calculate the new value after the action.
 newValue = goal.value + action.getGoalChange(goal)

// Calculate the change due to time alone.
 newValue += action.getDuration() * goal.getChange()

// Get the discontentment of this value.
discontentment += goal.getDiscontentment(newValue)
}
 }

