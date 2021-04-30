# Behaviour

## BehaviourTree

- The onInitialize() method is called once, immediately before the first call to
  the behavior’s update method.
- The update() method is called exactly once each time the behavior tree is
  updated, until it signals it has terminated thanks to its return status.
- The onTerminate() method is called once, immediately after the previous
  update signals it’s no longer running.

### Return Statuses

Each behavior, when executed, passes back a return status. The return status is a critical
part of any behavior tree, without which it simply wouldn’t work. In practice, return statuses plays two roles:
• Completion Status—If the behavior has terminated, the return status indicates whether it achieved its purpose. There are two completion statuses most
commonly used: SUCCESS (indicates that everything went as expected) and
FAILURE (specifies that something apparently went wrong).
• Execution Hints—While the behavior is running, each update of the behavior
also returns a status code. Most of the time, this is RUNNING, but modern BTs
can leverage this status code to provide much more efficient implementations. For
example, the SUSPENDED status code is an essential part of an event-driven BT

## LIMITATIONS OF BEHAVIOR TREES

Over the last decade, behavior trees have come from nowhere to become a core technique in
game AI, aided by good tool support in popular game engines. Inevitably with any hype, it
can be easy to see them as a solution to almost every problem you can imagine in game AI. It374 Chapter 5 Decision Making
is worth being a little cautious. Understanding what behavior trees are bad at is as important
as understanding where they excel.
We’ve already seen a key limitation of behavior trees. They are reasonably clunky when
representing the kind of state-based behavior that we met in the previous section. Not all
state-based behavior causes problems. If your character transitions between types of behavior
purely because of the success or failure of actions (so they get mad when they can’t do something, for example), then behavior trees work fine. But they become much more cumbersome
if you have a character who needs to respond to external events—interrupting a patrol route
to suddenly go into hiding or to raise an alarm, for example—or a character than needs to
switch strategies when its ammo is looking low. I’m not claiming those behaviors can’t be
implemented in behavior trees, just that it would be less idiomatic to do so.
Because behavior trees make it more difficult to think and design in terms of states, AI
based solely on behavior trees tends to avoid these kinds of behavior. If you look at a behavior
tree created by an artist or level designer, they tend to avoid noticeable changes of character
disposition or alarm behavior. This is a shame, since those cues are simple and powerful and
help raise the level of the AI.
We can build a hybrid system, of course, where characters have multiple behavior trees
and use a state machine to determine which behavior tree they are currently running. Using
the approach of having behavior tree libraries that we saw above, this provides the best of
both worlds. Unfortunately, it also adds considerable extra burden to the AI authors and
toolchain developers, since they now need to support two kinds of authoring: state machines
and behavior trees.
An alternative approach would be to create tasks in the behavior tree that behave like state
machines—detecting important events and terminating the current sub-tree to begin another.
This merely moves the authoring difficulty, however, as we still need to build a system for AI
authors to parameterize these relatively complex tasks.
Behavior trees on their own have been a big win for game AI, and developers will still be
exploring their potential for many years. As long as they are pushing forward the state of the
art, I suspect that there will not be a strong consensus on how best to avoid these limitations,
with developers and tool vendors experimenting with their own approaches.
