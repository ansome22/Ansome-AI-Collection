 fn fuzzyDecisionMaker(inputs: any[], membershipFns: Membershipfn[][], rules: FuzzyRule[]) -> f64[]{
// Will hold the degrees of membership for each input state and
// output state, respectively.
 inputDom: f64[] = [];
 outputDom: f64[] = [0, 0, ..., 0];

// Convert the inputs into state values.
 for i in 0..len(inputs):
// Get the input value.
 input = inputs[i];

// Get the membership fns for this input.
 membershipFnList = membershipFns[i];

// Go through each membership fn.
 for membershipFn in membershipFnList:
// Convert the input into a degree of membership.
 inputDom[membershipFn.stateId] = membershipFn.dom(input);

// Go through each rule.
 for rule in rules:
// Get the current output d.o.m. for the conclusion state.
 best = outputDom[rule.conclusionStateId];

// Hold the minimum of the inputDoms seen so far.
 min = 1;

// Go through each state in the input of the rule.
 for state in rule.inputStateIds:
// Get the d.o.m. for this input state.
dom = inputDom[state];

// If we’re smaller than the best conclusion so far, we may
// as well exit now, because even if we are the smallest in
// this rule, the conclusion will not be the best overall.
 if dom < best:
break continue// i.e., go to next rule.

// Check if we’re the lowest input d.o.m. so far.
 if dom < min:
 min = dom;

// min now holds the smallest d.o.m. of the inputs, and because
// we didn’t break above, we know it is larger than the current
// best, so write the current best.
 outputDom[rule.conclusionStateId] = min;

// Return the output state degrees of membership.
return outputDom
}