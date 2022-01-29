# Examples

asdasdasd

problem size really seems to matter eg for beach grass forest. 60 it sseems to work like a charm but sometimes else it gets rekt
its quite polymodal you get some that are all forest and some that are all beach, which is kind acool
you can see how a genetic algorithm could allow you to specify a desired distribution in the cost function that it could try to hit


pluroads with JUST small rollbacks seems to work quite well
flower also is better with just 1

roads does well with a handful of 2s, kinda makes sense when you think about it


so it seems like types of constraints need types of rollbacks

# Introduction
Inspired by [this](https://ijdykeman.github.io/ml/2017/10/12/wang-tile-procedural-generation.html)

This takes a tileset and generates an image, with the constraint that adjacent sides match.


Control signals in the tileset: red pixel at 3,3 means comment out. Blue pixel at 0,3 means include all rotations

TODO: add a "only if only solution" flag


This problem is actually computationally cooked (NP complete, maybe undecidable?) so depending on the tileset it might give you a lot of black squares which means it failed to find a tile satisying the constraints. Its actually really interesting once you think about the types of constraints you can specify. They can be long range like with roads. Check out the examples for more.


The quality of the result is completely dependent on the solver. The current solver maintains a priority queue of undecided tiles ordered by constrainedness. It fills probabilistically and rolls back when it hits a snag. The roll back gets bigger the more times a tile has been rolled back. This has a limited degree of effectiveness, it works OK for some tilesets as you can see.

## More Solver Ideas
* Smarter constraint calculation
  * include projected constraints from neighbouring tiles by number of ways to get to you
* Smarter rollbacks
  * only the tiles doing the constraining - I think currently its like 1 step forward 2 steps back
* hill climb - fill randomly and keep twiddling as long as it decreases constraint violations
* Metaheuristic ones - when hill climb stops cutting the mustard, you have to be able to go downhill a bit
  * Genetic algorithm
  * Tabu search 
  * Simulated annealing

## Usability Ideas
* Visualize calculation for greater insight + prettyness
* Output images with their tileset, maybe scaled up and with a grid

## Other Ideas / Remarks
* this is kinda like sudoku, it would make a good puzzle game
* wang tiles in Carcassonne
* the solver is really easy if the tileset is 'complete' - always a tile matching A to B
* for game content it probably doesn't need to use very sophisticated constraints. this 3x3 system is super emergent and interesting but we can get away with pretty simple constraints
* this is so small, where every pixel matters, make a say 6x6 tile version and use it for game content
* can add constraints for edges easily enough, we are kind of sufficiently constrained now I would say, although cool islands would be easy
* metaheuristics could have other constraints like maximize amount of this tile satisfying constraints (e.g. buildings in city) at this point its a basebuilding game
* changing rollback numbers really seems to affect it + it scales with size, sometimes it really sucks especially big.


=======

nothing seems to work better with big rollbacks
pluroads once worked really well?



rapid iteration would be good