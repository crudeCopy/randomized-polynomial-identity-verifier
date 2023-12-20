randomized-polynomial-identity-verifier
---------------------------------------
Here we have an algorithm which will check
the equality of a product of monomials with
a canonical-form polynomial, i.e.
```
(x + 1)(x - 2)(x + 3) == x^3 + 2x^2 - 5x - 6
```
The method the algorithm uses to do this is
based on random sampling. Where _d_ represents
the degree of the canonical polynomial (or
the number of monomials), a random integer is
selected between 1 and _r*d_, with some constant
_r_. The selected integer is then plugged into
both sides of the system and their results are
then checked for equality.

There is a chance of this algorithm giving a
false positive, due to possibly choosing one
of the _d_ roots of the polynomial at random.
As the book suggests, and assuming uniform
random selection, the probability of selecting
a root is _d_/_r*d_, or 1/_r_. 

As such, one idea for making false positive
results less probable is to increase the value
of _r_.

Another idea is to perform _k_ independent
random selections, only giving a positive
result if all selections demonstrate equality.
This method has (1/_r_)^_k_ of being wrong,
so it is exponentially small for large _k_.

Finally, to completely remove the chance of
being incorrect, the algorithm may choose
_d_+1 distinct numbers to use. Since there
are only _d_ possible roots, at least one
of these selections must not be a root.

Idea and proofs due to Mitzenmacher & Upfal's
book Probability and Computing: Randomized
Algorithms and Probabilistic Analysis.

