# FUZZY STATE MACHINES

Although developers often talk about fuzzy state machines, they don’t always mean the same
thing by it. A fuzzy state machine can be any state machine with some element of fuzziness.5.5 Fuzzy Logic 395
It can have transitions that use fuzzy logic to trigger, or it might use fuzzy states rather than
conventional states. It could even do both.
Although I’ve seen several approaches, with none of them particularly widespread, as an
example I will illustrate the approach with a simple state machine that uses fuzzy states but
crisp triggers for transitions.

## Performance

The algorithm requires temporary storage for each active state and therefore is O(n) in memory, where n is the number of active states (i.e., those with DOM > 0).
The algorithm looks at each transition for each active state and therefore is O(nm) in time,
where m is the number of transitions per state.
As in all previous decision-making tools, the performance and memory requirements can
easily be much higher if the algorithms in any of its data structures are not O(1) in both time
and memory.
