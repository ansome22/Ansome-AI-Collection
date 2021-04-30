# DECISION TREE LEARNING

Decision trees: a series of decisions that generate an action to take
based on a set of observations. At each branch of the tree some aspect of the game world was
considered and a different branch was chosen. Eventually, the series of branches lead to an
action.

Trees with many branch points can be very specific and make decisions based on the
intricate detail of their observations. Shallow trees, with only a few branches, give broad and
general behaviors.
Decision trees can be efficiently learned: constructed dynamically from sets of observations and actions provided through strong supervision. The constructed trees can then be used in the normal way to make decisions during gameplay.
There are a range of different decision tree learning algorithms used for classification,
prediction, and statistical analysis. Those used in game AI are typically based on Quinlan’s
ID3 algorithm, which we will examine in this section.
