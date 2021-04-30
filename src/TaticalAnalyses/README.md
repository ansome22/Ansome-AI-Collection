# Tactical analyses

Tactical analyses of all kinds are sometimes known as influence maps. Influence mapping is a
technique pioneered and widely applied in real-time strategy games, where the AI keeps track
of the areas of military influence for both sides. Similar techniques have also made inroads
into squad-based shooters and massively multi-player games. For this chapter, I will refer to
the general approach as tactical analysis to emphasize that military influence is only one thing
we might base our tactics on.
In military simulation an almost identical approach is commonly called terrain analysis
(a phrase also used in game AI), although again that also more properly refers to just one type
of tactical analysis. I will describe both influence mapping and terrain analysis in this section,
as well as general tactical analysis architectures.
There is not much difference between tactical waypoint approaches and tactical analyses.
By and large, papers and talks on AI have treated them as separate beasts, and admittedly512 Chapter 6 Tactical and Strategic AI
the technical problems are different depending on the genre of game being implemented. The
general theory is remarkably similar, however, and the constraints in some games (in shooters,
particularly) mean that implementing the two approaches would give pretty much the same
structure.
