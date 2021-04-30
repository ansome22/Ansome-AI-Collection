fn boltzmannAnnealParameters(parameters, func, temp){
    // Store the initial value.
    initialValue = func(parameters);
    
    // Loop through each parameter.
    for i in 0..parameters.size(){
        // Store the current parameter value.
        currentParameter = parameters[i].value;
        
        // Tweak it both up and down.
        for tweak in [-STEP, STEP]{
        // Apply the tweak.
        parameters[i].value += tweak;
        
        // Get the value of the fn.
        value = func(parameters[i]);
        
        // Is it the best so far?
        if value < initialValue{
            // Return it.
            return parameters
        }
        // Otherwise check if we should do it anyway.
        else{
        // Calculate the energy gain and coefficient.
        energyGain = value - initialValue;
        boltzmannCoeff = exp(-energyGain / temp);
        }
        
            // Randomly decide whether to accept it.
            if random() < boltzmannCoeff{
            // We’re going with the change, return it.
            return parameters
            }
        }
        // Reset the parameter to its old value.
        parameters[i].value = currentParameter
    }
    // We found no better parameters, return the originals.
     return parameters    
}
