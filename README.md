Tile has 4 sides each with a unique fingerprint
    fingerprint: RxPrime + GxPrime + BxPrime
    gather all matching, select probabilistically and reject all which dont satisfy surrounding sides
    matching has to be complimentary rather than identical? Or you could eliminate that by how you go about it

Tilespec duplicated for rotations, flips

Tile has a probability as well

tile itself probably a U32 of which tilespec it refers to




TODO make it actually solve when greediness doesnt get you there
rollback?

https://ijdykeman.github.io/ml/2017/10/12/wang-tile-procedural-generation.html

rollback: have a generation parameter which you increment around a guy whos stuck
maybe if it gets absurdly high
sort by level of constrainedness, solve most constrained first

put in rotations and flips

===============
Considerations:
* I wonder if the order you do it in matters, should it be random? or from a certain spot. Frontier list
* optimization later: you could stale tiles that reject too hard and have a chance to overwrite them, like annealing ish

===============
most constrained first
when a constraint gets updated, how constrained it is = how many possibilities it has remaining, do ones with less first
see how far that gets you
could be made more sophisticated by looking at possibilities for /those/ maybe but whatever

=======
prob need to flip dirs
also implement a pq with decrease key
stdlib so fucken shit!

===============
It would be possible to...
* constrain for edge of map, pick a colour

just specify colour map edge is on tileset i reckon
any other metadata we care about while im at it?

improve algorithm: can do N iterations of prediction, rule something out if it constrains neighbours down to 0


be good to visualize, i wonder if the deterministic placing cooks it
i wonder if rolling back further is any good
there do appear to be undetected defects of placement but I am yet to reproduce

seems as though more rollback doesnt help

I wonder if GA can do it
of course can just always do complete sets or whatever but this lets you constrain the design in interesting ways

coming back later ever help?
idk

roll back bigger chunks? maybe

beach one - pure symmetric vs not ay

have a mode that also shows the tilesets on the output image

writing this up will be tight as
maybe an upscale and draw grid will be g as well

what about imposing least constraints... hmm who knows

oh this is straight up a puzzle game but its like computer please do it for me

if a rollback doesnt expose more possibilities roll back more

rail infrastructure etc

its a puzzle game dude

symmetrical beach islands
disjoint beach islands - water and lava
then add the rest to make it symmetrical with 3

make it so union of 2 requires third, rock paper scissors isomorphism to ca?


maybe my backjumping could find one that is over constraining and then change it, but then also dont change it again go change other shit

test suite: add dontplaceafter2 etc
why does rollback 3 fuck it? be cool to print out rollback statistics