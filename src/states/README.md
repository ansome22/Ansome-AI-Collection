# States

In a state machine each character occupies one state. Normally, actions or behaviors are associated with each state. So, as long as the character remains in that state, it will continue
carrying out the same action.
States are connected together by transitions. Each transition leads from one state to another, the target state, and each has a set of associated conditions. If the game determines that
the conditions of a transition are met, then the character changes state to the transition’s target state. When a transition’s conditions are met, it is said to trigger, and when the transition
is followed to a new state, it has fired.

## Weaknesses

This approach to transitions gives a lot of flexibility, but at the price of lots of method calls.
These method calls have to be polymorphic, which can slow down the call and confuse the
processor. All this adds time, which may make it unsuitable for use in every frame on lots of
characters. The slowdown is minor however. On modern hardware the complexity of a state
machine is unlikely to be a performance bottleneck.
One advantage of this approach, that further doubles down on the polymorphic nature of
the implementation, is that transitions can be implemented in a scripting language for special
gameplay purposes. These are obviously even slower to execute, but the flexibility is often
worth it, allowing gameplay logic to be created simply and merged seamlessly into the rest of
the AI system.

## Weaknesses

This approach to transitions gives a lot of flexibility, but at the price of lots of method calls.
These method calls have to be polymorphic, which can slow down the call and confuse the
processor. All this adds time, which may make it unsuitable for use in every frame on lots of
characters. The slowdown is minor however. On modern hardware the complexity of a state
machine is unlikely to be a performance bottleneck.
One advantage of this approach, that further doubles down on the polymorphic nature of
the implementation, is that transitions can be implemented in a scripting language for special
gameplay purposes. These are obviously even slower to execute, but the flexibility is often
worth it, allowing gameplay logic to be created simply and merged seamlessly into the rest of
the AI system.

## Decorators

The name “decorator” is taken from object-oriented software engineering. The decorator
pattern refers to a class that wraps another class, modifying its behavior. If the decorator has
the same interface as the class it wraps, then the rest of the software doesn’t need to know if it
is dealing with the original class or the decorator.
In the context of a behavior tree, a Decorator is a type of task that has one single child
task and modifies its behavior in some way. You could think of it like a Composite task with
a single child. Unlike the handful of Composite tasks we’ll meet, however, there are many
different types of useful Decorators.
One simple and very common category of Decorators makes a decision whether to allow
their child behavior to run or not (they are sometimes called “filters”).
