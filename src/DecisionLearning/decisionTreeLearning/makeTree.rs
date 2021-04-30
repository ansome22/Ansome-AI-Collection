fn makeTree(examples, attributes, decisionNode):
// Calculate our initial entropy.
3 initialEntropy = entropy(examples)614 Chapter 7 Learning
4
// If we have no entropy, we can’t divide further.
6 if initialEntropy <= 0:
7 return
8
// Find the number of examples.
10 exampleCount = examples.length()
11
// Hold the best found split so far.
13 bestInformationGain = 0
14 bestSplitAttribute
15 bestSets
16
// Go through each attribute.
18 for attribute in attributes:
// Perform the split.
20 sets = splitByAttribute(examples, attribute)
21
// Find overall entropy and information gain.
23 overallEntropy = entropyOfSets(sets, exampleCount)
24 informationGain = initialEntropy - overallEntropy
25
// Check if we’ve got the best so far.
27 if informationGain > bestInformationGain:
28 bestInformationGain = informationGain
29 bestSplitAttribute = attribute
30 bestSets = sets
31
// Set the decision node’s test.
33 decisionNode.testValue = bestSplitAttribute
34
// The list of attributes to pass on down the tree should
// have the one we’re using removed.
37 newAttributes = copy(attributes)
38 newAttributes -= bestSplitAttribute
39
// Fill the daughter nodes.
41 for set in bestSets:
// Find the value for the attribute in this set.
43 attributeValue = set[0].getValue(bestSplitAttribute)
44
// Create a daughter node for the tree.
46 daughter = new MultiDecision()
47
// Add it to the tree.
49 decisionNode.daughterNodes[attributeValue] = daughter
507.6 Decision Tree Learning 615
// Recurse the algorithm.
52 makeTree(set, newAttributes, daughter)
