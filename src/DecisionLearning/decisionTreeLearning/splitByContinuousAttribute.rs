 fn splitByContinuousAttribute(examples, attribute){
    // We create a set of lists, so we can access each list
    // by the attribute value.620 Chapter 7 Learning
    bestGain = 0;
    bestSets;

    // Make sure the examples are sorted.
    setA = [];
    setB = sortReversed(examples, attribute);

    // Work out the number of examples and initial entropy.
    exampleCount = len(examples);
    initialEntropy = entropy(examples);

    // Go through each but the last example,
    // moving it to set A.
    while setB.length() > 1{
        // Move the lowest example from A to B.
        setB.push(setA.pop());
    }


    // Find overall entropy and information gain.
    overallEntropy = entropyOfSets(setA, setB], exampleCount);
    informationGain = initialEntropy - overallEntropy;

    // Check if it is the best.
    if informationGain >= bestGain{
        bestGain = informationGain;
        bestSets = [setA, setB];
    }


    // Calculate the threshold.
    setA = bestSets[0];
    setB = bestSets[1];
    threshold = setA[setA.length()-1].getValue(attribute);
    threshold += setB[setB.length()-1].getValue(attribute);
    threshold /= 2;

    // Return the sets.
    return bestSets, threshold;
}
