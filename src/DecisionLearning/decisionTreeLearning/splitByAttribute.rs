fn splitByAttribute(examples, attribute){
// Create a set of lists, so we can access each list
// by the attribute value.
 sets = {};

for example in examples{
    sets[example.getValue(attribute)] += example
}
return sets
}

