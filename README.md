the constraints are pretty small, so we can just check everything.

1.  **get all feature combos**: it uses `itertools` to generate every possible combination of features we're allowed to pick.
2.  **find the best tree for each combo**: for each set of features, it recursively builds an optimal decision tree. it does this by always picking the split (a feature and a value) that results in the biggest drop in entropy.
3.  **calculate final entropy**: once a tree can't be split anymore to reduce entropy, it calculates the total weighted entropy of all the leaf nodes.
4.  **pick the winner**: it keeps track of the feature combo that produced the tree with the absolute lowest entropy. if there's a tie, `itertools` naturally gives us the one with the smaller feature indices first, so that's handled.
