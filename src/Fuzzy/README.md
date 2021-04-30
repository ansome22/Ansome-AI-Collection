# Fuzzy

## WARNING

Fuzzy logic has been used in several games, and is applicable to many decision-making problems. It deserves its place in this book and in your toolbox of techniques. However, you
should be aware that fuzzy logic has, for valid reasons, been largely discredited within the
mainstream academic AI community for problems that probability can successfully model.
You can read more details in Russell and Norvig [54] but the executive summary is that it
is always better to use probability to represent any kind of uncertainty. Put in more concrete
terms, it has been proven that if you play any kind of zero-sum betting game, then a player
who is not basing their decisions on probability theory can expect to eventually lose money.
The reason is that flaws in any other theory of uncertainty, besides probability theory, can
potentially be exploited by an opponent.
Fuzzy logic became more popular than probability theory in games partly because it is
easier to translate simple rules (e.g. “walk slowly when cautious”). And partly because the
perception has been that using probabilistic methods can be slow. With the advent of Bayes
nets and other graphical modeling techniques, neither are such an issue. While I won’t explicitly cover Bayes networks in this book, we will look at various other related approaches
such as Markov systems.

## Performance

The algorithm is O(n + m) in memory, where n is the number of input states, and m is the
number of output states. It simply holds the degree of membership for each.
Outside the algorithm itself, the rules need to be stored.

## Weaknesses

The overwhelming weakness of this approach is its lack of scalability. It works well for a small
number of input variables and a small number of states per variable. To process a system
with 10 input variables, each with 5 states, would require almost 10 million rules. This is well
beyond the ability of anyone to create.
For larger systems of this kind, we can either use a small number of general fuzzy rules,392 Chapter 5 Decision Making
or we can use the Combs method for creating rules, where the number of rules scales linearly
with the number of input states.
