Updates all elements in the given `range` using the specified [binary operation](crate::traits::Monoid::combine()).

Does nothing if the `range` is empty.

[Unbounded bounds](std::ops::Bound) are clamped to the tree's range.

# Panics

Panics if the `range` is explicitly out of bounds.
