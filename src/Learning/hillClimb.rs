 fn hillClimb(initial: f64[], steps: int, func) -> f64[]{
   // Set the initial parameter settings.
   parameters: f64[] = initial;

   // Find the initial value for the initial parameters.
   value: f64 = func(parameters);

   // Go through a number of steps.
   for i in 0..steps{
      // Get the new parameter settings.
      newParameters: f64[] = optimizeParameters(parameters, func);

      // Get the new value.
      newValue: f64 = func(newParameters);

      // If we can’t improve, then end.584 Chapter 7 Learning
      if newValue <= value{
         break
      }

      // Store the new value for next iteration.
      value = newValue;
      parameters = newParameters;
   }
   // We’ve either run out of steps, or we can’t improve.
   return parameters
}
