
pub struct NaiveBayes {
    // Number of positive examples, none initially.
    examplesCountPositive = 0,
    // Number of negative examples, none initially.
    examplesCountNegative = 0,
    // Number of times each attribute was true for the positive
    // examples, initially all zero.
    attributeCountsPositive[NUM_ATTRIBUTES] = zeros(NUM_ATTRIBUTES),
    // Number of times each attribute was true for the negative
    // examples, initially all zero.
    attributeCountsNegative[NUM_ATTRIBUTES] = zeros(NUM_ATTRIBUTES),
}

fn update(attributes: bool[], label: bool){
// Check if this is a positive or negative example, update all
// the counts accordingly.
    if label{
// Using element-wise addition.
    attributeCountsPositive += attributes;
    examplesCountPositive += 1;
    }else{
    attributeCountsNegative += attributes;
    examplesCountNegative += 1;
}
}
fn predict(attributes: bool[]) -> bool{
// Predict must label this example as a positive or negative
// example.
x = naiveProbabilities(attributes,attributeCountsPositive, f64(examplesCountPositive), f64(examplesCountNegative));
y = naiveProbabilities(attributes, attributeCountsNegative, f64(examplesCountNegative), f64(examplesCountPositive));
return x >= y
}

fn naiveProbabilities(attributes: bool[],counts: int,m: f64,n: f64) -> f64{
// Compute the prior.
prior = m / (m + n);

// Naive assumption of conditional independence.
p = 1.0;

for i in 0..NUM_ATTRIBUTES{
    p /= m;
    if attributes[i]{
        p *= counts[i]
    }else{
        p *= m - counts[i];
    }
    }
    return prior * p
}