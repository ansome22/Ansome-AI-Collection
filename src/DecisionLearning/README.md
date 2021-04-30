# DECISION LEARNING

So far I have described learning algorithms that operate on relatively restricted domains: the
value of a parameter and predicting a series of player choices from a limited set of options.
To realize the potential of learning AI, we would need to allow the AI to learn to make
decisions. Chapter 5 outlined several methods for making decisions; the following sections
look at decision makers that choose based on their experience.
These approaches cannot replace the basic decision making tools. State machines, for
example, explicitly limit the ability of a character to make decisions that are not applicable in
a situation (no point choosing to fire if your weapon has no ammo, for example). Learning
is probabilistic; you will usually have some probability (however small) of carrying out each
possible action. Learning hard constraints is notoriously difficult to combine with learning
general patterns of behavior suitable for outwitting human opponents.

## THE STRUCTURE OF DECISION LEARNING

We can simplify the decision learning process into an easy to understand model. Our learning
character has some set of behavior options that it can choose from. These may be steering
behaviors, animations, or high-level strategies in a war game. In addition, it has some set of
observable values that it can get from the game level. These may include the distance to the
nearest enemy, the amount of ammo left, the relative size of each player’s army, and so on.

## Weak or Strong Supervision

In order to improve performance, we need to provide feedback to the learning algorithm. This
feedback is called “supervision,” and there are two varieties of supervision used by different
learning algorithms or by different flavors of the same algorithm.
Strong supervision takes the form of a set of correct answers. A series of observations
are each associated with the behavior that should be chosen. The learning algorithm learns
to choose the correct behavior given the observation inputs. These correct answers are often
provided by a human player. The developer may play the game for a while and have the AI
watch. The AI keeps track of the sets of observations and the decisions that the human player
makes. It can then learn to act in the same way.
Weak supervision doesn’t require a set of correct answers. Instead, some feedback is given
as to how good its action choices are. This can be feedback given by a developer, but more
commonly it is provided by an algorithm that monitors the AI’s performance in the game.
If the AI gets shot, then the performance monitor will provide negative feedback. If the AI
consistently beats its enemies, then feedback will be positive.
Strong supervision is easier to implement and get right, but it is less flexible: it requires
somebody to teach the algorithm what is right and wrong. Weak supervision can learn right
and wrong for itself, but is much more difficult to get right.
