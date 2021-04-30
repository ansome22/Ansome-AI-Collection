Tactical pathfinding combines the tactical analyses we’ve seen earlier in the chapter with the
pathfinding techniques from Chapter 4. It can provide quite impressive results when characters in the game move, taking account of their tactical surroundings, staying in cover, and
avoiding enemy lines of fire and common ambush points.
Tactical pathfinding has an unfair reputation for being significantly more sophisticated
than regular pathfinding. As a marketing feature that may be a benefit, but if it dissuades
programmers from attempting implementation, the reputation is unfortunate. In reality it is
no different at all from regular pathfinding. The same pathfinding algorithms are used on the
same kind of graph representation. The only modification is that the cost fn is extended
to include tactical information as well as distance or time.

## THE COST fn

The cost for moving along a connection in the graph should be based on both distance/time
(otherwise, we might embark on exceptionally long routes) and how tactically sensible the
maneuver is.

We can achieve this most simply by using signals. In place of an action in the sequence,
we allow two new kinds of entity: signal and wait.
Signal: A signal has an identifier. It is a message sent to anyone else who is interested. This
is typically any other AI behavior, although it could also be sent through an event or
sense simulation mechanism from Chapter 11 if finer control is needed.
Wait: A wait also has an identifier. It stops any elements of the script from progressing unless
it receives a matching signal.
