# GOAL-ORIENTED BEHAVIOR

So far I have focused on approaches that react: a set of inputs is provided to the character, and
a behavior selects an appropriate action. There is no implementation of desires or goals. The
character merely reacts to input.
It is possible, of course, to make the character seem like it has goals or desires, even with
the simplest decision making techniques. A character whose desire is to kill an enemy will
hunt one down, will react to the appearance of an enemy by attacking, and will search for an
enemy when there is a lack of one. The same character may also have the apparent desire to
stay alive, in which case it will take account of its own protection, reacting to low health or
the presence of danger. The underlying structure may be reacting to input, but the character
doesn’t need to appear that way.
In my experience, this is a misunderstanding that people who know academic AI have
about game AI. Simple techniques can be made to appear more intelligent because the game
world is inherently limited and the designer can constrain the way the player interacts with
it. Smoke and mirrors is not an insult, it is the way we work. Because, for players, it doesn’t
matter what is controlling the character, as long as it looks right.
Simple techniques take us a long way. But there are more advanced approaches we can use
to make the character more flexible in its goal seeking. In some game genres this is a useful
approach. It is particularly visible in people simulation games, such as The Sims [136].
Here, only a few characters are on-screen at one time. Each has a range of emotional
and physical parameters that change over time in relation to its environment and its actions.
The player can often control the character’s actions directly, although the character is always
capable of independent action.
In a game such as The Sims, there is no overall goal to the game. In other titles such as
Ghost Master [177], there is a definite aim (you try to scare the inhabitants out of a house
using various ghosts and supernatural powers).
In this kind of game a wide range of different actions is available to characters. Actions
might include boiling the kettle, sitting on a sofa, or talking to another character. The action
itself is represented by a canned animation.
Characters need to demonstrate their emotional and physical state by choosing appropriate actions. They should eat when hungry, sleep when tired, chat to friends when lonely, and
hug when in need of love. We could simply run a decision tree that selects available actions406 Chapter 5 Decision Making
based on the current emotional and physical parameters of the character. In a game such as
The Sims, this would lead to a very big decision tree. There are hundreds of parameterized
actions to choose from for every character.
A better approach would be to present the character with a suite of possible actions and
have it choose the one that best meets its immediate needs.
This is goal-oriented behavior (GOB), explicitly seeking to fulfill the character’s internal
goals. Like many algorithms in this book, the name can only be loosely applied. GOB may
mean different things to different people, and it is often used either vaguely to refer to any
goal seeking decision maker or to specific algorithms similar to those here. I will use it as a
general term.
In this chapter we will begin with a very simple GOB framework and use it to implement a utility-based GOB decision maker. I will later describe goal-oriented action planning
(GOAP), an extension to the basic GOB system that can plan sequences of actions to achieve
its goal.

## Performance

The algorithm is O(n + m) in time, where n is the number of goals, and m is the number of
possible actions. It is O(1) in memory, requiring only temporary storage. If goals are identified
by an associated zero-based integer (it is simple do, since the full range of goals is normally
known before the game runs), then the getGoalChange method of the action structure can be
simply implemented by looking up the change in an array, a constant time operation.

## Weaknesses

This approach is simple, fast, and can give surprisingly sensible results, especially in games
with a limited number of actions available (such as shooters, third-person action or adventure
games, or RPGs).
It has two major weaknesses, however: it fails to take account of side effects that an action
may have, and it doesn’t incorporate any timing information. We’ll resolve these issues in
turn.
