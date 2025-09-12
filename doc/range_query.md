Answers a query over the given `range`.

Returns [the identity element](crate::traits::Monoid::identity()) if the range is empty.

[Unbounded bounds](std::ops::Bound) are clamped to the tree's range.

# Panics

Panics if the range is explicitly out of bounds.
