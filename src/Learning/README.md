# Learning

## Online or offline

Learning can be performed during the game, while the player is playing. This is online learning, and it allows the characters to adapt dynamically to the player’s style and provides more
consistent challenges.

Unfortunately, online learning also produces problems with predictability and testing. If
the game is constantly changing, it can be difficult to replicate bugs and problems. If an enemy
character decides that the best way to tackle the player is to run into a wall, then it can be
a nightmare to replicate the behavior (at worst you’d have to play through the whole same
sequence of games, doing exactly the same thing each time as the player).

The majority of learning in game AI is done offline, either between levels of the game or
more often at the development studio before the game leaves the building. This is performed
by processing data about real games and trying to calculate strategies or parameters from
them.
This allows more unpredictable learning algorithms to be tried out and their results to be
tested exhaustively. The learning algorithms in games are usually applied offline; it is rare to
find games that use any kind of online learning.

## INTRA-BEHAVIOR LEARNING

he simplest kinds of learning are those that change a small area of a character’s behavior.
They don’t change the whole quality of the behavior, but simply tweak it a little. These intrabehavior learning techniques are easy to control and can be easy to test.

## INTER-BEHAVIOR LEARNING

The frontier for learning AI in games is learning of behavior. What I mean by behavior is a
qualitatively different mode of action—for example, a character that learns the best way to
kill an enemy is to lay an ambush or a character that learns to tie a rope across a backstreet to
stop an escaping motorbiker. Characters that can learn from scratch how to act in the game
provide a challenging opposition for even the best human players.

## A WARNING

In reality, learning is not as widely used as you might think. Some of this is due to the relative complexity of learning techniques (in comparison with pathfinding and movement algorithms, at least). But games developers master far more complex techniques all the time,
for graphics, network and physics simulation. The biggest problems with learning are not
difficulty, but reproducibility and quality control.
Imagine a game in which the enemy characters learn their environment and the player’s
actions over the course of several hours of gameplay. While playing one level, the QA team
notices that a group of enemies is stuck in one cavern, not moving around the whole map. It
is possible that this condition occurs only as a result of the particular set of things they have
learned.

## OVER-LEARNING

A common problem identified in much of the AI learning literature is over-fitting, or overlearning. This means that if a learning AI is exposed to a number of experiences and learns
from them, it may learn the response to only those situations. We normally want the learning
AI to be able to generalize from the limited number of experiences it has to be able to cope
with a wide range of new situations.

## HILL CLIMBING

Initially, a guess is made as to the best parameter value. This can be completely random; it
can be based on the programmer’s intuition or even on the results from a previous run of the
algorithm. This parameter value is evaluated to get a score.

### Direct Method

At each hill climbing step, a random number is added to the evaluation for each neighbor of
the current value. In this way the best neighbor is still more likely to be chosen, but it can be
overridden by a large random number. The range of the random number is initially large, but
is reduced over time.
For example, the random range is ±10, the evaluation of the current value is 0, and its
neighbors have evaluations of 20 and 39. A random number is added from the range ±10 to
each evaluation. It is possible that the first value (scoring 20) will be chosen over the second,
but only if the first gets a random number of +10 and the second gets a random number of
−10. In the vast majority of cases, the second value will be chosen.
Several steps later, the random range might be ±1, in which case the first neighbor could
never be chosen. On the other hand, at the start of the annealing, the random range might be
±100, where the first neighbor has a very good chance of being chosen.

### Boltzmann Probabilities

Motivated by the physical annealing process, the original simulated annealing algorithm used
a more complex method of introducing the random factor to hill climbing. It was based on a
slightly less complex hill climbing algorithm.
In our hill climbing algorithm we evaluate all neighbors of the current value and work out
which is the best one to move to. This is often called “steepest gradient” hill climbing, because
it moves in the direction that will bring the best results. A simpler hill climbing algorithm
will simply move as soon as it finds the first neighbor with a better score. It may not be the
best direction to move in, but is an improvement nonetheless.
