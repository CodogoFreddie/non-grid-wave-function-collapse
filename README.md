## Notes
Both sampling and generating requires a piossion disk distrubuted point set
In each case the points have some associated data:
 
+ The rule sampled from the point
   + Read once
   + Immutable
+ The state at that point
   + Read many
   + Mutable

Either way, we could just create a struct to generate the points, then use them however we want to.
Probably the best for now
