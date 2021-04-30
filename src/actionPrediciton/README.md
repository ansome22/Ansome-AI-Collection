# ACTION PREDICTION

It is often useful to be able to guess what players will do next. Whether it is guessing which
passage they are going to take, which weapon they will select, or which route they will attack
from, a game that can predict a player’s actions can mount a more challenging opposition.
Humans are notoriously bad at behaving randomly. Psychological research has been carried out over decades and shows that we cannot accurately randomize our responses, even if
we specifically try. Mind magicians and expert poker players make use of this. They can often
easily work out what we’ll do or think next based on a relatively small amount of experience
of what we’ve done in the past.
Often, it isn’t even necessary to observe the actions of the same player. We have shared
characteristics that run so deep that learning to anticipate one player’s actions can often lead
to better play against a completely different player.

## RAW PROBABILITY

The simplest way to predict the choice of a player is to keep a tally of the number of times they
choose each option. This will then form a raw probability of that player choosing that action
again.
For example, after 20 times through a level, if the first passage has been chosen 72 times,
and the second passage has been chosen 28 times, then the AI will be able to predict that a
player will choose the first route.
Of course, if the AI then always lays in wait for the player in the first route, the player will
very quickly learn to use the second route.
This kind of raw probability prediction is very easy to implement, but it gives a lot of
feedback to the player, who can use the feedback to make their decisions more random.
In the above example, the character will position itself on the most likely route. The player
will only fall foul of this once and then will use the other route. The character will continue
standing where the player isn’t until the probabilities balance. Eventually, the player will learn
to simply alternate different routes and always miss the character.
When the choice is made only once, then this kind of prediction may be all that is possible.
If the probabilities are gained from many different players, then it can be a good indicator of
which way a new player will go.
Often, a series of choices must be made, either repeats of the same choice or a series of
different choices. The early choices can have good predictive power over the later choices. We
can do much better than using raw probabilities.

## STRING MATCHING

When a choice is repeated several times (the selection of cover points or weapons when enemies attack, for example), a simple string matching algorithm can provide good prediction.
The sequence of choices made is stored as a string (it can be a string of numbers or objects,
not just a string of characters). In the left-and-right game this may look like “LRRLRLLLRRLRLRR,” for example. To predict the next choice, the last few choices are searched for in the
string, and the choice that normally follows is used as the prediction

## N-GRAMS

The string matching technique is rarely implemented by matching against a string. It is more
common to use a set of probabilities similar to the raw probability in the previous section. This594 Chapter 7 Learning
is known as an N-Gram predictor (where N is one greater than the window size parameter,
so 3-Gram would be a predictor with a window size of two).
In an N-Gram we keep a record of the probabilities of making

## WINDOW SIZE

Increasing the window size initially increases the performance of the prediction algorithm.
For each additional action in the window, the improvement reduces until there is no benefit
to having a larger window, and eventually the prediction gets worse with a larger window until
we end up making worse predictions than we would if we simply guessed at random.
This is because, while our future actions are predicted by our preceding actions, this is
rarely a long causal process. We are drawn toward certain actions and short sequences of
actions, but longer sequences only occur because they are made up of the shorter sequences.
If there is a certain degree of randomness in our actions, then a very long sequence will likely
have a fair degree of randomness in it. The very large window size is likely to include more
randomness and, therefore, be a poor predictor. There is a balance in having a large enough
window to accurately capture the way our actions influence each other, without being so long
that it gets foiled by our randomness. As the sequence of actions gets more random, the
window size needs to be reduced.

## HIERARCHICAL N-GRAMS

When an N-Gram algorithm is used for online learning, there is a balance between the maximum predictive power and the performance of the algorithm during the initial stages of learning. A larger window size may improve the potential performance, but will mean that the
algorithm takes longer to get to a reasonable performance level.
The hierarchical N-Gram algorithm effectively has several N-Gram algorithms working
in parallel, each with increasingly large window sizes. A hierarchical 3-Gram will have regular
1-Gram (i.e., the raw probability approach), 2-Gram, and 3-Gram algorithms working on the
same data.

### Performance

The algorithm is O(n) in memory and O(n) in time, where n is the highest numbered N-Gram
used.
The registerSequence method uses the O(1) registerSequence method of the N-Gram
class, so it is O(n) overall. The getMostLikely method uses the O(n) getMostLikely method
of the N-Gram class once, so it is O(n) overall.

### APPLICATION IN COMBAT

By far the most widespread game application of N-Gram prediction is in combat games. Beatem-ups, sword combat games, and any other combo-based melee games involve timed sequences of moves. Using an N-Gram predictor allows the AI to predict what the player is
trying to do as they start their sequence of moves. It can then select an appropriate rebuttal.
This approach is so powerful, however, that it can provide unbeatable AI. A common
requirement in this kind of game is to remove competency from the AI so that the player has
a sporting chance.
