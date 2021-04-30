use std::any;

use crate::actionPrediciton::NGramPredictor::*;

pub struct HierarchicalNGramPredictor{
 // Holds an array of n-grams with increasing n values.
 ngrams: Vec<NGramPredictor>,
 // The maximum window size + 1.
 nValue: i32,
 // The minimum number of samples an n-gram must have before
 // its allowed to predict.
 threshold: i32,
}

 fn HierarchicalNGramPredictor(n: int){
    // Store the maximum n-gram size.
    let nValue = n;

    // Create the array of n-grams.
    let ngrams = NGramPredictor[nValue];

    for i in 0..nValue{
        ngrams[i].nValue = i+1;
    }
}


fn registerSequence(actions: Vec<any>){
    // Go through each n-gram.
    for i in 0..nValue{
    // Create the sub-list of actions and register it.
    subActions = actions[(nValue - i)..nValue];
    ngrams[i].registerSequence(subActions);
    }

}
    fn getMostLikely(actions: Vec<any>) -> any{
    // Go through each n-gram in descending order.
        for i in nValue..0{
            // Find the relevant n-gram.
            let ngram = ngrams[i];

            // Get the sub-list of window actions.
            let subActions = actions[i..];

            // Check if we have enough entries.
            if (subActions in ngram.data && ngram.data[subActions].count > threshold) {

            // Get the ngram to do the prediction.
            return ngram.getMostLikely(subActions);
            }
            else{
            // If we get here, it is because no n-gram is over the
            // threshold: return no action.
            return null
        }
    }
}

