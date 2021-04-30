fn blackboardIteration(blackboard: Blackboard,experts: Expert[]) -> Action[]{
    // Go through each expert for their insistence.
    bestExpert: Expert = null;
    highestInsistence: f64 = 0;

    for expert in experts {
    //Ask for the expert’s insistence.
    insistence: f64 = expert.getInsistence(blackboard);

    // Check against the highest value so far.
    if insistence > highestInsistence{
        highestInsistence = insistence;
        bestExpert = expert;
    }


    // Make sure somebody insisted.
    if bestExpert {
        // Give control to the most insistent expert.
        bestExpert.run(blackboard);
    }
    
    }

    // Return all passed actions from the blackboard.
    return blackboard.passedActions;
}
