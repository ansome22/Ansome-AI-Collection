fn entropy(examples){
// Get the number of examples.
exampleCount = examples.length();

// Check if we only have one: in that case entropy is 0.
if exampleCount == 0{
    return 0
}

// Otherwise we need to keep a tally of how many of each
// different kind of action we have.
actionTallies = {};

// Go through each example.
for example in examples{
    // Increment the appropriate tally.
    actionTallies[example.action]++;

    // We now have the counts for each action in the set.
    actionCount = actionTallies.length();

    // If we have only one action then we have zero entropy.
    if actionCount == 0{
        return 0
    }
}
// Start with zero entropy.
entropy = 0;

// Add in the contribution to entropy of each action.
for actionTally in actionTallies{
    proportion = actionTally / exampleCount;
    entropy -= proportion * log2(proportion);
}

// Return the total entropy.
return entropy
}

